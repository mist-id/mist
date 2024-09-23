use base64::prelude::*;
use hmac::{Hmac, Mac};
use mist_common::Result;
use openidconnect::{CsrfToken, Nonce};
use sha2::Sha256;

use crate::session::SessionId;

pub fn created_signed_state(key: &[u8], csrf: &str, id: &SessionId) -> Result<CsrfToken> {
    let token = CsrfToken::new(format!("{}:{}:{}", csrf, id, sign_state(key, csrf, id)?));

    Ok(token)
}

pub fn sign_state(key: &[u8], csrf: &str, id: &SessionId) -> Result<String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)?;
    mac.update(csrf.as_bytes());
    mac.update(id.as_ref().as_bytes());

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
