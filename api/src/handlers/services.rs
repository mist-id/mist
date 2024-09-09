mod create;
mod destroy;
mod get;
mod list;
mod update;

use axum::{routing, Router};

use crate::state::ApiState;

pub(crate) fn router() -> Router<ApiState> {
    Router::new()
        .route("/services", routing::get(list::handler))
        .route("/services", routing::post(create::handler))
        .route("/services/:id", routing::get(get::handler))
        .route("/services/:id", routing::put(update::handler))
        .route("/services/:id", routing::delete(destroy::handler))
}
