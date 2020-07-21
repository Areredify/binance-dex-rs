use failure::Fallible;
use log::error;
use prost_amino::Message;
use reqwest::{Client, Method, Response};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string, to_value};
use url::Url;

use crate::{
    api_url::{CHAIN_ID, HTTP_URL},
    key_manager::KeyManager,
    model::{
        query::{self, Query},
        transaction::{Msg, StdSignMsg, StdSignature, StdTx, TxCommitResult},
        Error as BinanceError,
    },
};

mod transaction;
pub mod websocket;

pub use transaction::TransactionOptions;

#[derive(Default)]
pub struct BinanceDexClient {
    client: Client,
    key_manager: Option<KeyManager>,
}

impl BinanceDexClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_credentials(key_manager: KeyManager) -> Self {
        Self {
            key_manager: Some(key_manager),
            ..Default::default()
        }
    }

    pub async fn query<Q: Query>(&self, request: Q) -> Fallible<Q::Response> {
        let url = format!("{}{}", *HTTP_URL, request.get_endpoint());
        let url = Url::parse_with_params(&url, request.to_url_query())?;

        let req = self
            .client
            .request(Method::GET, url.as_str())
            .header("user-agent", "binance-dex-rs");

        self.handle_response(req.send().await?).await
    }

    async fn handle_response<T: DeserializeOwned>(&self, resp: Response) -> Fallible<T> {
        if resp.status().is_success() {
            let resp = resp.text().await?;
            match from_str::<T>(&resp) {
                Ok(resp) => Ok(resp),
                Err(e) => {
                    error!("Cannot deserialize '{}'", resp);
                    Err(e.into())
                }
            }
        } else {
            let resp_e = resp.error_for_status_ref().unwrap_err();
            if let Ok(e) = resp.json::<BinanceError>().await {
                Err(e.into())
            } else {
                Err(resp_e.into())
            }
        }
    }

    pub async fn fetch_acc_info(&self) -> Fallible<(i64, i64)> {
        let account = self
            .query(query::Account {
                address: self
                    .key_manager
                    .as_ref()
                    .unwrap()
                    .account_address_str
                    .clone(),
            })
            .await?;

        Ok((account.account_number, account.sequence))
    }

    async fn post_tx(&self, body: String, sync: bool) -> Fallible<TxCommitResult> {
        let url = format!("{}/broadcast", *HTTP_URL);
        let mut url = Url::parse(&url)?;

        if sync {
            url.set_query(Some("sync=true"));
        }

        let req = self
            .client
            .request(Method::POST, url.as_str())
            .header("user-agent", "binance-dex-rs")
            .header("content-type", "text/plain")
            .body(body.into_bytes());

        let resp: Vec<TxCommitResult> = self.handle_response(req.send().await?).await?;

        resp.into_iter()
            .nth(0)
            .ok_or_else(|| failure::format_err!("server sent an empty response"))
    }

    fn get_km(&self) -> Fallible<&KeyManager> {
        match self.key_manager.as_ref() {
            Some(km) => Ok(km),
            None => Err(failure::format_err!("no key manager present")),
        }
    }

    /// acc_info is a (account_number, sequence) pair.
    /// See docs.binance.org/guides/concepts/accounts.html for info.
    /// If not specified, most recent info will be fetched from the REST API.
    async fn broadcast(&self, msg: Msg, options: TransactionOptions) -> Fallible<TxCommitResult> {
        let km = self.get_km()?;

        let (account_number, sequence) = match options.acc_info {
            Some(a) => a,
            None => self.fetch_acc_info().await?,
        };
        let memo = options.memo.unwrap_or(String::new());

        let sign = StdSignMsg {
            chain_id: (*CHAIN_ID).into(),
            account_number,
            sequence,
            memo: memo.as_str(),
            msgs: &[&msg],
            source: 0,
            data: None,
        };

        let sign = km.sign(&sign)?;
        let sign = StdSignature {
            pub_key: km.public_key.clone(),
            signature: sign,
            account_number,
            sequence,
        };

        let tx = StdTx {
            msgs: vec![msg],
            signatures: vec![sign],
            memo,
            source: 0,
            data: vec![],
        };

        let mut body = vec![];
        tx.encode_length_delimited(&mut body)?;
        self.post_tx(hex::encode(&body), options.sync).await
    }
}

trait ToUrlQuery: Serialize {
    fn to_url_query(&self) -> Vec<(String, String)> {
        let v = to_value(self).unwrap();
        let v = if let Some(v) = v.as_object() {
            v
        } else {
            return vec![];
        };

        let mut vec = vec![];

        for (key, value) in v.into_iter() {
            if value.is_null() {
                continue;
            } else if value.is_string() {
                vec.push((key.clone(), value.as_str().unwrap().to_string()))
            } else {
                vec.push((key.clone(), to_string(value).unwrap()))
            }
        }
        vec
    }
}

impl<S: Serialize> ToUrlQuery for S {}

#[cfg(test)]
mod test {
    use crate::{
        key_manager::KeyManager,
        model::{transaction::*, *},
    };
    use failure::Fallible;
    use prost_amino::Message;

    fn encode_msg(
        km: &KeyManager,
        msg: Msg,
        account_number: i64,
        sequence: i64,
    ) -> Fallible<String> {
        let memo = String::new();

        let sign = StdSignMsg {
            chain_id: "bnbchain-1000",
            account_number,
            sequence,
            memo: &memo,
            msgs: &[&msg],
            source: 0,
            data: None,
        };

        let sign = km.sign(&sign)?;
        let sign = StdSignature {
            pub_key: km.public_key.clone(),
            signature: sign,
            account_number,
            sequence,
        };

        let tx = StdTx {
            msgs: vec![msg],
            signatures: vec![sign],
            memo,
            source: 0,
            data: vec![],
        };

        let mut x = vec![];
        tx.encode_length_delimited(&mut x)?;
        Ok(hex::encode(&x))
    }

    #[test]
    fn transactions() -> Fallible<()> {
        let km1 = KeyManager::from_private_key(
            "01a8d11703efbd8cd8653174216efd9b7901e081db96215b949739727b9047ba",
        )?;
        let km2 = KeyManager::from_private_key(
            "5cc80a4fee8b51afbbe71f2ae079c682f474b6f67e116b0e6c230506a6a695aa",
        )?;

        let msg = Msg::CancelOrder(CancelOrder {
            symbol: "BTC-86A_BNB".into(),
            sender: km1.account_address.clone(),
            refid: "1D0E3086E8E4E0A53C38A90D55BD58B34D57D2FA-5".into(),
        });
        assert_eq!(
            encode_msg(&km1, msg, 0, 5)?,
            "c701f0625dee0a53166e681b0a141d0e3086e8e4e0a53c38a90d55bd58b34d57d2fa120b4254432d383\
             6415f424e421a2a31443045333038364538453445304135334333384139304435354244353842333444\
             3537443246412d35126c0a26eb5ae98721027e69d96640300433654e016d218a8d7ffed751023d8efe8\
             1e55dedbd6754c9711240fe2fd18630317849bd1d4ae064f8c4fd95f6186bdb61e2b73a5fb5e93ac779\
             4d4a990ba943694659df9d3f49d5312fec020b80148677f3e95fd6d88486bba19d2005"
        );

        let msg = Msg::CreateOrder(CreateOrder {
            sender: km1.account_address.clone(),
            id: "1D0E3086E8E4E0A53C38A90D55BD58B34D57D2FA-5".into(),
            side: OrderSide::Buy as i64,
            symbol: "BTC-86A_BNB".into(),
            price: 100000000,
            quantity: 1000000000,
            ordertype: OrderType::Limit as i64,
            timeinforce: OrderDuration::GoodTillExpire as i64,
        });

        assert_eq!(
            encode_msg(&km1, msg, 0, 4)?,
            "d801f0625dee0a64ce6dc0430a141d0e3086e8e4e0a53c38a90d55bd58b34d57d2fa122a31443045333\
             0383645384534453041353343333841393044353542443538423334443537443246412d351a0b425443\
             2d3836415f424e42200228013080c2d72f388094ebdc034001126c0a26eb5ae98721027e69d96640300\
             433654e016d218a8d7ffed751023d8efe81e55dedbd6754c97112409fe317e036f2bdc8c87a0138dc52\
             367faef80ea1d6e21a35634b17a82ed7be632c9cb03f865f6f8a6872736ccab716a157f3cb99339afa5\
             5686aa455dc134f6a2004"
        );

        let msg = Msg::Transfer(Transfer {
            inputs: vec![TransferIO {
                address: km1.account_address.clone(),
                coins: vec![Coin {
                    denom: "BNB".into(),
                    amount: 100000000000000,
                }],
            }],
            outputs: vec![TransferIO {
                address: km2.account_address.clone(),
                coins: vec![Coin {
                    denom: "BNB".into(),
                    amount: 100000000000000,
                }],
            }],
        });

        assert_eq!(
            encode_msg(&km1, msg, 0, 1)?,
            "c601f0625dee0a522a2c87fa0a250a141d0e3086e8e4e0a53c38a90d55bd58b34d57d2fa120d0a03424\
             e42108080e983b1de1612250a146b571fc0a9961a7ddf45e49a88a4d83941fcabbe120d0a03424e4210\
             8080e983b1de16126c0a26eb5ae98721027e69d96640300433654e016d218a8d7ffed751023d8efe81e\
             55dedbd6754c97112408b23eecfa8237a27676725173e58154e6c204bb291b31c3b7b507c8f04e27739\
             09ba70e01b54f4bd0bc76669f5712a5a66b9508acdf3aa5e4fde75fbe57622a12001"
        );

        let msg = Msg::Transfer(Transfer {
            inputs: vec![TransferIO {
                address: km1.account_address.clone(),
                coins: vec![
                    Coin {
                        denom: "BNB".into(),
                        amount: 100000000000000,
                    },
                    Coin {
                        denom: "BTC".into(),
                        amount: 1000000000000,
                    },
                ],
            }],
            outputs: vec![TransferIO {
                address: km2.account_address.clone(),
                coins: vec![
                    Coin {
                        denom: "BNB".into(),
                        amount: 100000000000000,
                    },
                    Coin {
                        denom: "BTC".into(),
                        amount: 1000000000000,
                    },
                ],
            }],
        });

        assert_eq!(
            encode_msg(&km1, msg, 2, 3)?,
            "e401f0625dee0a6e2a2c87fa0a330a141d0e3086e8e4e0a53c38a90d55bd58b34d57d2fa120d0a03424e\
             42108080e983b1de16120c0a034254431080a094a58d1d12330a146b571fc0a9961a7ddf45e49a88a4d8\
             3941fcabbe120d0a03424e42108080e983b1de16120c0a034254431080a094a58d1d126e0a26eb5ae987\
             21027e69d96640300433654e016d218a8d7ffed751023d8efe81e55dedbd6754c97112407516bf5ac3b4\
             bf7a9037a4ca33c5eaa250392d8c4c13489985de079626f7daec488513c5315ae25549aa612142aa9f41\
             979f7b7fed4ae93c01449dabf1b0ef5218022003"
        );

        Ok(())
    }
}
