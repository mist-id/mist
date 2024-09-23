use std::{convert::Infallible, sync::Arc};

use axum::{
    extract::State,
    response::{
        sse::{Event as ServerEvent, KeepAlive},
        Sse,
    },
};
use eyre::OptionExt;
use mist_common::Result;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tower_cookies::Cookies;

use crate::{
    events::{get_event_key, Event},
    session::COOKIE_KEY,
    state::AuthnState,
};

pub(crate) async fn handler(
    cookies: Cookies,
    State(state): State<AuthnState>,
) -> Result<Sse<impl Stream<Item = std::result::Result<ServerEvent, Infallible>>>> {
    let cookie = cookies.get(COOKIE_KEY).ok_or_eyre("no cookie")?;

    let (tx, rx) = mpsc::channel(100);
    let tx_arc = Arc::new(tx);

    let mut subscription = state
        .nats
        .subscribe(get_event_key(&Event::Redirect, cookie.value()))
        .await?;

    tokio::spawn(async move {
        while (subscription.next().await).is_some() {
            let tx_clone = Arc::clone(&tx_arc);

            // When a message is received, we send a server-sent event to the user's
            // browser, which will trigger the client to redirect to the services
            // callback URL.
            tokio::spawn(async move {
                let event = ServerEvent::default().data("ready");

                if tx_clone.send(Ok(event)).await.is_err() {
                    tracing::error!("failed to send message to channel");
                }
            });
        }
    });

    let stream = ReceiverStream::new(rx);

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}
