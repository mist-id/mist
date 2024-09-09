mod create;
mod destroy;
mod get;
mod list;
mod preferred;
mod update;

use axum::{routing, Router};

use crate::state::ApiState;

pub(crate) fn router() -> Router<ApiState> {
    Router::new()
        .route("/services/:service_id/keys", routing::get(list::handler))
        .route("/services/:service_id/keys", routing::post(create::handler))
        .route("/keys/:id", routing::get(get::handler))
        .route("/keys/:id", routing::put(update::handler))
        .route("/keys/:id", routing::delete(destroy::handler))
        .route(
            "/services/:service_id/keys/preferred",
            routing::get(preferred::handler),
        )
}
