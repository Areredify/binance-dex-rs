use lazy_static::lazy_static;
use log::warn;
use std::env::var;

lazy_static! {
    pub static ref HTTP_URL: &'static str = {
        if var("BINANCE_DEX_TESTNET").unwrap_or_else(|_| "0".to_string()) == "0" {
            "https://dex.binance.org"
        } else {
            warn!("Using Binance DEX testnet API");
            "https://testnet-dex.binance.org"
        }
    };
    pub static ref WS_URL: &'static str = {
        if var("BINANCE_DEX_TESTNET").unwrap_or_else(|_| "0".to_string()) == "0" {
            "wss://dex.binance.org/api/ws"
        } else {
            warn!("Using Binance DEX testnet websocket API");
            "wss://testnet-dex.binance.org/api/ws"
        }
    };
    pub static ref NET_PREFIX: &'static str = {
        if var("BINANCE_DEX_TESTNET").unwrap_or_else(|_| "0".to_string()) == "0" {
            "bnb"
        } else {
            "tbnb"
        }
    };
    pub static ref CHAIN_ID: &'static str = {
        if var("BINANCE_DEX_TESTNET").unwrap_or_else(|_| "0".to_string()) == "0" {
            "Binance-Chain-Tigris"
        } else {
            "Binance-Chain-Ganges"
        }
    };
}
