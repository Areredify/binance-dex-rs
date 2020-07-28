// Binance DEX/Chain API model module.
// Reference: https://docs.binance.org/api-reference/dex-api/paths.html
// N.B.(!) the actual api may not match the reference, run tests before changing it.

// Note: Binance Chain uses RFC3339 for date representation, which
// matches default representation for serde, so no custom logic is required
// when de-/serializing DateTime.

use std::fmt;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub mod fixed8;
pub mod query;
pub mod transaction;
pub mod websocket;

use fixed8::Fixed8;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Binance Dex error with code {}: {}",
            self.code, self.message
        )
    }
}

impl std::error::Error for Error {}

pub type BlockHeight = i64;
pub type InlineFee = String;
pub type PriceQty = (Fixed8, Fixed8);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Times {
    pub ap_time: DateTime<Utc>,
    pub block_time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Validators {
    pub block_height: BlockHeight,
    pub validators: Vec<Validator>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Validator {
    pub address: String,
    pub pub_key: Vec<u8>,
    pub voting_power: i64,
    pub proposer_priority: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    Node,
    Qs,
    Ap,
    Ws,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Peer {
    pub id: String,
    pub original_listen_addr: String,
    pub listen_addr: String,
    pub access_addr: String,
    pub stream_addr: String,
    pub network: String,
    pub version: String,
    pub moniker: String,
    pub capabilities: Vec<Capability>,
    #[serde(rename = "accelerated")]
    pub is_accelerated: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ValidatorInfo {
    pub address: String,
    pub pub_key: Vec<u8>,
    pub voting_power: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SyncInfo {
    pub latest_block_hash: String,
    pub latest_app_hash: String,
    pub latest_block_height: BlockHeight,
    pub latest_block_time: DateTime<Utc>,
    pub catching_up: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProtocolVersion {
    pub p2p: u64,
    pub block: u64,
    pub app: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NodeInfo {
    pub protocol_version: ProtocolVersion,
    pub id: String,
    pub listen_addr: String,
    pub network: String,
    pub version: String,
    pub channels: String,
    pub moniker: String,
    pub other: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResultStatus {
    pub validator_info: ValidatorInfo,
    pub sync_info: SyncInfo,
    pub node_info: NodeInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Balance {
    pub symbol: String,
    pub free: Fixed8,
    pub locked: Fixed8,
    pub frozen: Fixed8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
    pub account_number: i64,
    pub address: String,
    pub balances: Vec<Balance>,
    pub public_key: Vec<u8>,
    pub sequence: i64,
    pub flags: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Intervals {
    #[serde(rename = "1m")]
    T1m,
    #[serde(rename = "3m")]
    T3m,
    #[serde(rename = "5m")]
    T5m,
    #[serde(rename = "15m")]
    T15m,
    #[serde(rename = "30m")]
    T30m,
    #[serde(rename = "1h")]
    T1h,
    #[serde(rename = "2h")]
    T2h,
    #[serde(rename = "4h")]
    T4h,
    #[serde(rename = "6h")]
    T6h,
    #[serde(rename = "8h")]
    T8h,
    #[serde(rename = "12h")]
    T12h,
    #[serde(rename = "1d")]
    T1d,
    #[serde(rename = "3d")]
    T3d,
    #[serde(rename = "1w")]
    T1w,
    #[serde(rename = "1M")]
    T1M,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct AccountSequence {
    pub sequence: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub original_symbol: String,
    pub total_supply: Fixed8,
    pub owner: String,
    pub mintable: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Market {
    pub base_asset_symbol: String,
    pub quote_asset_symbol: String,
    pub list_price: Fixed8,
    pub tick_size: Fixed8,
    pub lot_size: Fixed8,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum FixedFeeType {
    Proposer = 1,
    All = 2,
    Free = 3,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FixedFeeParams {
    pub msg_type: String,
    pub fee: u64,
    pub fee_for: FixedFeeType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DexFeeField {
    pub fee_name: String,
    pub fee_value: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Fee {
    Fixed {
        #[serde(flatten)]
        fixed_fee_params: FixedFeeParams,
    },
    Transfer {
        fixed_fee_params: FixedFeeParams,
        multi_transfer_fee: u64,
        lower_limit_as_multi: u64,
    },
    Dex {
        dex_fee_fields: Vec<DexFeeField>,
    },
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MarketDepth {
    pub asks: Vec<PriceQty>,
    pub bids: Vec<PriceQty>,
}

pub type Candlestick = (
    u64,    // 0          open time
    Fixed8, // 1         open price
    Fixed8, // 2      highest price
    Fixed8, // 3       lowest price
    Fixed8, // 4        close price
    Fixed8, // 5             volume
    u64,    // 6         close time
    Fixed8, // 7 quote asset volume
    i32,    // 8   number of trades
);

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum OrderSide {
    Buy = 1,
    Sell = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum OrderStatus {
    Ack,
    PartialFill,
    IocNoFill,
    FullyFill,
    Canceled,
    Expired,
    FailedBlocking,
    FailedMatching,
    IocExpire,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum OrderDuration {
    GoodTillExpire = 1,
    ImmediateOrCancel = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum OrderType {
    Limit = 2,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: String,
    pub symbol: String,
    pub owner: String,
    pub price: Fixed8,
    pub side: OrderSide,
    pub quantity: Fixed8,
    pub cumulate_quantity: Fixed8,
    pub status: OrderStatus,
    pub fee: String,
    pub time_in_force: OrderDuration,
    pub last_executed_price: Fixed8,
    pub last_executed_quantity: Fixed8,
    pub order_create_time: DateTime<Utc>,
    pub trade_id: String,
    pub transaction_time: DateTime<Utc>,
    pub transaction_hash: String,
    #[serde(rename = "type")]
    pub type_: OrderType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OrderList {
    #[serde(rename = "order")]
    pub orders: Vec<Order>,
    pub total: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    pub quote_asset_name: String,
    pub base_asset_name: String,
    pub ask_price: Fixed8,
    pub ask_quantity: Fixed8,
    pub bid_price: Fixed8,
    pub bid_quantity: Fixed8,
    pub close_time: u64,
    pub count: u64,
    pub first_id: String,
    pub high_price: Fixed8,
    pub last_id: String,
    pub last_price: Fixed8,
    pub last_quantity: Fixed8,
    pub low_price: Fixed8,
    pub open_price: Fixed8,
    pub open_time: u64,
    pub prev_close_price: Fixed8,
    pub price_change: Fixed8,
    pub price_change_percent: String,
    pub quote_volume: Fixed8,
    pub volume: Fixed8,
    pub weighted_avg_price: Fixed8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradePage {
    pub total: i32,
    #[serde(rename = "trade")]
    pub trades: Vec<Trade>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TickType {
    Unknown,
    SellTaker,
    BuyTaker,
    BuySurplus,
    SellSurplus,
    Neutral,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub base_asset: String,
    pub block_height: BlockHeight,
    pub buy_fee: InlineFee,
    pub buyer_id: String,
    pub buyer_order_id: String,
    pub buy_single_fee: InlineFee,
    pub buyer_source: i64,
    pub price: Fixed8,
    pub quantity: Fixed8,
    pub quote_asset: String,
    pub sell_fee: InlineFee,
    pub seller_id: String,
    pub seller_order_id: String,
    pub sell_single_fee: InlineFee,
    pub seller_source: i64,
    pub symbol: String,
    pub tick_type: TickType,
    pub time: u64,
    pub trade_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockExchangeFeePage {
    #[serde(rename = "blockExchangeFee")]
    pub block_exchange_fees: Vec<BlockExchangeFee>,
    pub total: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockExchangeFee {
    pub address: String,
    pub block_height: BlockHeight,
    pub block_time: u64,
    pub fee: InlineFee,
    pub trade_count: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AtomicSwapPage {
    pub atomic_swaps: Vec<AtomicSwap>,
    pub total: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AtomicSwap {
    pub block_timestamp: u64,
    pub closed_time: Option<u64>,
    pub cross_chain: i64,
    pub expected_income: String,
    pub expire_height: i64,
    pub from_addr: String,
    pub to_addr: String,
    pub in_amount: Option<String>,
    pub out_amount: Option<String>,
    pub random_number: Option<String>,
    pub random_number_hash: String,
    pub recipient_other_chain: String,
    pub status: i64,
    pub swap_id: String,
    pub timestamp: u64, // MEASURED IN SECONDS
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LockedCoin {
    pub symbol: String,
    pub amount: Fixed8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeLocks {
    pub id: i64,
    pub description: String,
    pub amount: Vec<LockedCoin>,
    pub locktime: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MiniTokens {
    pub name: String,
    pub symbol: String,
    pub original_symbol: String,
    pub total_supply: Fixed8,
    pub token_type: i64,
    pub owner: String,
    pub mintable: bool,
    pub token_uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dummy {
    pub _id: String,
}
