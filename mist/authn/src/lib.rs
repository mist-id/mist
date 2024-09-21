use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod app;
mod crypto;
mod handlers;
mod state;
mod webhooks;

const COOKIE_KEY: &str = "id";
const REDIS_AUTH_KEY: &str = "auth";
const REDIS_AUTH_HOOK_KEY: &str = "auth-hook";
const REDIS_RESPONSE_RECEIVED_KEY: &str = "response-received";

#[derive(Serialize, Deserialize)]
pub(crate) struct AuthSessionData {
    pub(crate) service_id: Uuid,
    pub(crate) user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct AuthHookSessionData {
    pub(crate) hook_id: Uuid,
    pub(crate) user_session_id: Uuid,
}
