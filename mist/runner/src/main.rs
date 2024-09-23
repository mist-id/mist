use std::net::SocketAddr;

use common::{env::Environment, Result};
use jobs::runners::webhooks;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let env = envy::from_env::<Environment>().unwrap();

    migrate(&env).await?;

    let api_env = env.clone();
    let api = tokio::spawn(async {
        let address = SocketAddr::from(([0, 0, 0, 0], 9001));
        let listener = TcpListener::bind(&address).await.unwrap();

        axum::serve(
            listener,
            api::app::app(api_env)
                .await
                .layer(TraceLayer::new_for_http()),
        )
        .await
        .unwrap();
    });

    let authn_env = env.clone();
    let authn = tokio::spawn(async {
        let address = SocketAddr::from(([0, 0, 0, 0], 9002));
        let listener = TcpListener::bind(&address).await.unwrap();

        axum::serve(
            listener,
            authn::app::app(authn_env)
                .await
                .layer(TraceLayer::new_for_http()),
        )
        .await
        .unwrap();
    });

    tokio::select! {
        _ = api => tracing::info!("Api complete"),
        _ = authn => tracing::info!("Authn complete"),
        _ = webhooks::run(&env).await.unwrap() => tracing::info!("Webhooks job complete"),
    }

    Ok(())
}

async fn migrate(env: &Environment) -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(env.postgres_pool_size)
        .connect(&env.postgres_url)
        .await?;

    sqlx::migrate!("../db/migrations").run(&pool).await?;

    Ok(())
}
