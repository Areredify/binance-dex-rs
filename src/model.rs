// Binance DEX/Chain API model module.
// Reference: https://docs.binance.org/api-reference/dex-api/paths.html

// Note: Binance Chain uses RFC3339 for date representation, which
// matches default representation for serde, so no custom logic is required
// when de-/serializing DateTime.

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Times {
    ap_time: DateTime<Utc>,
    block_time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Validators {
    block_height: i64,
    validators: Vec<Validator>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Validator {
    address: String,
    pub_key: Vec<i32>,
    voting_power: i64,
    #[serde(rename = "accum")]
    voting_accum_power: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Peer {
    id: String,
    original_listen_addr: String,
    listen_addr: String,
    access_addr: String,
    stream_addr: String,
    network: String,
    version: String,
    moniker: String,
    capabilites: Vec<String>,
    #[serde(rename = "accelerated")]
    is_accelerated: bool,
}

/*
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    hash: String,
    log: String,
    data: String,
    height: String
}
*/
