use binance_dex_rs::{
    model::*, query, BinanceDexClient, Fixed8, KeyManager, TransactionOptions, Transfer,
};
use failure::Fallible;

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let km = KeyManager::from_keystore("{keystore_path}", "{password}")?;
    let address = km.account_address_str.to_string();
    println!("{}", address);
    let client = BinanceDexClient::with_credentials(km);

    let transfer = Transfer::multiple_coins(
        "tbnb1w43vfmj35m8cpt64ymt2sgasutxfkq65lav26l".into(),
        vec![
            ("RUNE-67C".into(), Fixed8(10000)),
            ("AWC-31D".into(), Fixed8(10000)),
        ]
        .into_iter(),
    );

    let resp = client
        .transfer(
            transfer,
            TransactionOptions {
                memo: None,
                acc_info: None,
                sync: true,
            },
        )
        .await?;
    println!("{:#?}", resp);

    let resp = client
        .create_order(
            "XRP-FB8",
            "BNB",
            OrderSide::Buy,
            OrderDuration::ImmediateOrCancel,
            Fixed8(1197460),
            Fixed8(10000000),
            TransactionOptions {
                memo: None,
                acc_info: None,
                sync: true,
            },
        )
        .await?;

    println!("{:#?}", resp);

    Ok(())
}
