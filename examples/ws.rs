use binance_dex_rs::{BinanceDexWebsocket, Topic};
use failure::Fallible;
use futures::stream::StreamExt;
use std::pin::Pin;

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let rune = "RUNE-B1A_BNB".to_string();
    let ava = "AVA-645_BNB".to_string();
    let cbm = "CBM-4B2_BNB".to_string();

    // good symbol pairs: AVA-645_BNB, CBM-4B2_BNB
    // good order address bnb1w7puzjxu05ktc5zvpnzkndt6tyl720nsutzvpg
    // good transaction address bnb1dn3mhh2gl7vk38w6ppncrvzwzhccvvje49ymkk

    let mut websocket = BinanceDexWebsocket::new();

    Pin::new(&mut websocket)
        // Transfers
        // .subscribe(Topic::Transfers {
        //     address: "bnb1dn3mhh2gl7vk38w6ppncrvzwzhccvvje49ymkk".into(),
        // })
        // .await?;

        // Trades
        .subscribe(Topic::Trades{
            symbols: vec![rune, ava, cbm],
        })
        .await?;

        // MarketDepth
        // .subscribe(Topic::MarketDepth{
        //     amount: None,
        //     symbols: vec![rune, ava, cbm],
        // })
        // .await?;

        // Candlestick
        // .subscribe(Topic::Candlestick{
        //     interval: Intervals::T1h,
        //     symbols: vec![rune, ava, cbm],
        // })
        // .await?;

        // Ticker
        // .subscribe(Topic::Ticker{
        //     symbols: vec![rune, ava, cbm],
        // })

        // .await?;
        // AllTickers
        // .subscribe(Topic::AllTickers{
        // })
        // .await?;

        // MinTicker
        // .subscribe(Topic::MiniTicker {
        //     symbols: vec![rune, ava, cbm],
        // })
        // .await?;

        // AllMiniTickers
        // .subscribe(Topic::AllMiniTickers {
        // })
        // .await?;

        // Blockheight
        // .subscribe(Topic::Blockheight)
        // .await?;

    while let Some(msg) = websocket.next().await {
        println!("{:#?}", msg);
    }
    Ok(())
}
