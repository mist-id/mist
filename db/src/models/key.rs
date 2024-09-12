use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Key {
    pub id: Uuid,
    pub kind: KeyKind,
    #[serde(skip_serializing)]
    pub value: Vec<u8>,
    pub priority: i32,
    pub is_active: bool,
    pub service_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "key_kind", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum KeyKind {
    #[default]
    Token,
}

#[derive(Debug, PartialEq, derive_new::new)]
pub struct CreateKey {
    pub kind: KeyKind,
    #[new(into)]
    pub value: String,
    pub priority: i32,
    pub service_id: Uuid,
}

#[derive(Debug, PartialEq, derive_new::new)]
pub struct UpdateKey {
    pub is_active: Option<bool>,
}
