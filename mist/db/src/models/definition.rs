use bon::Builder;
use chrono::{DateTime, Utc};
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::*, types::Json};
use uuid::Uuid;

use super::service::ServiceId;

#[derive(
    Default, Clone, Copy, PartialEq, Debug, Display, Serialize, Deserialize, AsRef, From, Into,
)]
pub struct DefinitionId(pub Uuid);

impl DefinitionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct Definition {
    pub id: DefinitionId,
    pub name: String,
    pub value: Json<Value>,
    pub is_default: bool,
    pub service_id: ServiceId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Value {
    pub fields: Vec<Field>,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub required: bool,
}

#[derive(Builder, Debug, PartialEq)]
pub struct CreateDefinition {
    #[builder(into)]
    pub name: String,
    pub value: Value,
    pub is_default: bool,
}
