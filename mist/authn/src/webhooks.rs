use chrono::{DateTime, Utc};
use derive_more::{AsRef, Display, From, FromStr, Into};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{session::SessionId, utils::redis::TypedRedisKey};

pub(crate) mod registration;

#[derive(Default, Debug, Display, Serialize, Deserialize, AsRef, From, FromStr, Into)]
pub struct WebhookId(pub Uuid);

impl WebhookId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Webhook {
    pub(crate) meta: Meta,
    pub(crate) data: Request,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Meta {
    pub(crate) id: WebhookId,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) kind: Kind,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Kind {
    Registration,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Request {
    Registration(registration::Request),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Response {
    Registration(registration::Response),
}

impl Webhook {
    pub fn new(kind: Kind, data: Request) -> Self {
        Self {
            meta: Meta {
                id: WebhookId::new(),
                timestamp: Utc::now(),
                kind,
            },
            data,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct HookData {
    pub(crate) session_id: SessionId,
    pub(crate) identifier: String,
}

pub(crate) static HOOK_DATA: TypedRedisKey<HookData> = TypedRedisKey::new("mist-hook");
