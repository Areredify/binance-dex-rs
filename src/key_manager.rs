use std::io::Write;

use bech32::ToBase32;
use failure::Fallible;
use hex;
use ripemd160::Ripemd160;
use secp256k1::{PublicKey, Secp256k1, SecretKey, SignOnly};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::api_url::NET_PREFIX;

#[derive(Clone, Debug)]
pub struct KeyManager {
    private_key: SecretKey,
    context: Secp256k1<SignOnly>,
    pub account_address: Vec<u8>,
    pub account_address_str: String,
    pub public_key: Vec<u8>,
}

fn get_address(public_key: &[u8]) -> Fallible<Vec<u8>> {
    let mut sha = Sha256::default();
    sha.write_all(&public_key)?;

    let key = sha.finalize();

    let mut ripemd = Ripemd160::default();
    ripemd.write_all(&key)?;

    let address = ripemd.finalize();

    Ok(address.as_slice().into())
}

pub fn address_to_str(address: &[u8]) -> Fallible<String> {
    Ok(bech32::encode(*NET_PREFIX, address.to_base32())?)
}

impl KeyManager {
    pub fn from_private_key(private_key: &str) -> Fallible<KeyManager> {
        let private_key = hex::decode(private_key)?;
        let private_key = SecretKey::from_slice(&private_key)?;
        let context = Secp256k1::signing_only();
        let public_key = PublicKey::from_secret_key(&context, &private_key).serialize();
        let account_address = get_address(&public_key)?;
        let public_key = public_key.to_vec();
        let account_address_str = address_to_str(&account_address)?;

        Ok(Self {
            private_key,
            public_key,
            account_address,
            account_address_str,
            context,
        })
    }

    pub fn sign<M: Serialize>(&self, msg: M) -> Fallible<Vec<u8>> {
        let bytes = serde_json::to_vec(&serde_json::to_value(msg)?)?;
        let mut sha = Sha256::default();
        sha.write_all(&bytes)?;
        let hash = sha.finalize();
        Ok(self
            .context
            .sign(&secp256k1::Message::from_slice(&hash)?, &self.private_key)
            .serialize_compact()
            .to_vec())
    }
}
