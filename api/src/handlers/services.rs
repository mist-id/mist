mod create;
mod destroy;
mod get;
mod list;
mod update;

use axum::{routing, Router};
use db::models::service::Service;
use utoipa::OpenApi;

use crate::state::ApiState;

#[derive(OpenApi)]
#[openapi(
    paths(
        list::list_handler,
        create::create_handler,
        get::get_handler,
        update::update_handler,
        destroy::destroy_handler
    ),
    components(schemas(Service, create::Payload, update::Payload))
)]
pub(crate) struct Api;

pub(crate) fn router() -> Router<ApiState> {
    Router::new()
        .route("/services", routing::get(list::list_handler))
        .route("/services", routing::post(create::create_handler))
        .route("/services/:id", routing::get(get::get_handler))
        .route("/services/:id", routing::put(update::update_handler))
        .route("/services/:id", routing::delete(destroy::destroy_handler))
}
