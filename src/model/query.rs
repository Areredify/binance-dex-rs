use crate::model;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Query: Serialize {
    type Response: DeserializeOwned;

    fn get_endpoint(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Time;

impl Query for Time {
    type Response = model::Times;

    fn get_endpoint(&self) -> String {
        "/api/v1/time".into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeInfo;

impl Query for NodeInfo {
    type Response = model::ResultStatus;

    fn get_endpoint(&self) -> String {
        "/api/v1/node-info".into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Validators;

impl Query for Validators {
    type Response = model::Validators;

    fn get_endpoint(&self) -> String {
        "/api/v1/validators".into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Peers;

impl Query for Peers {
    type Response = Vec<model::Peer>;

    fn get_endpoint(&self) -> String {
        "/api/v1/peers".into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    #[serde(skip)]
    pub address: String,
}

impl Query for Account {
    type Response = model::Account;

    fn get_endpoint(&self) -> String {
        format!("/api/v1/account/{}", self.address)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountSequence {
    #[serde(skip)]
    pub address: String,
}

impl Query for AccountSequence {
    type Response = model::AccountSequence;

    fn get_endpoint(&self) -> String {
        format!("/api/v1/account/{}/sequence", self.address)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tokens {
    pub limit: Option<u32>,  // defaults to 100
    pub offset: Option<u32>, // defaults to 0
}

impl Query for Tokens {
    type Response = Vec<model::Token>;

    fn get_endpoint(&self) -> String {
        "/api/v1/tokens".into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Markets {
    pub limit: Option<u32>,  // defaults to 100
    pub offset: Option<u32>, // defaults to 0
}

impl Query for Markets {
    type Response = Vec<model::Market>;

    fn get_endpoint(&self) -> String {
        "/api/v1/markets".into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fees {}

impl Query for Fees {
    type Response = Vec<model::Fee>;

    fn get_endpoint(&self) -> String {
        "/api/v1/fees".into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketDepth {
    pub symbol: String,     // Market pair symbol, e.g. NNB-0AD_BNB
    pub limit: Option<u32>, // The limit of results. Allowed limits: [5, 10, 20, 50, 100, 500, 1000]
}

impl Query for MarketDepth {
    type Response = model::MarketDepth;

    fn get_endpoint(&self) -> String {
        "/api/v1/depth".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Candlestick {
    pub symbol: String,
    pub interval: model::Intervals,
    pub limit: Option<u32>, // default 300; max 1000.
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>, // start time in milliseconds
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>, // end time in milliseconds
}

impl Query for Candlestick {
    type Response = Vec<model::Candlestick>;

    fn get_endpoint(&self) -> String {
        "/api/v1/klines".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum OrderStatus {
    Ack,
    IocExpire,
    IocNoFill,
    FullyFill,
    Canceled,
    Expired,
    FailedBlocking,
    FailedMatching,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ClosedOrders {
    pub address: String,
    #[serde(rename = "start")]
    pub start_time: Option<u64>,
    #[serde(rename = "end")]
    pub end_time: Option<u64>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub side: Option<model::OrderSide>,
    pub status: Option<OrderStatus>,
    pub symbol: Option<String>,
    pub total: Option<i32>,
}

impl Query for ClosedOrders {
    type Response = model::OrderList;

    fn get_endpoint(&self) -> String {
        "/api/v1/orders/closed".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct OpenOrders {
    pub address: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub symbol: Option<String>,
    pub total: Option<i32>,
}

impl Query for OpenOrders {
    type Response = model::OrderList;

    fn get_endpoint(&self) -> String {
        "/api/v1/orders/open".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Order {
    #[serde(skip)]
    pub id: String,
}

impl Query for Order {
    type Response = model::Order;

    fn get_endpoint(&self) -> String {
        format!("/api/v1/orders/{}", self.id)
    }
}

/// *Description*: Gets 24 hour price change statistics for a market pair symbol. Updated every second.
/// *Rate Limit*: 5 requests per IP per second.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MarketTicker24hr {
    pub symbol: Option<String>,
}

impl Query for MarketTicker24hr {
    type Response = Vec<model::Ticker>;

    fn get_endpoint(&self) -> String {
        "/api/v1/ticker/24hr".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Trades {
    pub symbol: Option<String>,
    pub address: Option<String>,
    pub buyer_order_id: Option<String>,
    #[serde(rename = "end")]
    pub end_time: Option<u64>,
    pub start_time: Option<u64>,
    #[serde(rename = "height")]
    pub block_height: Option<model::BlockHeight>,
    pub limit: Option<u32>,  // default 500; max 1000
    pub offset: Option<u32>, // default 0;
    pub quote_asset: Option<String>,
    pub seller_order_id: Option<String>,
    pub side: Option<model::OrderSide>,
    pub total: Option<i32>,
}

impl Query for Trades {
    type Response = model::TradePage;

    fn get_endpoint(&self) -> String {
        "/api/v1/trades".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct BlockExchangeFee {
    pub address: String,
    #[serde(rename = "start")]
    pub start_time: Option<u64>,
    #[serde(rename = "end")]
    pub end_time: Option<u64>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub total: Option<i32>,
}

impl Query for BlockExchangeFee {
    type Response = model::BlockExchangeFeePage;

    fn get_endpoint(&self) -> String {
        "/api/v1/block-exchange-fee".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AtomicSwaps {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub from_address: Option<String>, // | at least one of from_adress and to_adress
    pub to_address: Option<String>,   // | should be provided
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Query for AtomicSwaps {
    type Response = model::AtomicSwapPage;

    fn get_endpoint(&self) -> String {
        "/api/v1/atomic-swaps".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AtomicSwap {
    #[serde(skip)]
    pub id: String,
}

impl Query for AtomicSwap {
    type Response = model::AtomicSwap;

    fn get_endpoint(&self) -> String {
        format!("/api/v1/atomic-swaps/{}", self.id)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Timelocks {
    #[serde(skip)]
    address: String,
    id: i64,
}

impl Query for Timelocks {
    type Response = model::TimeLocks;

    fn get_endpoint(&self) -> String {
        format!("/api/v1/timelocks/{}", self.id)
    }
}
