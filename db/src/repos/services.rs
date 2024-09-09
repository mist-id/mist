use async_trait::async_trait;
use common::error::Error;
use sqlx::{query_file_as, PgPool};
use uuid::Uuid;

use crate::models::{
    definition::Definition,
    service::{CreateService, Service, UpdateService},
};

#[async_trait]
#[mockall::automock]
pub trait ServiceRepo: Send + Sync {
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Service>, Error>;
    async fn create(&self, data: &CreateService) -> Result<Service, Error>;
    async fn get(&self, id: &Uuid) -> Result<Option<Service>, Error>;
    async fn get_by_name(&self, name: &str) -> Result<Option<Service>, Error>;
    async fn update(&self, id: &Uuid, date: &UpdateService) -> Result<Service, Error>;
    async fn destroy(&self, id: &Uuid) -> Result<Service, Error>;
    async fn get_default_profile(&self, id: &Uuid) -> Result<Definition, Error>;
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
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Service>, Error> {
        let profile = query_file_as!(Service, "sql/services/list.sql", &limit, &offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(profile)
    }

    async fn create(&self, data: &CreateService) -> Result<Service, Error> {
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

    async fn get(&self, id: &Uuid) -> Result<Option<Service>, Error> {
        let profile = query_file_as!(Service, "sql/services/get.sql", &id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(profile)
    }

    async fn get_by_name(&self, name: &str) -> Result<Option<Service>, Error> {
        let profile = query_file_as!(Service, "sql/services/get_by_name.sql", &name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(profile)
    }

    async fn update(&self, id: &Uuid, data: &UpdateService) -> Result<Service, Error> {
        let service = self.get(id).await?.ok_or(sqlx::Error::RowNotFound)?;

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

    async fn destroy(&self, id: &Uuid) -> Result<Service, Error> {
        let profile = query_file_as!(Service, "sql/services/destroy.sql", &id)
            .fetch_one(&self.pool)
            .await?;

        Ok(profile)
    }

    async fn get_default_profile(&self, id: &Uuid) -> Result<Definition, Error> {
        let profile = query_file_as!(Definition, "sql/services/get_default_definition.sql", &id)
            .fetch_one(&self.pool)
            .await?;

        Ok(profile)
    }
}
