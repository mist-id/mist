use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Json,
};
use common::Result;
use db::models::{identifier::CreateIdentifier, user::CreateUser};
use eyre::{eyre, OptionExt};
use fred::prelude::*;
use http::StatusCode;
use tower_cookies::Cookies;

use crate::{
    events::{get_event_key, Event},
    session::{AuthSession, AuthState, AUTH_SESSION, COOKIE_KEY},
    state::AuthnState,
    webhooks::{self, HOOK_DATA},
};

pub(crate) async fn handler(
    cookies: Cookies,
    State(state): State<AuthnState>,
    Json(body): Json<webhooks::Response>,
) -> Result<impl IntoResponse> {
    match body {
        webhooks::Response::Registration(body) => {
            handle_registration(&cookies, &state, &body).await
        }
    }
}

async fn handle_registration(
    cookies: &Cookies,
    state: &AuthnState,
    body: &webhooks::registration::Response,
) -> Result<impl IntoResponse> {
    let hook_session = HOOK_DATA
        .get(&state.redis, &body.meta.id.to_string())
        .await?;

    // If we shouldn't complete registration, clean up and redirect the user.
    // ---------------------------------------------------------------------

    if !body.complete {
        let mut cookie = cookies.get(COOKIE_KEY).ok_or_eyre("no cookie")?;

        // Clear the cookie.
        cookie.make_removal();

        // Delete hook data.
        HOOK_DATA
            .del(&state.redis, &body.meta.id.to_string())
            .await?;

        // Get the authenticating session to use later.
        let authenticating_session = AUTH_SESSION.get(&state.redis, cookie.value()).await?;

        // Delete the authenticating session.
        AUTH_SESSION.del(&state.redis, cookie.value()).await?;

        // Redirect the user to the service's logout URL.
        let service = state
            .repos
            .services
            .get(&authenticating_session.service_id)
            .await?;

        return Ok(Redirect::to(&service.logout_url).into_response());
    }

    // Get the user's session data.
    // ---------------------------

    let authenticating_session = AUTH_SESSION
        .get(&state.redis, &hook_session.session_id.to_string())
        .await?;

    // Create the user and associated DID if they don't already exist.
    // ---------------------------------------------------------------

    let existing = state
        .repos
        .identifiers
        .get_by_value(&hook_session.identifier)
        .await;

    if existing.is_ok() {
        return Err(eyre!("user already exists").into());
    }

    let user = state
        .repos
        .users
        .create(
            &CreateUser::builder()
                .id(authenticating_session.user_id)
                .service_id(authenticating_session.service_id)
                .build(),
        )
        .await?;

    let identifier = state
        .repos
        .identifiers
        .create(
            &CreateIdentifier::builder()
                .value(hook_session.identifier.clone())
                .user_id(authenticating_session.user_id)
                .build(),
        )
        .await?;

    // Complete the registration process.
    // ----------------------------------

    // Delete hook data now that we're done with it.
    HOOK_DATA
        .del(&state.redis, &body.meta.id.to_string())
        .await?;

    // Update the user session.
    AUTH_SESSION
        .set(
            &state.redis,
            &hook_session.session_id.to_string(),
            &AuthSession {
                service_id: authenticating_session.service_id,
                user_id: user.id,
                state: AuthState::Authenticated {
                    identifier_id: identifier.id,
                },
            },
            Expiration::EX(60 * 60 * 8),
        )
        .await?;

    // Send an event to the user's browser to let it know authentication is complete.
    //
    // This event will be picked up by an event listener in the browser listening to
    // the `waiting_for_completion` handler.
    state
        .nats
        .publish(
            get_event_key(&Event::Redirect, &hook_session.session_id.to_string()),
            "".into(),
        )
        .await?;

    Ok(StatusCode::OK.into_response())
}
