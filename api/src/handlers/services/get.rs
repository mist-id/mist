use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use common::error::Error;
use serde::Deserialize;
use uuid::Uuid;

use crate::state::ApiState;

#[derive(Deserialize)]
pub(crate) struct GetPath {
    id: Uuid,
}

pub(crate) async fn handler(
    State(state): State<ApiState>,
    Path(path): Path<GetPath>,
) -> Result<impl IntoResponse, Error> {
    let response = state
        .repos
        .services
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

    use anyhow::Result;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use common::env::Environment;
    use db::{
        models::service::Service,
        repos::{definitions::MockDefinitionRepo, keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use tower::ServiceExt;

    use super::*;

    use crate::{handlers::services::router, state::Repos};

    #[tokio::test]
    async fn gets() -> Result<()> {
        let id = Uuid::new_v4();

        let mut services = MockServiceRepo::new();

        services
            .expect_get()
            .once()
            .with(eq(id))
            .once()
            .returning(|_| Box::pin(ready(Ok(Some(Service::default())))));

        let app = router().with_state(ApiState {
            env: Environment::default(),
            repos: Repos {
                services: Arc::new(services),
                keys: Arc::new(MockKeyRepo::new()),
                definitions: Arc::new(MockDefinitionRepo::new()),
            },
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/services/{id}"))
                    .body(Body::from(()))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        Ok(())
    }
}
