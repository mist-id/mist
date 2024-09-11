use base64::prelude::*;
use common::Result;
use hmac::{Hmac, Mac};
use openidconnect::{CsrfToken, Nonce};
use sha2::Sha256;
use uuid::Uuid;

pub fn created_signed_state(key: &[u8], csrf: &str, uuid: &Uuid) -> Result<CsrfToken> {
    let token = CsrfToken::new(format!(
        "{}:{}:{}",
        csrf,
        uuid,
        sign_state(key, csrf, uuid)?
    ));

    Ok(token)
}

pub fn sign_state(key: &[u8], csrf: &str, uuid: &Uuid) -> Result<String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)?;
    mac.update(csrf.as_bytes());
    mac.update(uuid.as_bytes());

    Ok(BASE64_URL_SAFE.encode(mac.finalize().into_bytes()))
}

pub fn create_signed_nonce(key: &[u8], nonce: &str) -> Result<Nonce> {
    let nonce = Nonce::new(format!("{}:{}", nonce, sign_nonce(key, nonce)?));

    Ok(nonce)
}

pub fn sign_nonce(key: &[u8], nonce: &str) -> Result<String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)?;
    mac.update(nonce.as_bytes());

    Ok(BASE64_URL_SAFE.encode(mac.finalize().into_bytes()))
}
