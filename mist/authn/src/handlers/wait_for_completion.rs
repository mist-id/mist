use std::{convert::Infallible, sync::Arc};

use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
};
use common::Result;
use eyre::OptionExt;
use fred::prelude::*;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tower_cookies::Cookies;

use crate::{session::COOKIE_KEY, state::AuthnState, utils::redis::REDIRECT};

pub(crate) async fn handler(
    cookies: Cookies,
    State(state): State<AuthnState>,
) -> Result<Sse<impl Stream<Item = std::result::Result<Event, Infallible>>>> {
    let cookie = cookies.get(COOKIE_KEY).ok_or_eyre("no cookie")?;

    let (tx, rx) = mpsc::channel(100);
    let tx_arc = Arc::new(tx);

    // Subscribe to the response channel keyed by the cookie value (the user session ID).
    state
        .redis_sub
        .subscribe(REDIRECT.key(cookie.value()))
        .await?;

    state.redis_sub.on_message(move |_| {
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
