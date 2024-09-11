use async_trait::async_trait;
use common::Result;
use sqlx::{query_file_as, PgPool};

use crate::models::definition::{CreateDefinition, Definition};

#[async_trait]
#[mockall::automock]
pub trait DefinitionRepo: Send + Sync {
    async fn create(&self, data: &CreateDefinition) -> Result<Definition>;
}

pub struct PgDefinitionRepo {
    pool: PgPool,
}

impl PgDefinitionRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DefinitionRepo for PgDefinitionRepo {
    async fn create(&self, data: &CreateDefinition) -> Result<Definition> {
        let profile = query_file_as!(
            Definition,
            "sql/definitions/create.sql",
            &data.service_id,
            data.name,
            serde_json::to_value(&data.value)?,
            data.is_default
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(profile)
    }
}
