use bon::Builder;
use chrono::{DateTime, Utc};
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(
    Default, Clone, Copy, PartialEq, Debug, Display, Serialize, Deserialize, AsRef, From, Into,
)]
pub struct ServiceId(pub Uuid);

impl ServiceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Default, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Service {
    pub id: ServiceId,
    pub name: String,
    pub redirect_url: String,
    pub logout_url: String,
    pub webhook_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Builder)]
pub struct CreateService {
    #[builder(into)]
    pub name: String,
    #[builder(into)]
    pub redirect_url: String,
    #[builder(into)]
    pub logout_url: String,
    #[builder(into)]
    pub webhook_url: String,
}

#[derive(Debug, PartialEq, Builder)]
pub struct UpdateService {
    #[builder(into)]
    pub name: Option<String>,
    #[builder(into)]
    pub redirect_url: Option<String>,
    #[builder(into)]
    pub logout_url: Option<String>,
    #[builder(into)]
    pub webhook_url: Option<String>,
}
