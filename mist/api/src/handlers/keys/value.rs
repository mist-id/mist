use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use mist_common::{crypto::decrypt_service_key, Result};
use mist_db::models::key::KeyId;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::state::ApiState;

#[derive(Deserialize, IntoParams)]
pub(crate) struct PathParams {
    id: KeyId,
}

#[utoipa::path(
    tags = ["Keys"],
    summary = "Get key value",
    get,
    path = "/{id}/value",
    params(PathParams),
    responses(
        (status = 200, body = String),
        (status = 404)
    )
)]
pub(crate) async fn value_handler(
    State(state): State<ApiState>,
    Path(path): Path<PathParams>,
) -> Result<impl IntoResponse> {
    let key = state.repos.keys.get(&path.id).await?;
    let decrypted = decrypt_service_key(&state.env.master_key, &key.value)?;
    let encoded = hex::encode(decrypted);

    Ok(encoded)
}

#[cfg(test)]
mod tests {
    use std::{future::ready, sync::Arc};

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use mist_common::{
        crypto::{create_service_key, encrypt_service_key},
        env::Environment,
    };
    use mist_db::{
        models::{key::Key, service::ServiceId},
        repos::{keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use secstr::SecVec;
    use tower::ServiceExt;

    use super::*;

    use crate::{handlers::keys::router, state::Repos};

    #[tokio::test]
    async fn gets() -> Result<()> {
        let master_key =
            SecVec::from("cec1125efa4807e7b9aa961b04646a73c26fb0df36a87bf43f475a06d3fe2026");
        let service_id = ServiceId::new();
        let id = KeyId::new();

        let mut keys = MockKeyRepo::new();

        let master_key_clone = master_key.clone();
        keys.expect_get().with(eq(id)).once().returning(move |_| {
            Box::pin(ready(Ok(Key {
                value: encrypt_service_key(&master_key_clone, &create_service_key()).unwrap(),
                ..Default::default()
            })))
        });

        let app = router().with_state(ApiState {
            env: Environment {
                master_key,
                ..Default::default()
            },
            repos: Repos {
                services: Arc::new(MockServiceRepo::new()),
                keys: Arc::new(keys),
            },
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/services/{service_id}/keys/{id}/value"))
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(()))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        Ok(())
    }
}
