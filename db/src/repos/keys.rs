use aes_gcm::{
    aead::{Aead, OsRng},
    AeadCore, Aes256Gcm, KeyInit,
};
use async_trait::async_trait;
use common::Result;
use secstr::SecStr;
use sqlx::{query_file, query_file_as, PgPool};
use uuid::Uuid;

use crate::models::key::{CreateKey, Key, KeyKind, UpdateKey};

#[async_trait]
#[mockall::automock]
pub trait KeyRepo: Send + Sync {
    async fn list(
        &self,
        service_id: &Uuid,
        is_active: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Key>>;
    async fn create(&self, master_key: &SecStr, data: &CreateKey) -> Result<Key>;
    async fn get(&self, id: &Uuid) -> Result<Option<Key>>;
    async fn update(&self, id: &Uuid, data: &UpdateKey) -> Result<Key>;
    async fn destroy(&self, id: &Uuid) -> Result<Key>;
    async fn preferred(&self, service_id: &Uuid, kind: &KeyKind) -> Result<Key>;
}

pub struct PgKeyRepo {
    pool: PgPool,
}

impl PgKeyRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl KeyRepo for PgKeyRepo {
    async fn list(
        &self,
        service_id: &Uuid,
        is_active: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Key>> {
        let key = query_file_as!(
            Key,
            "sql/keys/list.sql",
            service_id,
            is_active,
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(key)
    }

    async fn create(&self, master_key: &SecStr, data: &CreateKey) -> Result<Key> {
        let master_bytes = hex::decode(master_key.unsecure())?;
        let key_bytes = data.value.as_bytes();

        let cipher = Aes256Gcm::new(master_bytes.as_slice().into());
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let mut key_encrypted = cipher.encrypt(&nonce, key_bytes)?;
        key_encrypted.splice(0..0, nonce.iter().cloned());

        let mut tx = self.pool.begin().await?;

        query_file!(
            "sql/keys/bump-priority.sql",
            &data.service_id,
            data.kind.clone() as KeyKind
        )
        .execute(&mut *tx)
        .await?;

        let key = query_file_as!(
            Key,
            "sql/keys/create.sql",
            data.service_id,
            data.kind.clone() as KeyKind,
            key_encrypted,
            data.priority
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(key)
    }

    async fn get(&self, id: &Uuid) -> Result<Option<Key>> {
        let key = query_file_as!(Key, "sql/keys/get.sql", &id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(key)
    }

    async fn update(&self, id: &Uuid, data: &UpdateKey) -> Result<Key> {
        let key = self.get(id).await?.ok_or(sqlx::Error::RowNotFound)?;

        let is_active = data.is_active.unwrap_or(key.is_active);

        let key = query_file_as!(Key, "sql/keys/update.sql", &id, is_active)
            .fetch_one(&self.pool)
            .await?;

        Ok(key)
    }

    async fn destroy(&self, id: &Uuid) -> Result<Key> {
        let key = query_file_as!(Key, "sql/keys/destroy.sql", &id)
            .fetch_one(&self.pool)
            .await?;

        Ok(key)
    }

    async fn preferred(&self, service_id: &Uuid, kind: &KeyKind) -> Result<Key> {
        let key = query_file_as!(
            Key,
            "sql/keys/preferred.sql",
            service_id,
            kind.clone() as KeyKind
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(key)
    }
}
