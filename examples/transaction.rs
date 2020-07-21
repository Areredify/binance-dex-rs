use binance_dex_rs::{model::*, BinanceDexClient, Fixed8, KeyManager, TransactionOptions};
use failure::Fallible;

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let km = KeyManager::from_private_key(
        "494d2cf5149505326ae596b586706fec8a70f3d434d918832037848ab7c4b17a",
    )?;
    println!("{}", km.account_address_str);

    let client = BinanceDexClient::with_credentials(km);

    let resp = client
        .create_order(
            "BNB",
            "BTCB-AFD",
            OrderSide::Buy,
            OrderDuration::ImmediateOrCancel,
            Fixed8(100000),
            Fixed8(10000),
            TransactionOptions {
                memo: None,
                acc_info: None,
                sync: true,
            },
        )
        .await?;

    println!("{:?}", resp);
    Ok(())
}
