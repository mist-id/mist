use bon::Builder;
use chrono::{DateTime, Utc};
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use utoipa::ToSchema;
use uuid::Uuid;

use super::service::ServiceId;

#[derive(
    Default, Clone, Copy, PartialEq, Debug, Display, Serialize, Deserialize, AsRef, From, Into,
)]
pub struct KeyId(pub Uuid);

impl KeyId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Default, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Key {
    pub id: KeyId,
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
    Api,
    Token,
}

#[derive(Builder, Debug, PartialEq)]
pub struct CreateKey {
    pub kind: KeyKind,
    pub priority: i32,
    pub service_id: ServiceId,
}

#[derive(Builder, Debug, PartialEq)]
pub struct UpdateKey {
    pub is_active: Option<bool>,
}
