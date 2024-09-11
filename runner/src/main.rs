use std::net::SocketAddr;

use common::{env::Environment, Result};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    sqlx::migrate!("../db/migrations");

    let env = envy::from_env::<Environment>().unwrap();

    let api_env = env.clone();
    let authn_env = env.clone();

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
    }

    Ok(())
}
