use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::redis::TypedRedisKey;

pub(crate) const COOKIE_KEY: &str = "mist";

#[derive(Serialize, Deserialize)]
pub(crate) struct AuthSession {
    pub(crate) user_id: Uuid,
    pub(crate) service_id: Uuid,
    pub(crate) state: AuthState,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum AuthState {
    Authenticating { action: AuthAction },
    Authenticated { identifier_id: Uuid },
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum AuthAction {
    Up,
    In,
}

pub(crate) static AUTH_SESSION: TypedRedisKey<AuthSession> = TypedRedisKey::new("mist-auth");
