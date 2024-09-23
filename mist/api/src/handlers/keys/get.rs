use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use mist_common::Result;
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
    summary = "Get key",
    get,
    path = "/{id}",
    params(PathParams),
    responses(
        (status = 200, body = Key),
        (status = 404)
    )
)]
pub(crate) async fn get_handler(
    State(state): State<ApiState>,
    Path(path): Path<PathParams>,
) -> Result<impl IntoResponse> {
    let key = state.repos.keys.get(&path.id).await?;

    Ok(Json(key))
}

#[cfg(test)]
mod tests {
    use std::{future::ready, sync::Arc};

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use mist_common::env::Environment;
    use mist_db::{
        models::{key::Key, service::ServiceId},
        repos::{keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use tower::ServiceExt;

    use super::*;

    use crate::{handlers::keys::router, state::Repos};

    #[tokio::test]
    async fn gets() -> Result<()> {
        let service_id = ServiceId::new();
        let id = KeyId::new();

        let mut keys = MockKeyRepo::new();

        keys.expect_get()
            .with(eq(id))
            .once()
            .returning(|_| Box::pin(ready(Ok(Key::default()))));

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
                    .method(http::Method::GET)
                    .uri(format!("/services/{service_id}/keys/{id}"))
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(()))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        Ok(())
    }
}
