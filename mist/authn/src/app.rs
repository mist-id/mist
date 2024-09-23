use std::sync::Arc;

use async_nats::{jetstream::Context, Client};
use axum::Router;
use fred::{
    prelude::{ClientLike, RedisClient},
    types::RedisConfig,
};
use mist_common::{env::Environment, Result};
use mist_db::repos::{
    identifiers::PgIdentifierRepo, keys::PgKeyRepo, services::PgServiceRepo, users::PgUserRepo,
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

    let redis = create_redis_client(&env).await.unwrap();
    let (nats, jetstream) = create_nas_client(&env).await.unwrap();

    Router::new()
        .nest("", handlers::router())
        .with_state(AuthnState {
            env,
            repos,
            redis,
            nats,
            jetstream,
        })
        .layer(CookieManagerLayer::new())
}

async fn create_redis_client(env: &Environment) -> Result<RedisClient> {
    let config = RedisConfig::from_url(&env.redis_url)?;
    let client = RedisClient::new(config, None, None, None);

    client.init().await?;

    Ok(client)
}

async fn create_nas_client(env: &Environment) -> Result<(Client, Context)> {
    let client = async_nats::connect(env.nats_url.clone()).await?;
    let jetstream = async_nats::jetstream::new(client.clone());

    Ok((client, jetstream))
}
