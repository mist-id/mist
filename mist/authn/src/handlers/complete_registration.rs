use axum::{extract::State, response::IntoResponse, Json};
use common::Result;
use db::models::{identifier::CreateIdentifier, user::CreateUser};
use eyre::eyre;
use fred::prelude::*;
use http::StatusCode;
use serde::Deserialize;

use crate::{
    events::{get_event_key, Event},
    session::{AuthSession, AuthState, SessionId, AUTH_SESSION},
    state::AuthnState,
};

#[derive(Deserialize)]
pub(crate) struct Payload {
    session_id: SessionId,
}

pub(crate) async fn handler(
    State(state): State<AuthnState>,
    Json(payload): Json<Payload>,
) -> Result<impl IntoResponse> {
    // Get session information.
    // ------------------------

    let session = AUTH_SESSION
        .get(&state.redis, &payload.session_id.to_string())
        .await?;

    let AuthState::Registering { identifier } = &session.state else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    // Create the user and associated DID if they don't already exist.
    // ---------------------------------------------------------------

    let authenticating_session = AUTH_SESSION
        .get(&state.redis, &payload.session_id.to_string())
        .await?;

    let existing = state.repos.identifiers.get_by_value(identifier).await;

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
                .value(identifier.clone())
                .user_id(authenticating_session.user_id)
                .build(),
        )
        .await?;

    // Complete the registration process.
    // ----------------------------------

    // Update the user session.
    AUTH_SESSION
        .set(
            &state.redis,
            &payload.session_id.to_string(),
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
            get_event_key(&Event::Redirect, &payload.session_id.to_string()),
            "".into(),
        )
        .await?;

    Ok(StatusCode::OK.into_response())
}
