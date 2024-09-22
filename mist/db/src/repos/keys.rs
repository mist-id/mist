use async_trait::async_trait;
use common::{
    crypto::{create_service_key, encrypt_service_key},
    Result,
};
use eyre::eyre;
use secstr::SecVec;
use sqlx::{query_file, query_file_as, query_file_scalar, PgPool};

use crate::models::{
    key::{CreateKey, Key, KeyId, KeyKind, UpdateKey},
    service::ServiceId,
};

#[async_trait]
#[mockall::automock]
pub trait KeyRepo: Send + Sync {
    async fn list(
        &self,
        service_id: &ServiceId,
        is_active: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Key>>;
    async fn create(&self, master_key: &SecVec<u8>, data: &CreateKey) -> Result<Key>;
    async fn get(&self, id: &KeyId) -> Result<Key>;
    async fn update(&self, id: &KeyId, data: &UpdateKey) -> Result<Key>;
    async fn destroy(&self, id: &KeyId) -> Result<Key>;
    async fn preferred(&self, service_id: &ServiceId, kind: &KeyKind) -> Result<Key>;
    async fn has_active_key_of_kind(&self, service_id: &ServiceId, kind: &KeyKind) -> Result<bool>;
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
        service_id: &ServiceId,
        is_active: bool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Key>> {
        let key = query_file_as!(
            Key,
            "sql/keys/list.sql",
            service_id.as_ref(),
            is_active,
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(key)
    }

    async fn create(&self, master_key: &SecVec<u8>, data: &CreateKey) -> Result<Key> {
        let key = create_service_key();
        let key_encrypted = encrypt_service_key(master_key, &key)?;

        let mut tx = self.pool.begin().await?;

        query_file!(
            "sql/keys/bump-priority.sql",
            &data.service_id.as_ref(),
            data.kind.clone() as KeyKind
        )
        .execute(&mut *tx)
        .await?;

        let key = query_file_as!(
            Key,
            "sql/keys/create.sql",
            data.service_id.as_ref(),
            data.kind.clone() as KeyKind,
            key_encrypted,
            data.priority
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(key)
    }

    async fn get(&self, id: &KeyId) -> Result<Key> {
        let key = query_file_as!(Key, "sql/keys/get.sql", &id.as_ref())
            .fetch_one(&self.pool)
            .await?;

        Ok(key)
    }

    async fn update(&self, id: &KeyId, data: &UpdateKey) -> Result<Key> {
        let key = self.get(id).await?;

        let is_active = data.is_active.unwrap_or(key.is_active);

        let key = query_file_as!(Key, "sql/keys/update.sql", &id.as_ref(), is_active)
            .fetch_one(&self.pool)
            .await?;

        Ok(key)
    }

    async fn destroy(&self, id: &KeyId) -> Result<Key> {
        let key = self.get(id).await?;

        let can_delete = self
            .has_active_key_of_kind(&key.service_id.into(), &key.kind)
            .await?;

        if !can_delete {
            return Err(eyre!("key is in use").into());
        }

        let deleted = query_file_as!(Key, "sql/keys/destroy.sql", &id.as_ref())
            .fetch_one(&self.pool)
            .await?;

        Ok(deleted)
    }

    async fn preferred(&self, service_id: &ServiceId, kind: &KeyKind) -> Result<Key> {
        let key = query_file_as!(
            Key,
            "sql/keys/preferred.sql",
            service_id.as_ref(),
            kind.clone() as KeyKind
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(key)
    }

    async fn has_active_key_of_kind(&self, service_id: &ServiceId, kind: &KeyKind) -> Result<bool> {
        let exists = query_file_scalar!(
            "sql/keys/has-active-key-of-kind.sql",
            service_id.as_ref(),
            kind.clone() as KeyKind
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(false);

        Ok(exists)
    }
}
