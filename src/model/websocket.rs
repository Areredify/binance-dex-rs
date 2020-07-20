use serde::{Deserialize, Serialize};

use crate::model::{
    self, Fixed8, InlineFee, Intervals, OrderSide, OrderStatus, OrderType, PriceQty,
};
use serde_json::json;

pub struct SubscriptionToken(pub usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MarketDepthLevelsExt {
    T100,
    T500,
    T1000,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
    MarketDepth {
        amount: Option<MarketDepthLevelsExt>, // extention number of bids an asks. None means top 20
        symbols: Vec<String>,
    },
    Candlestick {
        interval: Intervals,
        symbols: Vec<String>,
    },
    Ticker {
        symbols: Vec<String>,
    },
    AllTickers,
    MiniTicker {
        symbols: Vec<String>,
    },
    AllMiniTickers,
    BlockHeight,
}

impl Topic {
    pub fn to_subscribe_message(&self) -> String {
        let topic = match self {
            Self::Orders { .. } => "orders".into(),
            Self::Accounts { .. } => "accounts".into(),
            Self::Transfers { .. } => "transfers".into(),
            Self::Trades { .. } => "trades".into(),
            Self::MarketDiff { .. } => "marketDiff".into(),
            Self::MarketDepth { amount, .. } => match amount {
                Some(amount) => format!(
                    "marketDepth{}",
                    match amount {
                        MarketDepthLevelsExt::T100 => 100,
                        MarketDepthLevelsExt::T500 => 500,
                        MarketDepthLevelsExt::T1000 => 1000,
                    }
                ),
                None => "marketDepth".into(),
            },
            Self::Candlestick { interval, .. } => {
                let value = serde_json::to_value(interval).unwrap();
                format!("kline_{}", value.as_str().unwrap())
            }
            Self::Ticker { .. } => "ticker".into(),
            Self::AllTickers => "allTickers".into(),
            Self::MiniTicker { .. } => "miniTicker".into(),
            Self::AllMiniTickers => "allMiniTickers".into(),
            Self::BlockHeight => "blockheight".into(),
        };

        let value = match self {
            Self::Orders { address } | Self::Accounts { address } | Self::Transfers { address } => {
                json!({"method": "subscribe", "topic": topic, "address": address})
            }
            Self::Trades { symbols }
            | Self::MarketDiff { symbols, .. }
            | Self::MarketDepth { symbols, .. }
            | Self::Candlestick { symbols, .. }
            | Self::Ticker { symbols, .. }
            | Self::MiniTicker { symbols } => {
                json!({"method": "subscribe", "topic": topic, "symbols": symbols})
            }
            Self::AllTickers | Self::AllMiniTickers | Self::BlockHeight => {
                json!({"method": "subscribe", "topic": topic, "symbols": ["$all"]})
            }
        };
        value.to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message {
    Ping,
    Pong,
    Binary(Vec<u8>),
    Orders(Vec<Order>),
    Accounts(Accounts),
    Transfers(Transfers),
    Trades(Vec<Trade>),
    MarketDiff(MarketDiff),
    MarketDepth(MarketDepth),
    Candlestick(CandlestickEvent),
    Ticker(Ticker),
    AllTickers(Vec<Ticker>),
    MiniTicker(MiniTicker),
    AllMiniTickers(Vec<MiniTicker>),
    Blockheight(Blockheight),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketDepth {
    pub last_update_id: i64,
    pub symbol: String,
    pub bids: Vec<PriceQty>,
    pub asks: Vec<PriceQty>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Candlestick {
    #[serde(rename = "t")]
    pub start_time: i64,
    #[serde(rename = "T")]
    pub end_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: Intervals,
    #[serde(rename = "f")]
    pub first_trade_id: String,
    #[serde(rename = "L")]
    pub last_trade_id: String,
    #[serde(rename = "o")]
    pub open_price: Fixed8,
    #[serde(rename = "c")]
    pub close_price: Fixed8,
    #[serde(rename = "h")]
    pub high_price: Fixed8,
    #[serde(rename = "l")]
    pub low_price: Fixed8,
    #[serde(rename = "v")]
    pub base_asset_volume: Fixed8,
    #[serde(rename = "q")]
    pub quote_asset_volume: Fixed8,
    #[serde(rename = "n")]
    pub number_of_trades: i64,
    #[serde(rename = "x")]
    pub is_closed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CandlestickEvent {
    #[serde(rename = "e")]
    pub event: String,
    #[serde(rename = "E")]
    pub event_timestamp: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub candlestick: Candlestick,
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
    #[serde(rename = "x")]
    pub previous_day_close_price: Fixed8,
    #[serde(rename = "c")]
    pub current_day_close_price: Fixed8,
    #[serde(rename = "Q")]
    pub close_trade_quantity: Fixed8,
    #[serde(rename = "b")]
    pub best_bid_price: Fixed8,
    #[serde(rename = "B")]
    pub best_bid_quantity: Fixed8,
    #[serde(rename = "a")]
    pub best_ask_price: Fixed8,
    #[serde(rename = "A")]
    pub best_ask_quantity: Fixed8,
    #[serde(rename = "o")]
    pub open_price: Fixed8,
    #[serde(rename = "h")]
    pub high_price: Fixed8,
    #[serde(rename = "l")]
    pub low_price: Fixed8,
    #[serde(rename = "v")]
    pub base_asset_volume: Fixed8,
    #[serde(rename = "q")]
    pub quote_asset_volume: Fixed8,
    #[serde(rename = "O")]
    pub statistics_open_time: i64,
    #[serde(rename = "C")]
    pub statistics_close_time: i64,
    #[serde(rename = "F")]
    pub first_trade_id: String,
    #[serde(rename = "L")]
    pub last_trade_id: String,
    #[serde(rename = "n")]
    pub total_trades: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MiniTicker {
    #[serde(rename = "e")]
    pub event: String,
    #[serde(rename = "E")]
    pub event_timestamp: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "o")]
    pub open_price: Fixed8,
    #[serde(rename = "c")]
    pub close_price: Fixed8,
    #[serde(rename = "h")]
    pub high_price: Fixed8,
    #[serde(rename = "l")]
    pub low_price: Fixed8,
    #[serde(rename = "v")]
    pub base_asset_volume: Fixed8,
    #[serde(rename = "q")]
    pub quote_asset_volume: Fixed8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockheight {
    #[serde(rename = "h")]
    height: model::BlockHeight,
}
