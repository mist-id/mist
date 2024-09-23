use common::redis::TypedRedis;
use db::models::{identifier::IdentifierId, service::ServiceId, user::UserId};
use derive_more::{AsRef, Display, From, FromStr, Into};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) const COOKIE_KEY: &str = "mist";

#[derive(Default, Debug, Display, Serialize, Deserialize, AsRef, From, FromStr, Into)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct AuthSession {
    pub(crate) user_id: UserId,
    pub(crate) service_id: ServiceId,
    pub(crate) state: AuthState,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum AuthState {
    Authenticating { action: AuthAction },
    Registering { identifier: String },
    Authenticated { identifier_id: IdentifierId },
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum AuthAction {
    Up,
    In,
}

pub(crate) static AUTH_SESSION: TypedRedis<AuthSession> = TypedRedis::new("mist-auth");
