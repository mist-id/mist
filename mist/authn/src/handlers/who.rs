use axum::{extract::State, response::IntoResponse};
use common::Result;
use eyre::OptionExt;
use fred::prelude::*;
use http::StatusCode;
use tower_cookies::Cookies;

use crate::{state::AuthnState, AuthSessionData, COOKIE_KEY, REDIS_AUTH_KEY};

pub(crate) async fn handler(
    cookies: Cookies,
    State(state): State<AuthnState>,
) -> Result<impl IntoResponse> {
    match handle_request(cookies, state).await {
        Ok(response) => Ok(response.into_response()),
        Err(_) => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

async fn handle_request(cookies: Cookies, state: AuthnState) -> Result<impl IntoResponse> {
    let cookie = cookies.get(COOKIE_KEY).ok_or_eyre("no cookie")?;
    let session = state
        .redis_client
        .get::<String, _>(&format!("{REDIS_AUTH_KEY}-{}", cookie.value()))
        .await
        .map(|v| serde_json::from_str::<AuthSessionData>(&v))??;

    let user = state
        .repos
        .users
        .get(&session.user_id)
        .await?
        .ok_or_eyre("can't find user")?;

    Ok(serde_json::to_string(&user)?)
}
