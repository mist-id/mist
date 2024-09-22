use std::{io::Cursor, str::FromStr};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use base64::prelude::*;
use common::{crypto::decrypt_service_key, Result};
use db::models::{key::KeyKind, user::UserId};
use dif_presentation_exchange::PresentationDefinition;
use fred::types::Expiration;
use image::{ImageFormat, Luma};
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

use crate::{
    session::{AuthAction, AuthSession, AuthState, SessionId, AUTH_SESSION, COOKIE_KEY},
    state::AuthnState,
    utils::oidc,
    views,
};

#[derive(Deserialize)]
pub(crate) struct CreatePath {
    service_name: String,
    action: AuthAction,
}

pub(crate) async fn handler(
    cookies: Cookies,
    State(state): State<AuthnState>,
    Path(path): Path<CreatePath>,
) -> Result<impl IntoResponse> {
    // Get the services' token key for signing the state and nonce.
    // ------------------------------------------------------------

    let service = state.repos.services.get_by_name(&path.service_name).await?;

    let service_key = state
        .repos
        .keys
        .preferred(&service.id, &KeyKind::Token)
        .await?;

    let service_key = decrypt_service_key(&state.env.master_key, &service_key.value)?;

    // Get or create a session for the user.
    // -------------------------------------

    let redis_client = state.redis.clone();
    let session_id = match cookies.get(COOKIE_KEY) {
        Some(cookie) => Some(
            AUTH_SESSION
                .get(&redis_client, cookie.value())
                .await
                .ok()
                .and_then(|_| SessionId::from_str(cookie.value()).ok()),
        ),
        None => None,
    }
    .flatten();

    let session_id = match session_id {
        Some(value) => value,
        None => {
            let session_id = SessionId::new();
            let user_id = UserId::new();

            AUTH_SESSION
                .set(
                    &redis_client,
                    &session_id.to_string(),
                    &AuthSession {
                        service_id: service.id,
                        user_id,
                        state: AuthState::Authenticating {
                            action: path.action.clone(),
                        },
                    },
                    Expiration::EX(60 * 5),
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

    // Create a presentation from the service's default profile definition.
    // --------------------------------------------------------------------

    let fields = match path.action {
        AuthAction::Up => {
            let profile = state
                .repos
                .services
                .get_default_profile(&service.id)
                .await?;

            if let Some(profile) = profile {
                let fields = profile
                    .value
                    .fields
                    .iter()
                    .map(|f| json!({ "id": heck::AsSnakeCase(f.name.clone()).to_string(), "name": f.name, "constraints": {} }))
                    .collect::<Vec<_>>();

                fields
            } else {
                vec![json!({ "id": "skip", "name": "Skip", "constraints": {} })]
            }
        }
        AuthAction::In => {
            // Ideally, we send no fields for signing in, but Sphereon seems to require we send _something_.
            //
            // TODO: Remove this when I can.
            vec![json!({ "id": "skip", "name": "Skip", "constraints": {} })]
        }
    };

    let presentation = serde_json::from_value::<PresentationDefinition>(json!({
        "id": "registration-data",
        "input_descriptors": fields
    }))?;

    // Create the authorization URL and render it as a QR code.
    // --------------------------------------------------------

    let oidc_client = CoreClient::new(
        ClientId::new(service.name.clone()),
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

    let redirect_url = RedirectUrl::new(format!("{}/auth", state.env.authn_url))?;

    let (authorize_url, _, _) = oidc_client
        .set_redirect_uri(redirect_url)
        .authorize_url(
            AuthenticationFlow::<CoreResponseType>::Hybrid(vec![
                CoreResponseType::IdToken,
                CoreResponseType::Extension("vp_token".into()),
            ]),
            move || {
                oidc::created_signed_state(&state_sk, CsrfToken::new_random().secret(), &session_id)
                    .unwrap()
            },
            move || {
                oidc::create_signed_nonce(&nonce_sk, openidconnect::Nonce::new_random().secret())
                    .unwrap()
            },
        )
        .add_scope(Scope::new("vp_token".into()))
        .add_extra_param("response_mode", "post")
        .add_extra_param("id_token_type", "subject_signed_id_token")
        .add_extra_param(
            "presentation_definition",
            &serde_json::to_string::<PresentationDefinition>(&presentation)?,
        )
        .url();

    // Render the authorization URL as a QR code.
    let code = QrCode::new(authorize_url.as_str())?;
    let image = code.render::<Luma<u8>>().max_dimensions(500, 500).build();
    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    image.write_to(&mut cursor, ImageFormat::Png)?;
    let encoded = BASE64_STANDARD.encode(buf);

    // Render the view.
    Ok(views::scan::view(&service, &state.env.authn_url, &encoded))

    // -----------------------------------------------------------------------------
    // Once the user has responded to our auth request, we will continue the process
    // in the `verify_response` handler.
}
