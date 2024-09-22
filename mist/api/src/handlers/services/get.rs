use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::Result;
use db::models::service::ServiceId;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::state::ApiState;

#[derive(Deserialize, IntoParams)]
pub(crate) struct PathParams {
    id: ServiceId,
}

#[utoipa::path(
    tags = ["Services"],
    summary = "Get service",
    get,
    path = "/{id}",
    params(PathParams),
    responses(
        (status = 200, body = Service),
        (status = 404)
    )
)]
pub(crate) async fn get_handler(
    State(state): State<ApiState>,
    Path(path): Path<PathParams>,
) -> Result<impl IntoResponse> {
    let service = state.repos.services.get(&path.id).await?;

    Ok(Json(service))
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
        models::service::Service,
        repos::{keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use tower::ServiceExt;

    use super::*;

    use crate::{handlers::services::router, state::Repos};

    #[tokio::test]
    async fn gets() -> Result<()> {
        let id = ServiceId::new();

        let mut services = MockServiceRepo::new();

        services
            .expect_get()
            .once()
            .with(eq(id))
            .once()
            .returning(|_| Box::pin(ready(Ok(Service::default()))));

        let app = router().with_state(ApiState {
            env: Environment::default(),
            repos: Repos {
                services: Arc::new(services),
                keys: Arc::new(MockKeyRepo::new()),
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
