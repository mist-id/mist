use std::net::SocketAddr;

use axum::{
    debug_handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use eyre::Report;
use maud::{html, Markup};
use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/hook", post(hook));

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root(jar: CookieJar) -> Result<Markup> {
    let Some(cookie) = jar.get("id") else {
        return Ok(html! {
            script src="https://cdn.twind.style" crossorigin {}

            main {
                a href="http://0.0.0.0:9002/ACME" { "Login with Mist" }
            }
        });
    };

    let response = Client::new()
        .get("http://0.0.0.0:9002/who")
        .header("Cookie", format!("id={}", cookie.value()))
        .send()
        .await?;

    if response.status() == StatusCode::NOT_FOUND {
        return Ok(html! {
            script src="https://cdn.twind.style" crossorigin {}

            main {
                a href="http://0.0.0.0:9002/ACME" { "Login with Mist" }
            }
        });
    }

    let user = response.json::<serde_json::Value>().await?;

    Ok(html! {
        script src="https://cdn.twind.style" crossorigin {}

        main {
            pre { (serde_json::to_string_pretty(&user)?) }

            form method="POST" action="http://0.0.0.0:9002/kill" {
                button type="submit" { "Logout" }
            }
        }
    })
}

async fn hook(Json(body): Json<Value>) -> Result<StatusCode> {
    Client::new()
        .post("http://0.0.0.0:9002/hook")
        .json(&body)
        .send()
        .await?;

    Ok(StatusCode::OK)
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct Error(Report);

impl<E> From<E> for Error
where
    E: Into<Report>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
