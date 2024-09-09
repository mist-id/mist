mod create;
mod hook;
mod verify;
mod wait;
mod who;

use axum::{routing, Router};

use crate::state::AuthnState;

pub(crate) fn router() -> Router<AuthnState> {
    Router::new()
        .route("/:service_name", routing::get(create::handler))
        .route("/verify", routing::post(verify::handler))
        .route("/hook", routing::post(hook::handler))
        .route("/waiting", routing::get(wait::handler))
        .route("/who", routing::get(who::handler))
}
