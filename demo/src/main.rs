use std::net::SocketAddr;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use eyre::Report;
use maud::{html, Markup};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Deserialize)]
struct Environment {
    authn_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let env = envy::from_env::<Environment>()?;

    let app = Router::new()
        .route("/", get(root))
        .route("/hook", post(hook))
        .with_state(env);

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root(jar: CookieJar, State(env): State<Environment>) -> Result<Markup> {
    let Some(cookie) = jar.get("id") else {
        return Ok(login(&env));
    };

    // Check if the user is still logged in.
    let response = Client::new()
        .get(format!("{}/who", env.authn_url))
        .header("Cookie", format!("id={}", cookie.value()))
        .send()
        .await?;

    // Ask them to login if they're not.
    if response.status() == StatusCode::NOT_FOUND {
        return Ok(login(&env));
    }

    // Get the user session data.
    let user = response.json::<serde_json::Value>().await?;

    Ok(layout(html! {
        main {
            pre class="mb-4 p-4 rounded-md text-sm text-white bg-sky-900" {
                (serde_json::to_string_pretty(&user)?)
            }

            form method="POST" action={ (env.authn_url) "/kill" } {
                button
                    class="inline-block rounded border border-indigo-600 bg-indigo-600 px-12 py-3 text-md font-medium text-white hover:bg-white hover:text-indigo-600 focus:outline-none focus:ring active:text-indigo-500"
                    type="submit"
                    { "👋 Logout" }
            }
        }
    }))
}

fn layout(body: Markup) -> Markup {
    html! {
        title { "Sign in" }

        body class="bg-sky-50";

        script src="https://cdn.twind.style" crossorigin {}

        main class="flex flex-col items-center justify-center h-screen p-12" {
            (body)
        }
    }
}

fn login(env: &Environment) -> Markup {
    layout(html! {
        a
            class="inline-block rounded border border-indigo-600 bg-indigo-600 px-12 py-3 text-md font-medium text-white hover:bg-white hover:text-indigo-600 focus:outline-none focus:ring active:text-indigo-500"
            href={ (env.authn_url) "/ACME" }
            { "Login with Mist" }

        p class="mt-4 text-slate-500" {
            "What is this? Find out " a target="_blank" class="text-slate-900 hover:underline underline-offset-4" href="https://github.com/mist-id/mist" { "here" } "."
        }
    })
}

async fn hook(State(env): State<Environment>, Json(body): Json<Value>) -> Result<StatusCode> {
    Client::new()
        .post(format!("{}/hook", env.authn_url))
        .json(&body)
        .send()
        .await?;

    Ok(StatusCode::OK)
}

// Error handling.
// ---------------

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
