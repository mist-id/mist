use axum::{extract::State, response::IntoResponse, Json};
use common::Result;
use db::models::{identifier::CreateIdentifier, user::CreateUser};
use eyre::eyre;
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
) -> Result<impl IntoResponse> {
    let session_data = state
        .redis_client
        .get::<String, _>(&format!("{REDIS_AUTH_HOOK_KEY}-{}", body.meta.id))
        .await
        .map(|v| serde_json::from_str::<AuthHookSessionData>(&v))??;

    if session_data.hook_id != body.meta.id {
        return Err(eyre!("invalid hook id").into());
    };

    match body.meta.kind {
        WebhookKind::Registration => handle_registration(&state, &session_data, &body).await,
    }
}

async fn handle_registration(
    state: &AuthnState,
    hook_session: &AuthHookSessionData,
    body: &RegistrationWebhookResponse,
) -> Result<impl IntoResponse> {
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
        .create(
            &CreateUser::builder()
                .id(auth_session.user_id)
                .service_id(auth_session.service_id)
                .build(),
        )
        .await?;

    state
        .repos
        .identifiers
        .create(
            &CreateIdentifier::builder()
                .value(body.data.identifier.clone())
                .user_id(auth_session.user_id)
                .build(),
        )
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
