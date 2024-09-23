use async_trait::async_trait;
use mist_common::Result;
use sqlx::{query_file_as, PgPool};

use crate::models::identifier::{CreateIdentifier, Identifier, IdentifierId};

#[async_trait]
#[mockall::automock]
pub trait IdentifierRepo: Send + Sync {
    async fn create(&self, data: &CreateIdentifier) -> Result<Identifier>;
    async fn get(&self, id: &IdentifierId) -> Result<Identifier>;
    async fn get_by_value(&self, value: &str) -> Result<Identifier>;
}

pub struct PgIdentifierRepo {
    pool: PgPool,
}

impl PgIdentifierRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IdentifierRepo for PgIdentifierRepo {
    async fn create(&self, data: &CreateIdentifier) -> Result<Identifier> {
        let identifier = query_file_as!(
            Identifier,
            "sql/identifiers/create.sql",
            data.user_id.as_ref(),
            data.value,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(identifier)
    }

    async fn get(&self, id: &IdentifierId) -> Result<Identifier> {
        let identifier = query_file_as!(Identifier, "sql/identifiers/get.sql", id.as_ref())
            .fetch_one(&self.pool)
            .await?;

        Ok(identifier)
    }
    async fn get_by_value(&self, value: &str) -> Result<Identifier> {
        let identifier = query_file_as!(Identifier, "sql/identifiers/get_by_value.sql", value)
            .fetch_one(&self.pool)
            .await?;

        Ok(identifier)
    }
}
