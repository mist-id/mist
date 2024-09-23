mod create;
mod destroy;
mod get;
mod list;
mod preferred;
mod update;
mod value;

use axum::{routing, Router};
use mist_db::models::key::{Key, KeyKind};
use utoipa::OpenApi;

use crate::state::ApiState;

#[derive(OpenApi)]
#[openapi(
    paths(
        list::list_handler,
        create::create_handler,
        get::get_handler,
        update::update_handler,
        destroy::destroy_handler,
        value::value_handler,
        preferred::preferred_handler
    ),
    components(schemas(Key, KeyKind, create::Payload, update::Payload))
)]
pub(crate) struct Api;

pub(crate) fn router() -> Router<ApiState> {
    Router::new()
        .route(
            "/services/:service_id/keys",
            routing::get(list::list_handler),
        )
        .route(
            "/services/:service_id/keys",
            routing::post(create::create_handler),
        )
        .route(
            "/services/:service_id/keys/:id",
            routing::get(get::get_handler),
        )
        .route(
            "/services/:service_id/keys/:id",
            routing::put(update::update_handler),
        )
        .route(
            "/services/:service_id/keys/:id",
            routing::delete(destroy::destroy_handler),
        )
        .route(
            "/services/:service_id/keys/:id/value",
            routing::get(value::value_handler),
        )
        .route(
            "/services/:service_id/keys/preferred",
            routing::get(preferred::preferred_handler),
        )
}
