use crate::error::Result;
use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, KeyInit, Nonce};
use rand::{rngs::OsRng, RngCore};
use secstr::SecVec;

pub fn create_service_key() -> Vec<u8> {
    let mut buffer = vec![0u8; 32];

    OsRng.fill_bytes(&mut buffer);

    buffer
}

pub fn encrypt_service_key(master_key: &SecVec<u8>, service_key: &[u8]) -> Result<Vec<u8>> {
    let master_bytes = hex::decode(master_key.unsecure())?;

    let cipher = Aes256Gcm::new(master_bytes.as_slice().into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let mut key_encrypted = cipher.encrypt(&nonce, service_key)?;
    key_encrypted.splice(0..0, nonce.iter().cloned());

    Ok(key_encrypted)
}

pub fn decrypt_service_key(master_key: &SecVec<u8>, service_key: &[u8]) -> Result<Vec<u8>> {
    let master_key = hex::decode(master_key.unsecure())?;

    let (nonce, service_key) = service_key.split_at(12);
    let nonce = Nonce::from_slice(nonce);
    let cipher = Aes256Gcm::new(master_key.as_slice().into());

    let key = cipher.decrypt(nonce, service_key)?;

    Ok(key)
}
