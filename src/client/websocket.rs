use failure::Fallible;
use fehler::{throw, throws};
use futures::sink::Sink;
use futures::stream::{Stream, SplitStream};
use futures::task::{Context, Poll};
use streamunordered::{StreamUnordered, StreamYield};
use log::trace;
use pin_project::pin_project;
use serde_json::{from_str, to_string};
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message as WSMessage, MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::api_url::WS_URL;
use crate::model::websocket::Message as BinanceDexWsMessage;
use crate::BinanceDexClient;

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub type StoredStream = SplitStream<WSStream>;

impl BinanceDexClient {
    #[throws(failure::Error)]
    pub async fn websocket(&self) -> BinanceDexWebsocket {
        let (mut stream, _) = connect_async(Url::parse(&WS_URL).unwrap()).await?;
        let x = Pin::new(&mut stream);
        let message = serde_json::to_string(&serde_json::json!({ "method": "subscribe", "topic": "ticker", "symbols": ["TRYB-B5D_BNB", "MGT-3F0_BNB"] })).unwrap();
        //let message = serde_json::to_string(&serde_json::json!({ "method": "subscribe", "topic": "ticker", "symbols": ["TRYB-B5D_BNB"] })).unwrap();
        Pin::new(&mut stream).start_send(WSMessage::Text(message))?;
        BinanceDexWebsocket::new(stream)
    }
}

#[pin_project]
pub struct BinanceDexWebsocket {
    #[pin]
    inner: WSStream,
}

impl BinanceDexWebsocket {
    fn new(ws: WSStream) -> Self {
        Self { inner: ws }
    }
}

impl Stream for BinanceDexWebsocket {
    type Item = Fallible<BinanceDexWsMessage>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let poll = this.inner.poll_next(cx);
        match poll {
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(Some(Ok(m))) => match parse_message(m) {
                Ok(m) => Poll::Ready(Some(Ok(m))),
                Err(e) => Poll::Ready(Some(Err(e))),
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[throws(failure::Error)]
fn parse_message(msg: WSMessage) -> BinanceDexWsMessage {
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
