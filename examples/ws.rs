use binance_dex_rs::BinanceDexClient;
use failure::Fallible;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Fallible<()> {
    let client = BinanceDexClient::new();
    let mut websocket = client.websocket().await?;

    while let Some(msg) = websocket.next().await {
        println!("{:?}", msg);
    }
    Ok(())
}
