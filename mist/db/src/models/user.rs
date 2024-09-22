use bon::Builder;
use chrono::{DateTime, Utc};
use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use uuid::Uuid;

use super::service::ServiceId;

#[derive(
    Default, Clone, Copy, PartialEq, Debug, Display, Serialize, Deserialize, AsRef, From, Into,
)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: UserId,
    pub service_id: ServiceId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Builder)]
pub struct CreateUser {
    #[builder(into)]
    pub id: UserId,
    #[builder(into)]
    pub service_id: Uuid,
}
