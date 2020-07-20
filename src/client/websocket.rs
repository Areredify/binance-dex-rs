use std::collections::HashMap;

use failure::Fallible;
use futures::{
    sink::SinkExt,
    stream::Stream,
    task::{Context, Poll},
};
use log::debug;
use pin_project::pin_project;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::from_str;
use std::pin::Pin;
use streamunordered::{StreamUnordered, StreamYield};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message as WSMessage, MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::{
    api_url::WS_URL,
    model::websocket::{Message as BinanceDexWsMessage, SubscriptionToken, Topic},
};

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[pin_project]
#[derive(Default)]
pub struct BinanceDexWebsocket {
    #[pin]
    streams: StreamUnordered<WSStream>,
    tokens: HashMap<Topic, usize>,
    topics: HashMap<usize, Topic>,
}

impl BinanceDexWebsocket {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn subscribe(self: Pin<&mut Self>, topic: Topic) -> Fallible<SubscriptionToken> {
        let mut this = self.project();

        let (mut stream, _) = connect_async(Url::parse(&WS_URL).unwrap()).await?;
        let subscribe_message = topic.to_subscribe_message();
        debug!("{}", subscribe_message);

        stream.send(WSMessage::Text(subscribe_message)).await?;

        let token = this.streams.push(stream);
        this.tokens.insert(topic.clone(), token);
        this.topics.insert(token, topic.clone());

        Ok(SubscriptionToken(token))
    }

    pub async fn unsubscribe(self: Pin<&mut Self>, token: SubscriptionToken) -> Option<WSStream> {
        let this = self.project();
        this.streams.take(token.0)
    }

    pub async fn unsubscribe_topic(self: Pin<&mut Self>, topic: Topic) -> Option<WSStream> {
        let this = self.project();
        this.streams.take(*this.tokens.get(&topic)?)
    }
}

impl Stream for BinanceDexWebsocket {
    type Item = Fallible<BinanceDexWsMessage>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let poll = this.streams.poll_next(cx);
        match poll {
            Poll::Ready(Some((y, token))) => match y {
                StreamYield::Item(item) => {
                    let topic = this.topics.get(&token).unwrap();
                    Poll::Ready({
                        Some(
                            item.map_err(failure::Error::from)
                                .and_then(|msg| parse_message(msg, topic)),
                        )
                    })
                }
                StreamYield::Finished(_) => Poll::Pending,
            },
            Poll::Ready(None) => Poll::Pending,
            Poll::Pending => Poll::Pending,
        }
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Payload<T> {
    stream: String,
    data: T,
}

fn get_data<T: DeserializeOwned>(msg: &str) -> Fallible<T> {
    let payload: Payload<T> = from_str(msg)?;
    Ok(payload.data)
}

fn parse_message(msg: WSMessage, topic: &Topic) -> Fallible<BinanceDexWsMessage> {
    let msg = match msg {
        WSMessage::Text(msg) => msg,
        WSMessage::Binary(b) => return Ok(BinanceDexWsMessage::Binary(b)),
        WSMessage::Pong(..) => return Ok(BinanceDexWsMessage::Pong),
        WSMessage::Ping(..) => return Ok(BinanceDexWsMessage::Ping),
        WSMessage::Close(..) => {
            return Err(failure::format_err!("Socket with topic {:?} closed", topic))
        }
    };

    debug!("Incoming websocket message {}", msg);

    let message = match topic {
        Topic::Accounts { .. } => BinanceDexWsMessage::Accounts(get_data(&msg)?),
        Topic::Orders { .. } => BinanceDexWsMessage::Orders(get_data(&msg)?),
        Topic::Transfers { .. } => BinanceDexWsMessage::Transfers(get_data(&msg)?),
        Topic::MarketDiff { .. } => BinanceDexWsMessage::MarketDiff(get_data(&msg)?),
        Topic::Trades { .. } => BinanceDexWsMessage::Trades(get_data(&msg)?),
        Topic::MarketDepth { .. } => BinanceDexWsMessage::MarketDepth(get_data(&msg)?),
        Topic::Candlestick { .. } => BinanceDexWsMessage::Candlestick(get_data(&msg)?),
        Topic::Ticker { .. } => BinanceDexWsMessage::Ticker(get_data(&msg)?),
        Topic::AllTickers { .. } => BinanceDexWsMessage::AllTickers(get_data(&msg)?),
        Topic::MiniTicker { .. } => BinanceDexWsMessage::MiniTicker(get_data(&msg)?),
        Topic::AllMiniTickers { .. } => BinanceDexWsMessage::AllMiniTickers(get_data(&msg)?),
        Topic::BlockHeight { .. } => BinanceDexWsMessage::BlockHeight(get_data(&msg)?),
    };

    Ok(message)
}
