use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use common::Result;
use db::models::key::{KeyId, UpdateKey};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::state::ApiState;

#[derive(Serialize, Deserialize, IntoParams)]
pub(crate) struct PathParams {
    id: KeyId,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = UpdateKeyPayload)]
pub(crate) struct Payload {
    #[serde(rename = "active")]
    is_active: Option<bool>,
}

#[utoipa::path(
    tags = ["Keys"],
    summary = "Update key",
    put,
    path = "/{id}",
    params(PathParams),
    request_body = UpdateKeyPayload,
    responses(
        (status = 200, body = Key),
        (status = 404)
    )
)]
pub(crate) async fn update_handler(
    State(state): State<ApiState>,
    Path(path): Path<PathParams>,
    Json(payload): Json<Payload>,
) -> Result<impl IntoResponse> {
    let key = state
        .repos
        .keys
        .update(
            &path.id,
            &UpdateKey::builder()
                .maybe_is_active(payload.is_active)
                .build(),
        )
        .await?;

    Ok((StatusCode::OK, Json(key)))
}

#[cfg(test)]
mod tests {
    use std::{future::ready, sync::Arc};

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use common::env::Environment;
    use db::{
        models::{
            key::{Key, UpdateKey},
            service::ServiceId,
        },
        repos::{keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use tower::ServiceExt;

    use super::*;

    use crate::{handlers::keys::router, state::Repos};

    #[tokio::test]
    async fn updates() -> Result<()> {
        let service_id = ServiceId::new();
        let id = KeyId::new();

        let mut keys = MockKeyRepo::new();

        keys.expect_update()
            .with(eq(id), eq(UpdateKey::builder().is_active(true).build()))
            .once()
            .returning(|_, _| Box::pin(ready(Ok(Key::default()))));

        let app = router().with_state(ApiState {
            env: Environment::default(),
            repos: Repos {
                services: Arc::new(MockServiceRepo::new()),
                keys: Arc::new(keys),
            },
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/services/{service_id}/keys/{id}"))
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"
                        {
                            "active": true
                        }
                    "#,
                    ))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        Ok(())
    }
}
