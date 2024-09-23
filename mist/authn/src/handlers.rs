mod complete_registration;
mod kill_session;
mod start_auth;
mod verify_response;
mod wait_for_completion;
mod whoami;

use axum::{routing, Router};

use crate::state::AuthnState;

pub(crate) fn router() -> Router<AuthnState> {
    Router::new()
        .route("/:service_name/:action", routing::get(start_auth::handler))
        .route("/:service_name/out", routing::post(kill_session::handler))
        .route("/waiting", routing::get(wait_for_completion::handler))
        .route("/auth", routing::post(verify_response::handler))
        .route("/complete", routing::post(complete_registration::handler))
        .route("/whoami", routing::get(whoami::handler))
}
