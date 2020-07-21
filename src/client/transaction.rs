use std::collections::{BTreeMap, HashMap};

use failure::Fallible;

use crate::{
    client::BinanceDexClient,
    key_manager::str_to_address,
    model::{
        transaction::{
            CancelOrder, Coin as TxCoin, CreateOrder, CreateOrderResponse, Msg,
            Transfer as TxTransfer, TransferIO, TxCommitResult,
        },
        OrderDuration, OrderSide, OrderType,
    },
    util::combine_symbols,
    Fixed8,
};

pub struct TransactionOptions {
    pub memo: Option<String>,
    pub acc_info: Option<(i64, i64)>, // account_number, sequence pair
    pub sync: bool,
}

#[derive(Clone, Debug)]
pub struct Coin {
    pub denom: String,
    pub quantity: Fixed8,
}

#[derive(Clone, Debug)]
pub struct Transfer {
    outputs: HashMap<String, BTreeMap<String, Fixed8>>,
}

impl Transfer {
    fn insert_coin(m: &mut BTreeMap<String, Fixed8>, denom: String, quantity: Fixed8) {
        *m.entry(denom).or_default() += quantity;
    }

    pub fn add_coins(&mut self, reciever_address: String, coins: impl Iterator<Item = Coin>) {
        let self_coins = self.outputs.entry(reciever_address).or_default();
        for coin in coins {
            Self::insert_coin(self_coins, coin.denom, coin.quantity);
        }
    }

    pub(crate) fn merge(&self) -> BTreeMap<String, Fixed8> {
        let mut m = BTreeMap::new();
        for value in self.outputs.values() {
            for (denom, quantity) in value {
                Self::insert_coin(&mut m, denom.clone(), *quantity);
            }
        }

        m
    }
}

impl BinanceDexClient {
    /// acc_info is a (account_number, sequence) pair.
    /// see docs.binance.org/guides/concepts/accounts.html for info.
    /// If not specified, most recent info will be fetched from the REST API.
    pub async fn create_order(
        &self,
        base_symbol: &str,
        quote_asset_symbol: &str,
        side: OrderSide,
        timeinforce: OrderDuration,
        price: Fixed8,
        quantity: Fixed8,
        mut options: TransactionOptions,
    ) -> Fallible<(TxCommitResult, Option<CreateOrderResponse>)> {
        let km = self.get_km()?;

        let symbol = combine_symbols(base_symbol, quote_asset_symbol);

        let (account_number, sequence) = match options.acc_info {
            Some(a) => a,
            None => self.fetch_acc_info().await?,
        };

        let id = format!(
            "{}-{}",
            hex::encode_upper(&km.account_address),
            sequence + 1
        );

        let msg = Msg::CreateOrder(CreateOrder {
            symbol,
            id,
            sender: km.account_address.clone(),
            ordertype: OrderType::Limit as i64,
            side: side as i64,
            timeinforce: timeinforce as i64,
            price: price.0,
            quantity: quantity.0,
        });

        options.acc_info = Some((account_number, sequence));

        let result = self.broadcast(msg, options).await?;

        let resp = if result.data != "" && result.ok {
            Some(serde_json::from_str(&result.data)?)
        } else {
            None
        };

        Ok((result, resp))
    }

    pub async fn cancel_order(
        &self,
        symbol: String,
        id: String,
        options: TransactionOptions,
    ) -> Fallible<TxCommitResult> {
        let km = self.get_km()?;

        let msg = Msg::CancelOrder(CancelOrder {
            sender: km.account_address.clone(),
            symbol,
            refid: id,
        });

        self.broadcast(msg, options).await
    }

    pub async fn transfer(
        &self,
        transfer: Transfer,
        options: TransactionOptions,
    ) -> Fallible<TxCommitResult> {
        let km = self.get_km()?;

        let inputs: Vec<_> = transfer
            .merge()
            .into_iter()
            .map(|(denom, qty)| TxCoin {
                denom,
                amount: qty.0,
            })
            .collect();
        let inputs = vec![TransferIO {
            address: km.account_address.clone(),
            coins: inputs,
        }];

        let outputs: Result<Vec<TransferIO>, _> = transfer
            .outputs
            .into_iter()
            .map(|(key, value)| -> Fallible<TransferIO> {
                Ok(TransferIO {
                    address: str_to_address(&key)?,
                    coins: value
                        .into_iter()
                        .map(|(denom, qty)| TxCoin {
                            denom,
                            amount: qty.0,
                        })
                        .collect(),
                })
            })
            .collect();

        let outputs = outputs?;

        let msg = Msg::Transfer(TxTransfer { inputs, outputs });

        self.broadcast(msg, options).await
    }
}
