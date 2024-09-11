use async_trait::async_trait;
use common::Result;
use sqlx::{query_file_as, PgPool};

use crate::models::identifier::{CreateIdentifier, Identifier};

#[async_trait]
#[mockall::automock]
pub trait IdentifierRepo: Send + Sync {
    async fn create(&self, data: &CreateIdentifier) -> Result<Identifier>;
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
            data.user_id,
            data.value,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(identifier)
    }
}
