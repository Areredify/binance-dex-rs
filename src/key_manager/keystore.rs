use std::{fs::File, path::Path};

use aes_ctr::stream_cipher::{NewStreamCipher, SyncStreamCipher};
use failure::Fallible;
use serde::Deserialize;
use sha3::Digest;

use crate::key_manager::KeyManager;

#[derive(Deserialize, Clone, Debug)]
struct EncryptedKeystore {
    crypto: Crypto,
    version: u64,
    id: String,
}

#[derive(Deserialize, Clone, Debug)]
struct Crypto {
    cipher: String,
    #[serde(rename = "ciphertext")]
    cipher_text: String,
    #[serde(rename = "cipherparams")]
    cipher_params: CipherParams,
    #[serde(flatten)]
    kdf: KDF,
    mac: String,
}

#[derive(Deserialize, Clone, Debug)]
struct CipherParams {
    iv: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "kdf", content = "kdfparams")]
enum KDF {
    #[serde(rename = "pbkdf2")]
    PBKDF2 {
        prf: String,
        dklen: u32,
        salt: String,
        c: u32,
    },
}

fn get_kdf_key(kdf: KDF, password: &str) -> Fallible<Vec<u8>> {
    match kdf {
        KDF::PBKDF2 {
            prf,
            dklen,
            salt,
            c,
        } => {
            if prf != "hmac-sha256" {
                return Err(failure::format_err!("unsupported PBKDF2 PRF: {}", prf));
            }

            let salt = hex::decode(salt)?;
            let password = password.as_bytes();
            let mut buf = vec![0u8; dklen as usize];
            pbkdf2::pbkdf2::<hmac::Hmac<sha2::Sha256>>(password, &salt, c, &mut buf);
            Ok(buf)
        }
    }
}

fn aes_decrypt(key: &[u8], text: &mut [u8], iv: &[u8]) -> Fallible<()> {
    let mut aes = <aes_ctr::Aes256Ctr as NewStreamCipher>::new_var(key, iv)
        .map_err(|_| failure::format_err!("error while decrypting AES"))?;
    aes.apply_keystream(text);
    Ok(())
}

fn verify_mac(mac: &[u8], kdf_key: &[u8], cipher_text: &[u8]) -> Fallible<()> {
    let mut hasher = sha3::Keccak512::default();
    hasher.update(&kdf_key[16..32]);
    hasher.update(&cipher_text);
    let calculated_mac = hasher.finalize().as_slice().to_vec();

    if calculated_mac != mac {
        let mut sha256 = sha2::Sha256::default();
        sha256.update(&kdf_key[16..32]);
        sha256.update(&cipher_text);
        let calculated_mac = sha256.finalize().as_slice().to_vec();

        if calculated_mac != mac {
            return Err(failure::format_err!(
                "error while decrypting keystore: mismatched MAC"
            ));
        }
    }

    Ok(())
}

impl KeyManager {
    pub fn from_keystore<P: AsRef<Path>>(path: P, password: &str) -> Fallible<KeyManager> {
        let file = File::open(path)?;
        let encrypted: EncryptedKeystore = serde_json::from_reader(&file)?;

        // Decode hex strings from the file
        let mac = hex::decode(encrypted.crypto.mac)?;
        let iv = hex::decode(encrypted.crypto.cipher_params.iv)?;
        let mut cipher_text = hex::decode(encrypted.crypto.cipher_text)?;

        // Decryption
        let kdf_key = get_kdf_key(encrypted.crypto.kdf, password)?;
        verify_mac(&mac, &kdf_key, &cipher_text)?;

        aes_decrypt(&kdf_key, &mut cipher_text, &iv)?;
        KeyManager::from_private_key_bytes(&cipher_text)
    }
}
