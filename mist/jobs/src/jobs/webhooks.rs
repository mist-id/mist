use async_nats::jetstream::{publish::PublishAck, Context};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use mist_common::error::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const STREAM_NAME: &str = "jobs-webhooks";
pub const CONSUMER_NAME: &str = "jobs-webhooks-consumer";

#[derive(Serialize, Deserialize)]
pub struct Webhook {
    pub(crate) url: String,
    pub(crate) payload: String,
}

impl Webhook {
    pub async fn publish<T: Serialize>(
        jetstream: &Context,
        url: &str,
        kind: &str,
        data: &T,
    ) -> Result<PublishAck, Error> {
        let payload = Payload::new(kind, data);

        let published = jetstream
            .publish(
                format!("{STREAM_NAME}.{}", kind),
                Webhook {
                    url: url.into(),
                    payload: serde_json::to_string(&payload)?,
                }
                .try_into()?,
            )
            .await?
            .await?;

        Ok(published)
    }
}

impl TryInto<Bytes> for Webhook {
    type Error = Error;

    fn try_into(self) -> Result<Bytes, Self::Error> {
        Ok(serde_json::to_string(&self)?.into())
    }
}

#[derive(Serialize)]
pub struct Payload<T: Serialize> {
    pub meta: Meta,
    pub data: T,
}

#[derive(Serialize)]
pub struct Meta {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub kind: String,
}

impl<T: Serialize> Payload<T> {
    fn new(kind: impl ToString, data: T) -> Self {
        Self {
            meta: Meta {
                id: Uuid::new_v4(),
                timestamp: Utc::now(),
                kind: kind.to_string(),
            },
            data,
        }
    }
}
