use std::{io::Cursor, str::FromStr};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use base64::prelude::*;
use common::error::Error;
use db::models::key::KeyKind;
use dif_presentation_exchange::PresentationDefinition;
use fred::prelude::*;
use http::StatusCode;
use image::{ImageFormat, Luma};
use maud::{html, PreEscaped};
use openidconnect::{
    core::{CoreClient, CoreResponseType},
    AuthUrl, AuthenticationFlow, ClientId, CsrfToken, IssuerUrl, JsonWebKeySet, RedirectUrl, Scope,
};
use qrcode::QrCode;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{
    cookie::{time::Duration, SameSite},
    Cookie, Cookies,
};
use uuid::Uuid;

use crate::{
    crypto::{
        keys::decrypt_service_key,
        oidc::{create_signed_nonce, created_signed_state},
    },
    state::AuthnState,
    AuthSessionData, COOKIE_KEY, REDIS_AUTH_KEY,
};

#[derive(Deserialize)]
pub(crate) struct CreatePath {
    service_name: String,
}

pub(crate) async fn handler(
    cookies: Cookies,
    State(state): State<AuthnState>,
    Path(path): Path<CreatePath>,
) -> Result<impl IntoResponse, Error> {
    let service = state.repos.services.get_by_name(&path.service_name).await?;

    let Some(service) = service else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    // Get the services' token key for signing the state and nonce.
    // ------------------------------------------------------------

    let service_key = state
        .repos
        .keys
        .preferred(&service.id, &KeyKind::Token)
        .await?;

    let service_key = decrypt_service_key(&state.env.master_key, &service_key)?;

    // Create a presentation from the services' default profile definition.
    // --------------------------------------------------------------------

    let profile = state
        .repos
        .services
        .get_default_profile(&service.id)
        .await?;

    let fields = profile
        .value
        .fields
        .iter()
        .map(|f| json!({ "id": heck::AsSnakeCase(f.name.clone()).to_string(), "name": f.name, "constraints": {} }))
        .collect::<Vec<_>>();

    let presentation = serde_json::from_value::<PresentationDefinition>(json!({
        "id": "registration-data",
        "input_descriptors": fields
    }))?;

    // Get or create a session for the user.
    // -------------------------------------

    let redis_client = state.redis_client.clone();
    let session_uuid = match cookies.get(COOKIE_KEY) {
        Some(cookie) => {
            let cookie_value = cookie.value().to_string();

            Some(
                redis_client
                    .get::<String, String>(format!("{REDIS_AUTH_KEY}-{}", cookie_value))
                    .await
                    .ok()
                    .and_then(|_| Uuid::from_str(&cookie_value).ok()),
            )
        }
        None => None,
    }
    .flatten();

    let session_uuid = match session_uuid {
        Some(value) => value,
        None => {
            let session_id = Uuid::new_v4();
            let user_id = Uuid::new_v4();

            state
                .redis_client
                .set(
                    format!("{REDIS_AUTH_KEY}-{}", session_id.clone()),
                    serde_json::to_string(&AuthSessionData {
                        service_id: service.id,
                        user_id,
                    })?,
                    Some(Expiration::EX(60 * 5)),
                    None,
                    false,
                )
                .await?;

            cookies.add(
                Cookie::build((COOKIE_KEY, session_id.to_string()))
                    .secure(!state.env.development)
                    .http_only(true)
                    .same_site(SameSite::Strict)
                    .max_age(Duration::hours(8))
                    .path("/")
                    .build(),
            );

            session_id
        }
    };

    // Create the authorization URL and render it as a QR code.
    // --------------------------------------------------------

    let oidc_client = CoreClient::new(
        ClientId::new(service.name),
        None,
        // Placeholder, not used for SIOP.
        IssuerUrl::new("https://not.needed".into())?,
        // Only the prefix matters for SIOP.
        AuthUrl::new("siopv2://authenticate".into())?,
        None,
        None,
        JsonWebKeySet::default(),
    );

    let state_sk = service_key.clone();
    let nonce_sk = service_key.clone();

    let (authorize_url, _, _) = oidc_client
        .set_redirect_uri(RedirectUrl::new(format!(
            "{}/verify",
            state.env.authn_base_url
        ))?)
        .authorize_url(
            AuthenticationFlow::<CoreResponseType>::Hybrid(vec![
                CoreResponseType::IdToken,
                CoreResponseType::Extension("vp_token".into()),
            ]),
            // It would be better if neither of these had to unwrap, but these args take a closure
            // that must return a CsrfToken and Nonce respectively. Really, though, the only way
            // this can fail is if the service key isn't long enough which should be prevented by
            // the validations when the key is created.
            move || {
                created_signed_state(&state_sk, CsrfToken::new_random().secret(), &session_uuid)
                    .unwrap()
            },
            move || {
                create_signed_nonce(&nonce_sk, openidconnect::Nonce::new_random().secret()).unwrap()
            },
        )
        .add_scope(Scope::new("vp_token".into()))
        .add_extra_param("id_token_type", "subject_signed_id_token")
        .add_extra_param(
            "presentation_definition",
            &serde_json::to_string::<PresentationDefinition>(&presentation)?,
        )
        .add_extra_param("response_mode", "post")
        .url();

    let code = QrCode::new(authorize_url.as_str())?;
    let image = code.render::<Luma<u8>>().max_dimensions(500, 500).build();
    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    image.write_to(&mut cursor, ImageFormat::Png)?;
    let encoded = BASE64_STANDARD.encode(buf);

    Ok(html! {
        script src="https://cdn.twind.style" crossorigin {}

        script {
            (PreEscaped(format!(r#"
                document.addEventListener("DOMContentLoaded", () => {{
                    const source = new EventSource("/waiting");

                    source.onmessage = (event) => {{
                        if (event.data === "ready") {{
                            window.location.href = "{0}"
                        }}
                    }};
                }});
            "#, service.redirect_url)))
        }

        body class="bg-slate-100";

        main class="flex flex-col items-center justify-center h-screen p-12 shadow-md" {
            h1 class="mb-4 text-2xl text-slate-700" { "Scan the QR code to Sign in" }

            img src={"data:image/png;base64," (encoded)};

            p class="mt-4 text-slate-500" { "Your identity, your data —"
                span class="text-slate-700" { " Anchored in "
                    a class="text-slate-900 underline underline-offset-8" href="https://mist.id" { "Mist" }
                }
            }

            div class="mt-2 text-black" {
                svg style="width: 50px" {
                    path d="M7.055 25.445c0 9.93 8.015 17.946 17.945 17.946 9.852 0 17.867-8.016 17.945-17.868a4.486 4.486 0 0 1-4.508 4.352c-2.44 0-4.503-1.984-4.503-4.508v-5.32c0-7.406-6.032-13.438-13.442-13.438-7.406 0-13.437 6.032-13.437 13.438Zm6.715-7.843a2.855 2.855 0 0 1 2.855 2.859v4c0 1.578-1.29 2.86-2.855 2.86a2.86 2.86 0 0 1-2.856-2.86v-4a2.86 2.86 0 0 1 2.856-2.86Zm8.37 0A2.856 2.856 0 0 1 25 20.46v4a2.868 2.868 0 0 1-2.86 2.86 2.86 2.86 0 0 1-2.85-2.86v-4a2.86 2.86 0 0 1 2.85-2.86Zm0 0";
                }
            }
        }
    }
    .into_response())

    // -----------------------------------------------------
    // Once the user has responded to our auth request,
    // we will continue the process in the `verify` handler.
}
