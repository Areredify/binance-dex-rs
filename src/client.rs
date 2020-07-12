use failure::Fallible;
use log::error;
use reqwest::{Client, Method, Response};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string, to_value};
use url::Url;

use crate::{api_url::HTTP_URL, model::Error as BinanceError, query::Query};

pub mod websocket;

#[derive(Default)]
pub struct BinanceDexClient {
    client: Client,
}

impl BinanceDexClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn query<Q: Query>(&self, request: Q) -> Fallible<Q::Response> {
        let url = format!("{}{}", *HTTP_URL, request.get_endpoint());
        let url = Url::parse_with_params(&url, request.to_url_query())?;

        let req = self
            .client
            .request(Method::GET, url.as_str())
            .header("user-agent", "binance-dex-rs");

        self.handle_response(req.send().await?).await
    }

    async fn handle_response<T: DeserializeOwned>(&self, resp: Response) -> Fallible<T> {
        if resp.status().is_success() {
            let resp = resp.text().await?;
            match from_str::<T>(&resp) {
                Ok(resp) => Ok(resp),
                Err(e) => {
                    error!("Cannot deserialize '{}'", resp);
                    Err(e.into())
                }
            }
        } else {
            let resp_e = resp.error_for_status_ref().unwrap_err();
            if let Ok(e) = resp.json::<BinanceError>().await {
                Err(e.into())
            } else {
                Err(resp_e.into())
            }
        }
    }
}

trait ToUrlQuery: Serialize {
    fn to_url_query(&self) -> Vec<(String, String)> {
        let v = to_value(self).unwrap();
        let v = if let Some(v) = v.as_object() {
            v
        } else {
            return vec![];
        };

        let mut vec = vec![];

        for (key, value) in v.into_iter() {
            if value.is_null() {
                continue;
            } else if value.is_string() {
                vec.push((key.clone(), value.as_str().unwrap().to_string()))
            } else {
                vec.push((key.clone(), to_string(value).unwrap()))
            }
        }
        vec
    }
}

impl<S: Serialize> ToUrlQuery for S {}
