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

#[derive(derive_new::new)]
pub struct CreateUser {
    #[new(into)]
    pub id: Uuid,
    #[new(into)]
    pub service_id: Uuid,
}
