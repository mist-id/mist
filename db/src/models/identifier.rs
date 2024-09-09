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

#[derive(derive_new::new)]
pub struct CreateIdentifier {
    #[new(into)]
    pub value: String,
    #[new(into)]
    pub user_id: Uuid,
}
