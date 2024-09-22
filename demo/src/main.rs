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
use maud::{html, Markup, PreEscaped};
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
    let Some(cookie) = jar.get("mist") else {
        return Ok(login(&env));
    };

    // Check if the user is still logged in.
    let response = Client::new()
        .get(format!("{}/whoami", env.authn_url))
        .header("Cookie", format!("mist={}", cookie.value()))
        .send()
        .await?;

    // Ask them to login if they're not.
    if response.status() == StatusCode::NOT_FOUND {
        return Ok(login(&env));
    }

    // Get the user session data.
    let user = response.json::<serde_json::Value>().await?;

    let json = serde_json::to_string_pretty(&user)?
        .lines()
        .map(|line| html! { pre { code { (line) } } }.into_string())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(layout(html! {
        script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js" crossorigin {}
        link href="https://unpkg.com/@catppuccin/highlightjs@1.0.0/css/catppuccin-mocha.css" rel="stylesheet" type="text/css";

        script { "hljs.highlightAll();" }

        div class="mb-4 text-xs w-1/3 mockup-code" style="background-color: #1e1e2e;" {
            pre class="language-none mb-2" data-prefix="$" { code { "curl -X GET " (env.authn_url) "/whoami -H 'Cookie: mist=" (cookie.value()) "' | jq" } }

            div class="mx-2 language-json catppuccin-mocha" {
                (PreEscaped(json))
            }
        }

        form method="POST" action={ (env.authn_url) "/ACME/out" } {
            button class="btn btn-outline btn-neutral" type="submit" { "👋 Sign out" }
        }
    }))
}

fn layout(body: Markup) -> Markup {
    html! {
        title { "Sign in" }

        script src="https://cdn.twind.style" crossorigin {}
        link href="https://cdn.jsdelivr.net/npm/daisyui@4.12.10/dist/full.min.css" rel="stylesheet" type="text/css";

        body class="bg-base-200";

        main class="flex flex-col items-center justify-center h-screen" {
            h1 class="mb-4 text-5xl font-bold" { "ACME" }

            (body)
        }
    }
}

fn login(env: &Environment) -> Markup {
    layout(html! {
        div class="join" {
            a class="btn join-item btn-outline btn-neutral" href={ (env.authn_url) "/ACME/up" } { "Sign up" }
            a class="btn join-item btn-outline btn-neutral" href={ (env.authn_url) "/ACME/in" } { "Sign in" }
        }
    })
}

async fn hook(State(env): State<Environment>, Json(body): Json<Value>) -> Result<StatusCode> {
    Client::new()
        .post(format!("{}/hook", env.authn_url))
        .json(&serde_json::json!({ "registration": { "meta": body["meta"], "complete": true } }))
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
