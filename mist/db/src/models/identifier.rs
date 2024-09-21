use bon::Builder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct Identifier {
    pub id: Uuid,
    pub value: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Builder)]
pub struct CreateIdentifier {
    #[builder(into)]
    pub value: String,
    #[builder(into)]
    pub user_id: Uuid,
}
