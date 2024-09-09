use std::{convert::Infallible, sync::Arc};

use anyhow::Context;
use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
};
use common::error::Error;
use fred::prelude::*;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tower_cookies::Cookies;

use crate::{state::AuthnState, COOKIE_KEY, REDIS_RESPONSE_RECEIVED_KEY};

pub(crate) async fn handler(
    cookies: Cookies,
    State(state): State<AuthnState>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, Error> {
    let cookie = cookies.get(COOKIE_KEY).context("no cookie")?;

    let (tx, rx) = mpsc::channel(100);
    let tx_arc = Arc::new(tx);

    // Subscribe to the response channel keyed by the cookie value (the user session ID).
    state
        .redis_sub_client
        .subscribe(format!(
            "{REDIS_RESPONSE_RECEIVED_KEY}-{uuid}",
            uuid = cookie.value()
        ))
        .await?;

    state.redis_sub_client.on_message(move |_| {
        let tx_clone = Arc::clone(&tx_arc);

        tokio::spawn(async move {
            let event = Event::default().data("ready");

            // When a mesasge is received, we send a server-sent event to the user's
            // browser, which will trigger the client to redirect to the services
            // callback URL.
            if tx_clone.send(Ok(event)).await.is_err() {
                tracing::error!("failed to send message to chanenl");
            }
        });

        Ok(())
    });

    let stream = ReceiverStream::new(rx);

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}
