use lazy_static::lazy_static;
use log::warn;
use std::env::var;

lazy_static! {
    pub static ref BASE_URL: &'static str = {
        if var("BINANCE_DEX_TESTNET").unwrap_or_else(|_| "0".to_string()) == "0" {
            "https://dex.binance.org"
        } else {
            warn!("Using Binance DEX testnet API");
            "https://testnet-dex.binance.org"
        }
    };
}
