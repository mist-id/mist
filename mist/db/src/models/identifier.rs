use bon::Builder;
use chrono::{DateTime, Utc};
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use uuid::Uuid;

use super::user::UserId;

#[derive(
    Default, Clone, Copy, PartialEq, Debug, Display, Serialize, Deserialize, AsRef, From, Into,
)]
pub struct IdentifierId(pub Uuid);

impl IdentifierId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct Identifier {
    pub id: IdentifierId,
    pub value: String,
    pub user_id: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Builder)]
pub struct CreateIdentifier {
    #[builder(into)]
    pub value: String,
    #[builder(into)]
    pub user_id: UserId,
}
