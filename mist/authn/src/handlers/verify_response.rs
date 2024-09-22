use std::str::FromStr;

use axum::{extract::State, response::IntoResponse, Form};
use chrono::Utc;
use common::{crypto::decrypt_service_key, Result};
use db::models::{key::KeyKind, service::Service};
use eyre::{eyre, OptionExt};
use fred::prelude::*;
use http::StatusCode;
use openidconnect::core::CoreIdTokenClaims;
use serde::Deserialize;
use serde_json::{Map, Value};
use ssi::{did::VerificationMethod, did_resolve::ResolutionResult, jwk::JWK, vc::OneOrMany};

use crate::{
    events::{get_event_key, Event},
    session::{AuthAction, AuthSession, AuthState, SessionId, AUTH_SESSION},
    state::AuthnState,
    utils::{
        oidc,
        sphereon::{SphereonCredentialWrapper, SphereonTokenWrapper},
    },
    webhooks::{self, HookData, Webhook, HOOK_DATA},
};

#[derive(Deserialize)]
pub(crate) struct VerifyBody {
    state: String,
    id_token: String,
    vp_token: String,
}

pub(crate) async fn handler(
    State(state): State<AuthnState>,
    Form(body): Form<VerifyBody>,
) -> Result<impl IntoResponse> {
    // Get the user's session data.
    // ----------------------------

    let parts = body.state.split(':').collect::<Vec<&str>>();

    let [received_state, received_session_id, received_signature] = parts.as_slice() else {
        return Err(eyre!("state does not match expected structure").into());
    };

    let session = AUTH_SESSION.get(&state.redis, received_session_id).await?;

    let AuthState::Authenticating { action } = &session.state else {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    };

    // Get the services' token key for verifying the state and nonce.
    // --------------------------------------------------------------

    let service = state.repos.services.get(&session.service_id).await?;

    let service_key = state
        .repos
        .keys
        .preferred(&service.id, &KeyKind::Token)
        .await?;

    let service_key = decrypt_service_key(&state.env.master_key, &service_key.value)?;

    // Verify that the state returned by the user matches the one we sent.
    // -------------------------------------------------------------------

    let expected_signature = oidc::sign_state(
        &service_key,
        received_state,
        &SessionId::from_str(received_session_id)?,
    )?;

    if received_signature != &expected_signature {
        return Err(eyre!("state does not match").into());
    }

    // Get some useful information from the JWT header.
    // ------------------------------------------------

    // Extract header first to figure out what algo to use.
    let header = jsonwebtoken::decode_header(&body.id_token)?;

    // The `kid` in this case is the user's DID.
    let did = header.kid.ok_or_eyre("header missing kid")?;

    // Resolve the DID.
    // ----------------

    let document = reqwest::get(format!(
        "{resolver_host}/{did}",
        resolver_host = &state.env.resolver_url
    ))
    .await?
    .json::<ResolutionResult>()
    .await?
    .did_document
    .ok_or_eyre("no document")?;

    // Get the verification method to use for authentication.
    // ------------------------------------------------------

    let verif_methods = document
        .verification_method
        .ok_or_eyre("document is missing verification methods")?;
    let auth_methods = document
        .authentication
        .ok_or_eyre("document is missing authentication property")?;

    // Get the first auth method for convenience.
    let first = auth_methods
        .first()
        .ok_or_eyre("document is missing an auth method")?;

    let method = match first {
        // If the verification method is a DID URL, we need to resolve that DID and find the
        // verification method where ID = DID URL.
        VerificationMethod::DIDURL(url) => {
            let other_document = reqwest::get(format!(
                "{resolver_host}/{url}",
                resolver_host = &state.env.resolver_url
            ))
            .await?
            .json::<ResolutionResult>()
            .await?
            .did_document
            .ok_or_eyre("no document")?;

            &other_document
                .verification_method
                .ok_or_eyre("other document is missing verification methods")?
                .iter()
                .filter_map(|m| {
                    if let VerificationMethod::Map(v) = m {
                        Some(v)
                    } else {
                        None
                    }
                })
                .find(|m| m.id == url.to_string())
                .ok_or_eyre("could not find verification method")?
                .clone()
        }
        // If the verification method is a DID URL fragment, we need to find the verification
        // method where ID = fargment.
        VerificationMethod::RelativeDIDURL(url) => &verif_methods
            .iter()
            .filter_map(|m| {
                if let VerificationMethod::Map(v) = m {
                    Some(v)
                } else {
                    None
                }
            })
            .find(|m| m.id == url.to_string())
            .ok_or_eyre("could not find verification method")?
            .clone(),
        // In the case where it's a map, we can just return it directly.
        VerificationMethod::Map(method) => method,
    };

    // Verify and decode the ID Token by using the public key from the found verification method.
    // ------------------------------------------------------------------------------------------

    let jwk = method
        .public_key_jwk
        .as_ref()
        .ok_or_eyre("could not get public jwk")?;

    // Ensure that the public key is for verification. I _think_ this should be
    // `verify`, but Sphereon sends `sig`.
    //
    // See: https://www.rfc-editor.org/rfc/rfc7517#section-4.3
    if jwk.public_key_use.as_deref() != Some("sig") {
        return Err(eyre!("public key is not for verification").into());
    }

    let decoded_id_token = ssi::jwt::decode_verify::<CoreIdTokenClaims>(&body.id_token, jwk)?;

    // Make sure the token hasn't expired.
    if decoded_id_token.expiration() < Utc::now() {
        return Err(eyre!("token has expired").into());
    }

    // Verify that the nonce in the ID token matches the one we sent.
    // --------------------------------------------------------------

    let parts = decoded_id_token
        .nonce()
        .ok_or_eyre("could not find nonce")?
        .secret()
        .split(':')
        .collect::<Vec<&str>>();
    let (received_nonce, received_signature) = parts
        .first()
        .zip(parts.get(1))
        .ok_or_eyre("nonce does not match expected structure")?;

    let expected_signature = oidc::sign_nonce(&service_key, received_nonce)?;

    if received_signature != &expected_signature {
        return Err(eyre!("invalid nonce").into());
    }

    match action {
        AuthAction::Up => {
            handle_up(
                &state,
                &session,
                received_session_id,
                &service,
                jwk,
                &did,
                &body.vp_token,
            )
            .await?
        }
        AuthAction::In => handle_in(&state, &service, received_session_id, &did).await?,
    }

    Ok(StatusCode::OK.into_response())
}

async fn handle_up(
    state: &AuthnState,
    session: &AuthSession,
    session_id: &str,
    service: &Service,
    jwk: &JWK,
    did: &str,
    vp_token: &str,
) -> Result<()> {
    // Get their profile data from the received VCs.
    // --------------------------------------------

    let decoded_vp_token = ssi::jwt::decode_verify::<SphereonTokenWrapper>(vp_token, jwk)?;
    let credential = decoded_vp_token.verifiable_credential[0].clone();
    let decoded_credential = ssi::jwt::decode_unverified::<SphereonCredentialWrapper>(&credential)?;

    let OneOrMany::One(subject) = decoded_credential.vc.credential_subject else {
        return Err(eyre!("multiple subjects").into());
    };

    let profile = subject.property_set.ok_or_eyre("no property set")?;
    let json_map = profile.into_iter().collect::<Map<String, Value>>();

    // Send user data to the services' webhook endpoint so they can create the user on their end.
    // ------------------------------------------------------------------------------------------

    let hook = Webhook::new(
        webhooks::Kind::Registration,
        webhooks::Request::Registration(webhooks::registration::Request {
            id: session.user_id,
            identifier: did.into(),
            profile: json_map,
        }),
    );

    HOOK_DATA
        .set(
            &state.redis,
            &hook.meta.id.to_string(),
            &HookData {
                session_id: SessionId::from_str(session_id)?,
                identifier: did.into(),
            },
            Expiration::EX(60 * 5),
        )
        .await?;

    reqwest::Client::new()
        .post(&service.webhook_url)
        .json(&hook)
        .send()
        .await?;

    Ok(())

    // ------------------------------------------------------------------------------------
    // When the server responds, we'll continue the process in the `handle_webhook` handler.
}

async fn handle_in(
    state: &AuthnState,
    service: &Service,
    session_id: &str,
    did: &str,
) -> Result<()> {
    // Get the existing uer ID via their DID.
    // --------------------------------------

    let identifier = state.repos.identifiers.get_by_value(did).await?;
    let user = state.repos.users.get(&identifier.user_id).await?;

    // Complete the authentication process.
    // ------------------------------------

    // Update user session.
    AUTH_SESSION
        .set(
            &state.redis,
            session_id,
            &AuthSession {
                service_id: service.id,
                user_id: user.id,
                state: AuthState::Authenticated {
                    identifier_id: identifier.id,
                },
            },
            Expiration::EX(60 * 60 * 8),
        )
        .await?;

    // Send an event to the user's browser to let it know authentication is complete.
    //
    // This event will be picked up by an event listener in the browser listening to
    // the `waiting_for_completion` handler.
    state
        .nats
        .publish(get_event_key(&Event::Redirect, session_id), "".into())
        .await?;

    Ok(())
}
