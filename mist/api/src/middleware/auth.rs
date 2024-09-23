use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use constant_time_eq::constant_time_eq;
use mist_common::{crypto::decrypt_service_key, error::Result};
use mist_db::models::{key::KeyKind, service::ServiceId};
use serde::Deserialize;

use crate::state::ApiState;

#[derive(Deserialize)]
pub(crate) struct PathParams {
    service_id: Option<ServiceId>,
}

pub(crate) async fn middleware(
    State(state): State<ApiState>,
    Path(path): Path<PathParams>,
    request: Request,
    next: Next,
) -> Result<Response> {
    // Check if the request has an Authorization header.
    let Some(header) = request.headers().get("Authorization") else {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    };

    // Check if the Authorization header is the master key.
    // ----------------------------------------------------

    let is_master_key = constant_time_eq(
        &hex::decode(header.as_bytes())?,
        &hex::decode(state.env.master_key.unsecure())?,
    );

    // Check if the Authorization header is a service key.
    // ---------------------------------------------------

    let is_service_key = if let Some(service_id) = path.service_id {
        let service_key = state
            .repos
            .keys
            .preferred(&service_id, &KeyKind::Api)
            .await?;

        constant_time_eq(
            &hex::decode(header.as_bytes())?,
            &decrypt_service_key(&state.env.master_key, &service_key.value)?,
        )
    } else {
        false
    };

    // Authorize if given a master key or relevant service key.
    if is_master_key || (path.service_id.is_some() && is_service_key) {
        Ok(next.run(request).await)
    } else {
        Ok(StatusCode::UNAUTHORIZED.into_response())
    }
}
