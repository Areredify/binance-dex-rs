use std::collections::HashMap;

use failure::Fallible;
use fehler::{throw, throws};
use futures::{
    sink::Sink,
    stream::{SplitStream, Stream, StreamExt},
    task::{Context, Poll},
};
use log::trace;
use pin_project::pin_project;
use serde::Serialize;
use serde_json::{from_str, json, to_string};
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
    BinanceDexClient,
};

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

impl BinanceDexClient {
    #[throws(failure::Error)]
    pub async fn websocket(&self) -> BinanceDexWebsocket {
        BinanceDexWebsocket::new()
    }
}

#[pin_project]
#[derive(Default)]
pub struct BinanceDexWebsocket {
    #[pin]
    streams: StreamUnordered<WSStream>,
    tokens: HashMap<Topic, usize>,
    topics: HashMap<usize, Topic>,
}

#[derive(Serialize)]
pub struct Message<'a, T> {
    method: &'a str,
    #[serde(flatten)]
    payload: T,
}

impl BinanceDexWebsocket {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn subscribe(self: Pin<&mut Self>, topic: Topic) -> Fallible<SubscriptionToken> {
        let mut this = self.project();

        let (mut stream, _) = connect_async(Url::parse(&WS_URL).unwrap()).await?;

        let message = Message {
            method: "subscribe",
            payload: &topic,
        };
        Pin::new(&mut stream).start_send(WSMessage::Text(serde_json::to_string(&message)?))?;

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

#[throws(failure::Error)]
fn parse_message(msg: WSMessage, topic: &Topic) -> BinanceDexWsMessage {
    match msg {
        WSMessage::Text(message) => {
            let message = message.as_str();
            println!("received text message: {}", message);
            match from_str(message) {
                Ok(r) => r,
                Err(_) => BinanceDexWsMessage::Unknown,
            }
        }
        WSMessage::Close(_) => throw!(failure::Error::from_boxed_compat("websocket closed".into())),
        WSMessage::Binary(c) => throw!(failure::Error::from_boxed_compat(
            "unexpected binary content".into()
        )),
        WSMessage::Ping(_) => BinanceDexWsMessage::Ping,
        WSMessage::Pong(_) => BinanceDexWsMessage::Pong,
    }
}
