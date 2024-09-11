use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use common::Result;
use db::models::key::Key;
use secstr::SecVec;

pub fn decrypt_service_key(master_key: &SecVec<u8>, service_key: &Key) -> Result<Vec<u8>> {
    let master_key = hex::decode(master_key.unsecure())?;

    let (nonce, service_key) = service_key.value.split_at(12);
    let nonce = Nonce::from_slice(nonce);
    let cipher = Aes256Gcm::new(master_key.as_slice().into());

    let key = cipher.decrypt(nonce, service_key)?;

    Ok(key)
}
