use std::str::FromStr;

use anyhow::Context;
use axum::{extract::State, response::IntoResponse, Form};
use common::error::Error;
use db::models::key::KeyKind;
use fred::prelude::*;
use http::StatusCode;
use openidconnect::core::CoreIdTokenClaims;
use serde::Deserialize;
use serde_json::{Map, Value};
use ssi::{
    did::VerificationMethod,
    did_resolve::ResolutionResult,
    vc::{Credential, OneOrMany},
};
use uuid::Uuid;

use crate::{
    crypto::{
        keys::decrypt_service_key,
        oidc::{sign_nonce, sign_state},
    },
    state::AuthnState,
    webhooks::RegistrationWebhook,
    AuthHookSessionData, AuthSessionData, REDIS_AUTH_HOOK_KEY, REDIS_AUTH_KEY,
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
) -> Result<impl IntoResponse, Error> {
    // Get the user's session data.
    // ---------------------

    let parts = body.state.split(':').collect::<Vec<&str>>();

    let [received_state, received_session_id, received_signature] = parts.as_slice() else {
        return Err(anyhow::anyhow!("state does not match expected structure").into());
    };

    let session_data = state
        .redis_client
        .get::<String, _>(&format!("{REDIS_AUTH_KEY}-{received_session_id}"))
        .await?;

    let session_data = serde_json::from_str::<AuthSessionData>(&session_data)?;

    // Get the services' token key for verifying the state and nonce.
    // --------------------------------------------------------------

    let service = state.repos.services.get(&session_data.service_id).await?;

    let Some(service) = service else {
        return Ok(StatusCode::NOT_FOUND);
    };

    let service_key = state
        .repos
        .keys
        .preferred(&service.id, &KeyKind::Token)
        .await?;

    let service_key = decrypt_service_key(&state.env.master_key, &service_key)?;

    // Verify that the state returned by the user matches the one we sent.
    // -------------------------------------------------------------------

    let expected_signature = sign_state(
        &service_key,
        received_state,
        &Uuid::from_str(received_session_id)?,
    )?;

    if received_signature != &expected_signature {
        return Err(anyhow::anyhow!("state does not match").into());
    }

    // Get some useful information from the JWT header.
    // ------------------------------------------------

    // Extract header first to figure out what algo to use.
    let header = jsonwebtoken::decode_header(&body.id_token)?;

    // The `kid` in this case is the user's DID.
    let did = header.kid.context("header missing kid")?;

    // Resolve the DID.
    // ----------------

    let document = reqwest::get(format!(
        "{resolver_host}/{did}",
        resolver_host = &state.env.resolver_host
    ))
    .await?
    .json::<ResolutionResult>()
    .await?
    .did_document
    .context("no document")?;

    // Get the verification method to use for authentication.
    // ------------------------------------------------------

    let verif_methods = document
        .verification_method
        .context("document is missing verification methods")?;
    let auth_methods = document
        .authentication
        .context("document is missing authentication property")?;

    // Get the first auth method for convenience.
    let first = auth_methods
        .first()
        .context("document is missing an auth method")?;

    let method = match first {
        // If the verification method is a DID URL, we need to resolve that DID and find the
        // verification method where ID = DID URL.
        VerificationMethod::DIDURL(url) => {
            let other_document = reqwest::get(format!(
                "{resolver_host}/{url}",
                resolver_host = &state.env.resolver_host
            ))
            .await?
            .json::<ResolutionResult>()
            .await?
            .did_document
            .context("no document")?;

            &other_document
                .verification_method
                .context("other document is missing verification methods")?
                .iter()
                .filter_map(|m| {
                    if let VerificationMethod::Map(v) = m {
                        Some(v)
                    } else {
                        None
                    }
                })
                .find(|m| m.id == url.to_string())
                .context("could not find verification method")?
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
            .context("could not find verification method")?
            .clone(),
        // In the case where it's a map, we can just return it directly.
        VerificationMethod::Map(method) => method,
    };

    // Verify and decode the ID Token by using the public key from the found verification method.
    // ------------------------------------------------------------------------------------------

    let jwk = method
        .public_key_jwk
        .as_ref()
        .context("could not get public jwk")?;

    let decoded_id_token = ssi::jwt::decode_verify::<CoreIdTokenClaims>(&body.id_token, jwk)?;

    // Verify that the nonce in the ID token matches the one we sent.
    // --------------------------------------------------------------

    let parts = decoded_id_token
        .nonce()
        .context("could not find nonce")?
        .secret()
        .split(':')
        .collect::<Vec<&str>>();
    let (received_nonce, received_signature) = parts
        .first()
        .zip(parts.get(1))
        .context("nonce does not match expected structure")?;

    let expected_signature = sign_nonce(&service_key, received_nonce)?;

    if received_signature != &expected_signature {
        return Err(anyhow::anyhow!("invalid nonce").into());
    }

    // Get their profile data from the received VCs.
    // --------------------------------------------

    let decoded_vp_token = ssi::jwt::decode_verify::<SphereonTokenWrapper>(&body.vp_token, jwk)?;
    let credential = decoded_vp_token.verifiable_credential[0].clone();
    let decoded_credential = ssi::jwt::decode_unverified::<SphereonCredentialWrapper>(&credential)?;

    let OneOrMany::One(subject) = decoded_credential.vc.credential_subject else {
        return Err(anyhow::anyhow!("multiple subjects").into());
    };

    let profile = subject.property_set.context("no property set")?;
    let json_map = profile.into_iter().collect::<Map<String, Value>>();

    // Send user data to the services' webhook endpoint so they can create the user on their end.
    // ------------------------------------------------------------------------------------------

    let hook = RegistrationWebhook::new(&session_data.user_id, &did, json_map);

    state
        .redis_client
        .set(
            format!("{REDIS_AUTH_HOOK_KEY}-{}", hook.meta.id),
            serde_json::to_string(&AuthHookSessionData {
                hook_id: hook.meta.id,
                user_session_id: Uuid::from_str(received_session_id)?,
            })?,
            Some(Expiration::EX(60 * 5)),
            None,
            false,
        )
        .await?;

    reqwest::Client::new()
        .post(service.webhook_url)
        .json(&hook)
        .send()
        .await?;

    Ok(StatusCode::OK)

    // -------------------------------------------------------------------------
    // If all is well, the service will respond to our webhook request
    // with their own webhook request that we will handle in the `hook` handler.
}

// It's unclear to me why these are needed. The spec itself doesn't mention
// anything about them, so I think they're specific to Sphereon?
// -------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SphereonTokenWrapper {
    verifiable_credential: Vec<String>,
}

#[derive(Deserialize)]
struct SphereonCredentialWrapper {
    vc: Credential,
}
