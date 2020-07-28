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
pub struct Account<'a> {
    #[serde(skip)]
    pub address: &'a str,
}

impl Query for Account<'_> {
    type Response = model::Account;

    fn get_endpoint(&self) -> String {
        format!("/api/v1/account/{}", self.address)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountSequence<'a> {
    #[serde(skip)]
    pub address: &'a str,
}

impl Query for AccountSequence<'_> {
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
pub struct MarketDepth<'a> {
    pub symbol: &'a str,    // Market pair symbol, e.g. NNB-0AD_BNB
    pub limit: Option<u32>, // The limit of results. Allowed limits: [5, 10, 20, 50, 100, 500, 1000]
}

impl Query for MarketDepth<'_> {
    type Response = model::MarketDepth;

    fn get_endpoint(&self) -> String {
        "/api/v1/depth".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Candlestick<'a> {
    pub symbol: &'a str,
    pub interval: model::Intervals,
    pub limit: Option<u32>, // default 300; max 1000.
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>, // start time in milliseconds
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>, // end time in milliseconds
}

impl Query for Candlestick<'_> {
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
pub struct ClosedOrders<'a> {
    pub address: &'a str,
    #[serde(rename = "start")]
    pub start_time: Option<u64>,
    #[serde(rename = "end")]
    pub end_time: Option<u64>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub side: Option<model::OrderSide>,
    pub status: Option<OrderStatus>,
    pub symbol: Option<&'a str>,
    pub total: Option<i32>,
}

impl Query for ClosedOrders<'_> {
    type Response = model::OrderList;

    fn get_endpoint(&self) -> String {
        "/api/v1/orders/closed".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct OpenOrders<'a> {
    pub address: &'a str,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub symbol: Option<&'a str>,
    pub total: Option<i32>,
}

impl Query for OpenOrders<'_> {
    type Response = model::OrderList;

    fn get_endpoint(&self) -> String {
        "/api/v1/orders/open".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Order<'a> {
    #[serde(skip)]
    pub id: &'a str,
}

impl Query for Order<'_> {
    type Response = model::Order;

    fn get_endpoint(&self) -> String {
        format!("/api/v1/orders/{}", self.id)
    }
}

/// *Description*: Gets 24 hour price change statistics for a market pair symbol. Updated every second.
/// *Rate Limit*: 5 requests per IP per second.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MarketTicker24hr<'a> {
    pub symbol: Option<&'a str>,
}

impl Query for MarketTicker24hr<'_> {
    type Response = Vec<model::Ticker>;

    fn get_endpoint(&self) -> String {
        "/api/v1/ticker/24hr".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Trades<'a> {
    pub symbol: Option<&'a str>,
    pub address: Option<&'a str>,
    pub buyer_order_id: Option<&'a str>,
    #[serde(rename = "end")]
    pub end_time: Option<u64>,
    pub start_time: Option<u64>,
    #[serde(rename = "height")]
    pub block_height: Option<model::BlockHeight>,
    pub limit: Option<u32>,  // default 500; max 1000
    pub offset: Option<u32>, // default 0;
    pub quote_asset: Option<&'a str>,
    pub seller_order_id: Option<&'a str>,
    pub side: Option<model::OrderSide>,
    pub total: Option<i32>,
}

impl Query for Trades<'_> {
    type Response = model::TradePage;

    fn get_endpoint(&self) -> String {
        "/api/v1/trades".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct BlockExchangeFee<'a> {
    pub address: &'a str,
    #[serde(rename = "start")]
    pub start_time: Option<u64>,
    #[serde(rename = "end")]
    pub end_time: Option<u64>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub total: Option<i32>,
}

impl Query for BlockExchangeFee<'_> {
    type Response = model::BlockExchangeFeePage;

    fn get_endpoint(&self) -> String {
        "/api/v1/block-exchange-fee".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AtomicSwaps<'a> {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub from_address: Option<&'a str>, // | at least one of from_adress and to_adress
    pub to_address: Option<&'a str>,   // | should be provided
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Query for AtomicSwaps<'_> {
    type Response = model::AtomicSwapPage;

    fn get_endpoint(&self) -> String {
        "/api/v1/atomic-swaps".into()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AtomicSwap<'a> {
    #[serde(skip)]
    pub id: &'a str,
}

impl Query for AtomicSwap<'_> {
    type Response = model::AtomicSwap;

    fn get_endpoint(&self) -> String {
        format!("/api/v1/atomic-swaps/{}", self.id)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Timelocks<'a> {
    #[serde(skip)]
    address: &'a str,
    id: i64,
}

impl Query for Timelocks<'_> {
    type Response = model::TimeLocks;

    fn get_endpoint(&self) -> String {
        format!("/api/v1/timelocks/{}", self.id)
    }
}
