use std::time::Duration;

use async_nats::jetstream::{
    consumer,
    stream::{Config, RetentionPolicy},
};
use common::{env::Environment, error::Result};
use futures::StreamExt;
use tokio::task::JoinHandle;

use crate::jobs::webhooks::{Webhook, CONSUMER_NAME, STREAM_NAME};

pub async fn run(env: &Environment) -> Result<JoinHandle<()>> {
    let client = async_nats::connect(env.nats_url.clone()).await?;
    let jetstream = async_nats::jetstream::new(client);

    let webhook_stream = match jetstream.get_stream(&STREAM_NAME).await {
        Ok(stream) => stream,
        Err(_) => {
            jetstream
                .create_stream(Config {
                    name: STREAM_NAME.into(),
                    retention: RetentionPolicy::WorkQueue,
                    subjects: vec![format!("{STREAM_NAME}.>")],
                    ..Default::default()
                })
                .await?
        }
    };

    let webhook_consumer = match webhook_stream.get_consumer(CONSUMER_NAME).await {
        Ok(consumer) => consumer,
        Err(_) => {
            webhook_stream
                .create_consumer(consumer::pull::Config {
                    durable_name: Some(CONSUMER_NAME.into()),
                    ack_wait: Duration::from_secs(30),
                    max_deliver: 5,
                    ..Default::default()
                })
                .await?
        }
    };

    let handle = tokio::spawn(async move {
        loop {
            let Ok(mut messages) = webhook_consumer.fetch().messages().await else {
                tracing::error!("failed to fetch messages");

                continue;
            };

            while let Some(Ok(message)) = messages.next().await {
                match serde_json::from_slice::<Webhook>(&message.payload) {
                    Ok(webhook) => {
                        if let Err(e) = reqwest::Client::new()
                            .post(&webhook.url)
                            .json(&webhook.payload)
                            .send()
                            .await
                        {
                            tracing::error!("failed to send webhook: {:?}", e);

                            continue;
                        }
                    }
                    Err(e) => {
                        tracing::error!("failed to deserialize payload: {:?}", e);

                        continue;
                    }
                }

                if let Err(e) = message.ack().await {
                    tracing::error!("failed to ack message: {:?}", e);
                }
            }
        }
    });

    Ok(handle)
}
