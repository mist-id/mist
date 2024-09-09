use std::sync::Arc;

use axum::Router;
use common::env::Environment;
use db::repos::{definitions::PgDefinitionRepo, keys::PgKeyRepo, services::PgServiceRepo};
use sqlx::postgres::PgPoolOptions;

use crate::{
    handlers::{keys, services},
    state::{ApiState, Repos},
};

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

    Router::new()
        .nest("/api/v1", services::router())
        .nest("/api/v1", keys::router())
        .with_state(ApiState { env, repos })
}
