use axum::{extract::State, response::IntoResponse, Json};
use common::error::Error;
use db::models::{identifier::CreateIdentifier, user::CreateUser};
use fred::prelude::*;
use http::StatusCode;

use crate::{
    state::AuthnState,
    webhooks::{RegistrationWebhookResponse, WebhookKind},
    AuthHookSessionData, AuthSessionData, REDIS_AUTH_HOOK_KEY, REDIS_AUTH_KEY,
    REDIS_RESPONSE_RECEIVED_KEY,
};

pub(crate) async fn handler(
    State(state): State<AuthnState>,
    Json(body): Json<RegistrationWebhookResponse>,
) -> Result<impl IntoResponse, Error> {
    let session_data = state
        .redis_client
        .get::<String, _>(&format!("{REDIS_AUTH_HOOK_KEY}-{}", body.meta.id))
        .await?;

    let session_data = serde_json::from_str::<AuthHookSessionData>(&session_data)?;

    if session_data.hook_id != body.meta.id {
        return Err(anyhow::anyhow!("invalid hook id").into());
    }

    match body.meta.kind {
        WebhookKind::Registration => handle_registration(&state, &session_data, &body).await,
    }
}

async fn handle_registration(
    state: &AuthnState,
    hook_session: &AuthHookSessionData,
    body: &RegistrationWebhookResponse,
) -> Result<impl IntoResponse, Error> {
    // Get the user's session data.
    // ---------------------------

    let auth_session = state
        .redis_client
        .get::<String, _>(&format!(
            "{REDIS_AUTH_KEY}-{}",
            hook_session.user_session_id
        ))
        .await?;

    let auth_session = serde_json::from_str::<AuthSessionData>(&auth_session)?;

    // Create the user and associated DID.
    // -----------------------------------

    state
        .repos
        .users
        .create(&CreateUser::new(
            auth_session.user_id,
            auth_session.service_id,
        ))
        .await?;

    state
        .repos
        .identifiers
        .create(&CreateIdentifier::new(
            body.data.identifier.clone(),
            auth_session.user_id,
        ))
        .await?;

    // Complete the registration process.
    // ----------------------------------

    // Send an event to the user's browser to let it know authentication is complete.
    //
    // This event will be picked up by an event listener in the browser listening to the `wait` handler.
    state
        .redis_pub_client
        .publish(
            format!(
                "{REDIS_RESPONSE_RECEIVED_KEY}-{}",
                hook_session.user_session_id
            ),
            "...",
        )
        .await?;

    // Delete hook session so it can't be reused.
    state
        .redis_client
        .del(&format!("{REDIS_AUTH_HOOK_KEY}-{}", body.meta.id))
        .await?;

    Ok(StatusCode::OK.into_response())
}
