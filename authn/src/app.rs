use std::sync::Arc;

use axum::Router;
use common::{env::Environment, error::Error};
use db::repos::{
    identifiers::PgIdentifierRepo, keys::PgKeyRepo, services::PgServiceRepo, users::PgUserRepo,
};
use fred::prelude::{ClientLike, RedisClient};
use sqlx::postgres::PgPoolOptions;
use tower_cookies::CookieManagerLayer;

use crate::{
    handlers,
    state::{AuthnState, Repos},
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
        users: Arc::new(PgUserRepo::new(postgres.clone())),
        identifiers: Arc::new(PgIdentifierRepo::new(postgres.clone())),
    };

    let redis_client = create_redis_client().await.unwrap();
    let redis_pub_client = create_redis_client().await.unwrap();
    let redis_sub_client = create_redis_client().await.unwrap();

    Router::new()
        .nest("", handlers::router())
        .with_state(AuthnState {
            env,
            repos,
            redis_client,
            redis_pub_client,
            redis_sub_client,
        })
        .layer(CookieManagerLayer::new())
}

async fn create_redis_client() -> Result<RedisClient, Error> {
    let client = RedisClient::default();

    client.init().await?;

    Ok(client)
}
