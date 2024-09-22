use std::sync::Arc;

use async_nats::Client;
use axum::Router;
use common::{env::Environment, Result};
use db::repos::{
    identifiers::PgIdentifierRepo, keys::PgKeyRepo, services::PgServiceRepo, users::PgUserRepo,
};
use fred::{
    prelude::{ClientLike, RedisClient},
    types::RedisConfig,
};
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

    let redis_client = create_redis_client(&env).await.unwrap();
    let nats_client = create_client(&env).await.unwrap();

    Router::new()
        .nest("", handlers::router())
        .with_state(AuthnState {
            env,
            repos,
            redis: redis_client,
            nats: nats_client,
        })
        .layer(CookieManagerLayer::new())
}

async fn create_redis_client(env: &Environment) -> Result<RedisClient> {
    let config = RedisConfig::from_url(&env.redis_url)?;
    let client = RedisClient::new(config, None, None, None);

    client.init().await?;

    Ok(client)
}

async fn create_client(env: &Environment) -> Result<Client> {
    let client = async_nats::connect(env.nats_url.clone()).await?;

    Ok(client)
}
