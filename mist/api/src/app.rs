use std::sync::Arc;

use axum::{middleware, Router};
use common::env::Environment;
use db::repos::{keys::PgKeyRepo, services::PgServiceRepo};
use sqlx::postgres::PgPoolOptions;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    handlers::{keys, services},
    middleware::auth,
    state::{ApiState, Repos},
};

#[derive(OpenApi)]
#[openapi(
    info(version = "latest", title = "Mist", license(name = "Apache 2.0", url = "http://www.apache.org/licenses/LICENSE-2.0")),
    nest((path = "/services", api = services::Api), (path = "/services/{service_id}/keys", api = keys::Api)),
    tags((name = "Services"), (name = "Keys"))
)]
struct Api;

pub async fn app(env: Environment) -> Router {
    let postgres = PgPoolOptions::new()
        .max_connections(env.postgres_pool_size)
        .connect(&env.postgres_url)
        .await
        .unwrap();

    let repos = Repos {
        services: Arc::new(PgServiceRepo::new(postgres.clone())),
        keys: Arc::new(PgKeyRepo::new(postgres.clone())),
    };

    let state = ApiState {
        env: env.clone(),
        repos,
    };

    let mut app = Router::new()
        .nest("", services::router())
        .nest("", keys::router())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::middleware,
        ))
        .with_state(state);

    if env.development {
        app = app.merge(SwaggerUi::new("/swagger").url("/openapi.json", Api::openapi()));
    }

    app
}
