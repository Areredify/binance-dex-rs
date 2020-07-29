use prost_amino::Message;
use prost_amino_derive::Message;
use serde::{Deserialize, Serialize, Serializer};

use crate::key_manager::address_to_str;

fn i64_to_string<S: Serializer>(x: &i64, s: S) -> std::result::Result<S::Ok, S::Error> {
    s.serialize_str(&x.to_string())
}

fn serialize_address<S: Serializer>(address: &[u8], s: S) -> std::result::Result<S::Ok, S::Error> {
    s.serialize_str(&address_to_str(address).unwrap())
}

use prost_amino::{
    bytes::{Buf, BufMut},
    DecodeError,
};

#[derive(Clone, Message)]
#[amino_name = "auth/StdTx"]
pub struct StdTx {
    #[prost_amino(message, repeated, tag = "1")]
    pub msgs: Vec<Msg>,
    #[prost_amino(message, repeated, tag = "2")]
    pub signatures: Vec<StdSignature>,
    #[prost_amino(string, tag = "3")]
    pub memo: String,
    #[prost_amino(int64, tag = "4")]
    pub source: i64,
    #[prost_amino(bytes, tag = "5")]
    pub data: Vec<u8>,
}

#[derive(Serialize, Clone, Debug)]
pub struct StdSignMsg<'a> {
    #[serde(serialize_with = "i64_to_string")]
    pub account_number: i64,
    pub chain_id: &'a str,
    pub memo: &'a str,
    pub msgs: &'a [&'a Msg],
    #[serde(serialize_with = "i64_to_string")]
    pub sequence: i64,
    #[serde(serialize_with = "i64_to_string")]
    pub source: i64,
    pub data: Option<Vec<u8>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TxCommitResult {
    pub ok: bool,
    pub log: String,
    pub hash: String,
    pub code: i32,
    pub data: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Msg {
    CancelOrder(CancelOrder),
    CreateOrder(CreateOrder),
    Transfer(Transfer),
}

impl Message for Msg {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
        Self: Sized,
    {
        match self {
            Self::CancelOrder(m) => m.encode_raw(buf),
            Self::CreateOrder(m) => m.encode_raw(buf),
            Self::Transfer(m) => m.encode_raw(buf),
        }
    }

    fn encoded_len(&self) -> usize {
        match self {
            Self::CancelOrder(m) => m.encoded_len(),
            Self::CreateOrder(m) => m.encoded_len(),
            Self::Transfer(m) => m.encoded_len(),
        }
    }

    // FIXME(areredify):
    // For now, I don't want to support decoding transactions, maybe later
    fn merge_field<B>(&mut self, _buf: &mut B) -> Result<(), DecodeError>
    where
        B: Buf,
        Self: Sized,
    {
        unimplemented!()
    }

    fn clear(&mut self) {
        *self = Default::default();
    }
}

impl Default for Msg {
    fn default() -> Self {
        Self::CancelOrder(Default::default())
    }
}

#[derive(Clone, Message)]
pub struct StdSignature {
    #[prost_amino(bytes, tag = "1", amino_name = "tendermint/PubKeySecp256k1")]
    pub pub_key: Vec<u8>,
    #[prost_amino(bytes, tag = "2")]
    pub signature: Vec<u8>,
    #[prost_amino(int64, tag = "3")]
    pub account_number: i64,
    #[prost_amino(int64, tag = "4")]
    pub sequence: i64,
}

#[derive(Serialize, Clone, Message)]
#[amino_name = "dex/NewOrder"]
pub struct CreateOrder {
    #[serde(serialize_with = "serialize_address")]
    #[prost_amino(bytes, tag = "1")]
    pub sender: Vec<u8>,
    #[prost_amino(string, tag = "2")]
    pub id: String,
    #[prost_amino(string, tag = "3")]
    pub symbol: String,
    #[prost_amino(int64, tag = "4")]
    pub ordertype: i64,
    #[prost_amino(int64, tag = "5")]
    pub side: i64,
    #[prost_amino(int64, tag = "6")]
    pub price: i64,
    #[prost_amino(int64, tag = "7")]
    pub quantity: i64,
    #[prost_amino(int64, tag = "8")]
    pub timeinforce: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateOrderResponse {
    pub order_id: String,
}

#[derive(Serialize, Clone, Message)]
#[amino_name = "dex/CancelOrder"]
pub struct CancelOrder {
    #[serde(serialize_with = "serialize_address")]
    #[prost_amino(bytes, tag = "1")]
    pub sender: Vec<u8>,
    #[prost_amino(string, tag = "2")]
    pub symbol: String,
    #[prost_amino(string, tag = "3")]
    pub refid: String,
}

#[derive(Serialize, Clone, Message)]
pub struct Coin {
    #[prost_amino(string, tag = "1")]
    pub denom: String,
    #[prost_amino(int64, tag = "2")]
    pub amount: i64,
}

#[derive(Serialize, Clone, Message)]
pub struct TransferIO {
    #[serde(serialize_with = "serialize_address")]
    #[prost_amino(bytes, tag = "1")]
    pub address: Vec<u8>,
    #[prost_amino(message, repeated, tag = "2")]
    pub coins: Vec<Coin>,
}

#[derive(Serialize, Clone, Message)]
#[amino_name = "cosmos-sdk/Send"]
pub struct Transfer {
    #[prost_amino(message, repeated, tag = "1")]
    pub inputs: Vec<TransferIO>,
    #[prost_amino(message, repeated, tag = "2")]
    pub outputs: Vec<TransferIO>,
}
