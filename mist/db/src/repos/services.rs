use async_trait::async_trait;
use mist_common::{
    crypto::{create_service_key, encrypt_service_key},
    Result,
};
use secstr::SecVec;
use sqlx::{query_file_as, PgPool};

use crate::models::{
    definition::{CreateDefinition, Definition},
    key::{Key, KeyKind},
    service::{CreateService, Service, ServiceId, UpdateService},
};

#[async_trait]
#[mockall::automock]
pub trait ServiceRepo: Send + Sync {
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Service>>;
    async fn create(
        &self,
        master_key: &SecVec<u8>,
        service: &CreateService,
        definition: &Option<CreateDefinition>,
    ) -> Result<Service>;
    async fn get(&self, id: &ServiceId) -> Result<Service>;
    async fn get_by_name(&self, name: &str) -> Result<Service>;
    async fn update(&self, id: &ServiceId, date: &UpdateService) -> Result<Service>;
    async fn destroy(&self, id: &ServiceId) -> Result<Service>;
    async fn get_default_profile(&self, id: &ServiceId) -> Result<Option<Definition>>;
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

    async fn create(
        &self,
        master_key: &SecVec<u8>,
        service: &CreateService,
        definition: &Option<CreateDefinition>,
    ) -> Result<Service> {
        let mut tx = self.pool.begin().await?;

        // Create the service.
        // -------------------

        let service = query_file_as!(
            Service,
            "sql/services/create.sql",
            &service.name,
            &service.redirect_url,
            &service.logout_url,
            &service.webhook_url
        )
        .fetch_one(&mut *tx)
        .await?;

        // Create the API key.
        // -------------------

        let key = create_service_key();
        let key_encrypted = encrypt_service_key(master_key, &key)?;

        query_file_as!(
            Key,
            "sql/keys/create.sql",
            service.id.as_ref(),
            KeyKind::Api as KeyKind,
            key_encrypted,
            1
        )
        .fetch_one(&mut *tx)
        .await?;

        // Create the signing key.
        // -----------------------

        let key = create_service_key();
        let key_encrypted = encrypt_service_key(master_key, &key)?;

        query_file_as!(
            Key,
            "sql/keys/create.sql",
            service.id.as_ref(),
            KeyKind::Token as KeyKind,
            key_encrypted,
            1
        )
        .fetch_one(&mut *tx)
        .await?;

        // Create the default definition.
        // ------------------------------

        if let Some(definition) = definition {
            query_file_as!(
                Definition,
                "sql/definitions/create.sql",
                service.id.as_ref(),
                definition.name,
                serde_json::to_value(&definition.value)?,
                definition.is_default
            )
            .fetch_one(&mut *tx)
            .await?;
        }

        // All done; commit the transaction.
        tx.commit().await?;

        Ok(service)
    }

    async fn get(&self, id: &ServiceId) -> Result<Service> {
        let profile = query_file_as!(Service, "sql/services/get.sql", &id.as_ref())
            .fetch_one(&self.pool)
            .await?;

        Ok(profile)
    }

    async fn get_by_name(&self, name: &str) -> Result<Service> {
        let profile = query_file_as!(Service, "sql/services/get_by_name.sql", &name)
            .fetch_one(&self.pool)
            .await?;

        Ok(profile)
    }

    async fn update(&self, id: &ServiceId, data: &UpdateService) -> Result<Service> {
        let service = self.get(id).await?;

        let name = data.name.as_deref().unwrap_or(&service.name);
        let redirect_url = data
            .redirect_url
            .as_deref()
            .unwrap_or(&service.redirect_url);
        let logout_url = data.logout_url.as_deref().unwrap_or(&service.logout_url);
        let webhook_url = data.webhook_url.as_deref().unwrap_or(&service.webhook_url);

        let profile = query_file_as!(
            Service,
            "sql/services/update.sql",
            &id.as_ref(),
            &name,
            &redirect_url,
            &logout_url,
            &webhook_url
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(profile)
    }

    async fn destroy(&self, id: &ServiceId) -> Result<Service> {
        let profile = query_file_as!(Service, "sql/services/destroy.sql", &id.as_ref())
            .fetch_one(&self.pool)
            .await?;

        Ok(profile)
    }

    async fn get_default_profile(&self, id: &ServiceId) -> Result<Option<Definition>> {
        let profile = query_file_as!(
            Definition,
            "sql/services/get_default_definition.sql",
            &id.as_ref()
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(profile)
    }
}
