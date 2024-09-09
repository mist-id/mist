use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct Service {
    pub id: Uuid,
    pub name: String,
    pub redirect_url: String,
    pub webhook_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, derive_new::new)]
pub struct CreateService {
    #[new(into)]
    pub name: String,
    #[new(into)]
    pub redirect_url: String,
    #[new(into)]
    pub webhook_url: String,
}

#[derive(Debug, PartialEq, derive_new::new)]
pub struct UpdateService {
    #[new(into)]
    pub name: Option<String>,
    #[new(into)]
    pub redirect_url: Option<String>,
    #[new(into)]
    pub webhook_url: Option<String>,
}
