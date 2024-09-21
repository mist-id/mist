use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
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
    let mut cookie = cookies.get(COOKIE_KEY).ok_or_eyre("no cookie")?;
    let key = format!("{REDIS_AUTH_KEY}-{}", cookie.value());

    let session = state
        .redis_client
        .get::<String, _>(&key)
        .await
        .map(|v| serde_json::from_str::<AuthSessionData>(&v))??;

    let service = state.repos.services.get(&session.service_id).await?;

    cookie.make_removal();
    state.redis_client.del(&key).await?;

    Ok(Redirect::to(&service.logout_url))
}
