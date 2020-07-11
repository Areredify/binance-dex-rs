use binance_dex_rs::{BinanceDexClient, Topic};
use failure::Fallible;
use futures::stream::StreamExt;
use std::pin::Pin;

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    // good symbol pairs: AVA-645_BNB, CBM-4B2_BNB
    // good order address bnb1w7puzjxu05ktc5zvpnzkndt6tyl720nsutzvpg
    // good transaction address bnb1dn3mhh2gl7vk38w6ppncrvzwzhccvvje49ymkk

    let client = BinanceDexClient::new();
    let mut websocket = client.websocket().await?;

    Pin::new(&mut websocket)
        .subscribe(Topic::Transfers {
            address: "bnb1dn3mhh2gl7vk38w6ppncrvzwzhccvvje49ymkk".into(),
        })
        .await?;

    while let Some(msg) = websocket.next().await {
        println!("{:#?}", msg);
    }
    Ok(())
}
