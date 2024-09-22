use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use common::Result;
use eyre::OptionExt;
use tower_cookies::Cookies;

use crate::{
    session::{AUTH_SESSION, COOKIE_KEY},
    state::AuthnState,
};

pub(crate) async fn handler(
    cookies: Cookies,
    State(state): State<AuthnState>,
) -> Result<impl IntoResponse> {
    let mut cookie = cookies.get(COOKIE_KEY).ok_or_eyre("no cookie")?;
    let session_id = cookie.value().to_string();

    cookie.make_removal();

    let session = AUTH_SESSION.get(&state.redis, &session_id).await?;
    let service = state.repos.services.get(&session.service_id).await?;

    AUTH_SESSION.del(&state.redis, &session_id).await?;

    Ok(Redirect::to(&service.logout_url))
}
