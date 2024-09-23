use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_garde::WithValidation;
use garde::Validate;
use mist_common::Result;
use mist_db::models::{
    key::{CreateKey, KeyKind},
    service::ServiceId,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::state::ApiState;

#[derive(Serialize, Deserialize, IntoParams)]
pub(crate) struct PathParams {
    service_id: ServiceId,
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[schema(as = CreateKeyPayload)]
pub(crate) struct Payload {
    #[garde(skip)]
    kind: KeyKind,
}

#[utoipa::path(
    tags = ["Keys"],
    summary = "Create key",
    post,
    path = "",
    request_body = CreateKeyPayload,
    responses(
        (status = 201, body = Key)
    )
)]
pub(crate) async fn create_handler(
    State(state): State<ApiState>,
    Path(path): Path<PathParams>,
    WithValidation(payload): WithValidation<Json<Payload>>,
) -> Result<impl IntoResponse> {
    let key = state
        .repos
        .keys
        .create(
            &state.env.master_key,
            &CreateKey::builder()
                .kind(payload.kind.clone())
                .priority(1)
                .service_id(path.service_id)
                .build(),
        )
        .await?;

    Ok((StatusCode::CREATED, Json(key)))
}

#[cfg(test)]
mod tests {
    use std::{future::ready, sync::Arc};

    use axum::{body::Body, extract::Request, http};
    use mist_common::env::Environment;
    use mist_db::{
        models::key::Key,
        repos::{keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use secstr::SecVec;
    use tower::ServiceExt;

    use crate::{handlers::keys::router, state::Repos};

    use super::*;

    #[tokio::test]
    async fn creates() -> Result<()> {
        let master_key =
            SecVec::from("57a9f41af0c50e8b6560e5b65ad3b4111fa78a591ab6ce2f87d0b8c18d8ecd9");
        let service_id = ServiceId::new();

        let mut keys = MockKeyRepo::new();

        keys.expect_create()
            .with(
                eq(master_key.clone()),
                eq(CreateKey::builder()
                    .kind(KeyKind::Token)
                    .priority(1)
                    .service_id(service_id)
                    .build()),
            )
            .once()
            .returning(|_, _| Box::pin(ready(Ok(Key::default()))));

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
                    .method(http::Method::POST)
                    .uri(format!("/services/{service_id}/keys"))
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"
                        {
                            "kind": "token"
                        }
                    "#,
                    ))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::CREATED);

        Ok(())
    }
}
