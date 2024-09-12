use bon::Builder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub service_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Builder)]
pub struct CreateUser {
    #[builder(into)]
    pub id: Uuid,
    #[builder(into)]
    pub service_id: Uuid,
}
