use std::sync::Arc;

use axum::Router;
use common::env::Environment;
use db::repos::{definitions::PgDefinitionRepo, keys::PgKeyRepo, services::PgServiceRepo};
use sqlx::postgres::PgPoolOptions;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    handlers::{keys, services},
    state::{ApiState, Repos},
};

#[derive(OpenApi)]
#[openapi(
    info(version = "latest", title = "Mist", license(name = "Apache 2.0", url = "http://www.apache.org/licenses/LICENSE-2.0")),
    nest((path = "/services", api = services::Api), (path = "/keys", api = keys::Api)),
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
        definitions: Arc::new(PgDefinitionRepo::new(postgres.clone())),
    };

    let is_development = env.development;

    let mut app = Router::new()
        .nest("", services::router())
        .nest("", keys::router())
        .with_state(ApiState { env, repos });

    if is_development {
        app = app.merge(SwaggerUi::new("/swagger").url("/openapi.json", Api::openapi()));
    }

    app
}
