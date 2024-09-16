use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use common::Result;
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::state::ApiState;

#[derive(Deserialize, IntoParams)]
pub(crate) struct PathParams {
    id: Uuid,
}

#[utoipa::path(
    tags = ["Keys"],
    summary = "Get key",
    get,
    path = "/{id}",
    params(PathParams),
    responses(
        (status = 200, body = Key),
        (status = 400)
    )
)]
pub(crate) async fn get_handler(
    State(state): State<ApiState>,
    Path(path): Path<PathParams>,
) -> Result<impl IntoResponse> {
    let response = state
        .repos
        .keys
        .get(&path.id)
        .await?
        .map_or(StatusCode::NOT_FOUND.into_response(), |r| {
            Json(r).into_response()
        });

    Ok(response)
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
        models::key::Key,
        repos::{keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use tower::ServiceExt;

    use super::*;

    use crate::{handlers::keys::router, state::Repos};

    #[tokio::test]
    async fn gets() -> Result<()> {
        let id = Uuid::new_v4();

        let mut keys = MockKeyRepo::new();

        keys.expect_get()
            .with(eq(id))
            .once()
            .returning(|_| Box::pin(ready(Ok(Some(Key::default())))));

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
                    .uri(format!("/keys/{id}"))
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(()))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        Ok(())
    }
}
