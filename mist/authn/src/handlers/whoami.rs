use axum::{extract::State, response::IntoResponse};
use common::Result;
use eyre::OptionExt;
use http::StatusCode;
use serde::Serialize;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    session::{AuthState, AUTH_SESSION, COOKIE_KEY},
    state::AuthnState,
};

#[derive(Serialize)]
pub(crate) struct Response {
    id: Uuid,
    identifier: String,
}

pub(crate) async fn handler(
    cookies: Cookies,
    State(state): State<AuthnState>,
) -> Result<impl IntoResponse> {
    let cookie = cookies.get(COOKIE_KEY).ok_or_eyre("no cookie")?;

    let Ok(session) = AUTH_SESSION.get(&state.redis, cookie.value()).await else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    let AuthState::Authenticated { identifier_id } = session.state else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    let user = state.repos.users.get(&session.user_id).await?;
    let identifier = state.repos.identifiers.get(&identifier_id).await?;

    Ok(serde_json::to_string(&Response {
        id: user.id,
        identifier: identifier.value,
    })?
    .into_response())
}
