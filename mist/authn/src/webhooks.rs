use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::redis::TypedRedisKey;

pub(crate) mod registration;

#[derive(Serialize, Deserialize)]
pub(crate) struct Webhook {
    pub(crate) meta: Meta,
    pub(crate) data: Request,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Meta {
    pub(crate) id: Uuid,
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
                id: Uuid::new_v4(),
                timestamp: Utc::now(),
                kind,
            },
            data,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct HookData {
    pub(crate) session_id: Uuid,
    pub(crate) identifier: String,
}

pub(crate) static HOOK_DATA: TypedRedisKey<HookData> = TypedRedisKey::new("mist-hook");
