use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum WebhookKind {
    Registration,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Meta {
    pub(crate) id: Uuid,
    pub(crate) kind: WebhookKind,
    pub(crate) timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RegistrationWebhook {
    pub(crate) meta: Meta,
    pub(crate) data: RegistrationWebhookData,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RegistrationWebhookData {
    pub(crate) id: Uuid,
    pub(crate) identifier: String,
    pub(crate) profile: Map<String, Value>,
}

impl RegistrationWebhook {
    pub fn new(user_id: &Uuid, did: &str, profile: Map<String, Value>) -> Self {
        Self {
            meta: Meta {
                id: Uuid::new_v4(),
                kind: WebhookKind::Registration,
                timestamp: Utc::now(),
            },
            data: RegistrationWebhookData {
                id: *user_id,
                identifier: did.into(),
                profile,
            },
        }
    }
}

pub type RegistrationWebhookResponse = RegistrationWebhook;
