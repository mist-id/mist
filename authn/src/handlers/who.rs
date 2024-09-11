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
    let cookie = cookies.get(COOKIE_KEY).ok_or_eyre("no cookie")?;

    let session = state
        .redis_client
        .get::<String, _>(&format!("{REDIS_AUTH_KEY}-{}", cookie.value()))
        .await?;

    let session = serde_json::from_str::<AuthSessionData>(&session)?;

    Ok(match state.repos.users.get(&session.user_id).await? {
        Some(user) => serde_json::to_string(&user)?.into_response(),
        None => StatusCode::UNAUTHORIZED.into_response(),
    })
}
