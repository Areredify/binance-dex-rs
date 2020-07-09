use serde::{Deserialize, Serialize};

use crate::model::{Fixed8, InlineFee, OrderSide, OrderStatus, OrderType, PriceQty};


#[derive(Serialize, Clone, Debug)]
pub enum Command {

}

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "topic", rename_all = "camelCase")]
pub enum Topic {
    Orders {
        address: String,
    },
    Accounts {
        address: String,
    },
    Transfers {
        address: String,
    },
    Trades {
        symbols: Vec<String>,
    },
    MarketDiff {
        symbols: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message {
    Ping,
    Pong,
    Payload(Payload),
    Unknown
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Payload {

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Orders {
    #[serde(rename = "e")]
    pub event: String, 
    #[serde(rename = "E")]
    pub event_timestamp: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: OrderSide,
    #[serde(rename = "o")]
    pub type_: OrderType,
    #[serde(rename = "q")]
    pub qty: Fixed8,
    #[serde(rename = "p")]
    pub price: Fixed8,
    #[serde(rename = "x")]
    pub current_execution_type: String,
    #[serde(rename = "X")]
    pub current_status: OrderStatus,
    #[serde(rename = "i")]
    pub order_id: String,
    #[serde(rename = "l")]
    pub last_executed_qty: Fixed8,
    #[serde(rename = "L")]
    pub last_executed_price: Fixed8,
    #[serde(rename = "z")]
    pub commulative_filled_qty: Fixed8,
    #[serde(rename = "n")]
    pub comission_amount: InlineFee,
    #[serde(rename = "T")]
    pub transaction_time: i64,
    #[serde(rename = "t")]
    pub trade_id: String,
    #[serde(rename = "O")]
    pub creation_time: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssetBalance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "f")]
    pub free: Fixed8,
    #[serde(rename = "l")]
    pub locked: Fixed8,
    #[serde(rename = "r")]
    pub frozen: Fixed8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Accounts {
    #[serde(rename = "e")]
    pub event: String, 
    #[serde(rename = "E")]
    pub event_timestamp: i64,
    #[serde(rename = "B")]
    pub balances: Vec<AssetBalance>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssetAmount {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "A")]
    pub amount: Fixed8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transfer {
    #[serde(rename = "o")]
    pub to_addr: String,
    #[serde(rename = "c")]
    pub coins: Vec<AssetAmount>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transfers {
    #[serde(rename = "e")]
    pub event: String, 
    #[serde(rename = "E")]
    pub event_timestamp: i64,
    #[serde(rename = "H")]
    pub transaction_hash: String,
    #[serde(rename = "M")]
    pub memo: String,
    #[serde(rename = "f")]
    pub from_addr: String,
    #[serde(rename = "t")]
    pub transfers: Vec<Transfer>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trade {
    #[serde(rename = "e")]
    pub event: String, 
    #[serde(rename = "E")]
    pub event_timestamp: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub trade_id: String,
    #[serde(rename = "p")]
    pub price: Fixed8,
    #[serde(rename = "q")]
    pub qty: Fixed8,
    #[serde(rename = "b")]
    pub buyer_order_id: String,
    #[serde(rename = "a")]
    pub seller_order_id: String,
    #[serde(rename = "T")]
    pub trade_time: i64,
    #[serde(rename = "sa")]
    pub seller_address: String,
    #[serde(rename = "ba")]
    pub buyer_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketDiff {
    #[serde(rename = "e")]
    pub event: String, 
    #[serde(rename = "E")]
    pub event_timestamp: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b")]
    pub bids: Vec<PriceQty>,
    #[serde(rename = "a")]
    pub asks: Vec<PriceQty>,
}

// 24 hr ticker
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ticker {
    #[serde(rename = "e")]
    pub event: String, 
    #[serde(rename = "E")]
    pub event_timestamp: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price_change: Fixed8,
    #[serde(rename = "P")]
    pub price_change_percent: Fixed8,
    #[serde(rename = "w")]
    pub weighted_avg_price: Fixed8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MiniTicker {

}
