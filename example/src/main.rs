use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use maud::{html, Markup};
use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/hook", post(hook));

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(jar: CookieJar) -> Markup {
    let Some(cookie) = jar.get("id") else {
        return html! {
            script src="https://cdn.twind.style" crossorigin {}

            main {
                a href="http://0.0.0.0:9002/ACME" { "Login with Mist" }
            }
        };
    };

    let response = Client::new()
        .get("http://0.0.0.0:9002/who")
        .header("Cookie", format!("id={}", cookie.value()))
        .send()
        .await
        .unwrap();

    if response.status() == StatusCode::UNAUTHORIZED {
        return html! {
            script src="https://cdn.twind.style" crossorigin {}

            main {
                a href="http://0.0.0.0:9002/ACME" { "Login with Mist" }
            }
        };
    }

    let user = response.json::<serde_json::Value>().await.unwrap();

    html! {
        script src="https://cdn.twind.style" crossorigin {}

        main {
            pre { (serde_json::to_string_pretty(&user).unwrap()) }
        }
    }
}

async fn hook(Json(body): Json<Value>) -> StatusCode {
    Client::new()
        .post("http://0.0.0.0:9002/hook")
        .json(&body)
        .send()
        .await
        .unwrap();

    StatusCode::OK
}
