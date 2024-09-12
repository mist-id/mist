use async_trait::async_trait;
use common::Result;
use sqlx::{query_file_as, PgPool};
use uuid::Uuid;

use crate::models::{
    definition::Definition,
    service::{CreateService, Service, UpdateService},
};

#[async_trait]
#[mockall::automock]
pub trait ServiceRepo: Send + Sync {
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Service>>;
    async fn create(&self, data: &CreateService) -> Result<Service>;
    async fn get(&self, id: &Uuid) -> Result<Service>;
    async fn get_by_name(&self, name: &str) -> Result<Option<Service>>;
    async fn update(&self, id: &Uuid, date: &UpdateService) -> Result<Service>;
    async fn destroy(&self, id: &Uuid) -> Result<Service>;
    async fn get_default_profile(&self, id: &Uuid) -> Result<Definition>;
}

pub struct PgServiceRepo {
    pool: PgPool,
}

impl PgServiceRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ServiceRepo for PgServiceRepo {
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Service>> {
        let profile = query_file_as!(Service, "sql/services/list.sql", &limit, &offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(profile)
    }

    async fn create(&self, data: &CreateService) -> Result<Service> {
        let service = query_file_as!(
            Service,
            "sql/services/create.sql",
            &data.name,
            &data.redirect_url,
            &data.webhook_url
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(service)
    }

    async fn get(&self, id: &Uuid) -> Result<Service> {
        let profile = query_file_as!(Service, "sql/services/get.sql", &id)
            .fetch_one(&self.pool)
            .await?;

        Ok(profile)
    }

    async fn get_by_name(&self, name: &str) -> Result<Option<Service>> {
        let profile = query_file_as!(Service, "sql/services/get_by_name.sql", &name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(profile)
    }

    async fn update(&self, id: &Uuid, data: &UpdateService) -> Result<Service> {
        let service = self.get(id).await?;

        let name = data.name.as_deref().unwrap_or(&service.name);
        let redirect_url = data.webhook_url.as_deref().unwrap_or(&service.redirect_url);
        let webhook_url = data.webhook_url.as_deref().unwrap_or(&service.webhook_url);

        let profile = query_file_as!(
            Service,
            "sql/services/update.sql",
            &id,
            &name,
            &redirect_url,
            &webhook_url
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(profile)
    }

    async fn destroy(&self, id: &Uuid) -> Result<Service> {
        let profile = query_file_as!(Service, "sql/services/destroy.sql", &id)
            .fetch_one(&self.pool)
            .await?;

        Ok(profile)
    }

    async fn get_default_profile(&self, id: &Uuid) -> Result<Definition> {
        let profile = query_file_as!(Definition, "sql/services/get_default_definition.sql", &id)
            .fetch_one(&self.pool)
            .await?;

        Ok(profile)
    }
}
