use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use mist_common::Result;
use mist_db::models::service::ServiceId;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::state::ApiState;

#[derive(Deserialize, IntoParams)]
pub(crate) struct PathParams {
    id: ServiceId,
}

#[utoipa::path(
    tags = ["Services"],
    summary = "Delete service",
    delete,
    path = "/{id}",
    params(PathParams),
    responses(
        (status = 200, body = Service),
        (status = 404)
    )
)]
pub(crate) async fn destroy_handler(
    State(state): State<ApiState>,
    Path(path): Path<PathParams>,
) -> Result<impl IntoResponse> {
    let service = state.repos.services.destroy(&path.id).await?;

    Ok(Json(service))
}

#[cfg(test)]
mod tests {
    use std::{future::ready, sync::Arc};

    use axum::{
        body::Body,
        extract::Request,
        http::{self, StatusCode},
    };
    use mist_db::{
        models::service::Service,
        repos::{keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use tower::ServiceExt;

    use super::*;

    use crate::{handlers::services::router, state::Repos};

    #[tokio::test]
    async fn destroys() -> Result<()> {
        let id = ServiceId::new();

        let mut services = MockServiceRepo::new();

        services
            .expect_destroy()
            .with(eq(id))
            .once()
            .returning(|_| Box::pin(ready(Ok(Service::default()))));

        let app = router().with_state(ApiState {
            env: mist_common::env::Environment::default(),
            repos: Repos {
                services: Arc::new(services),
                keys: Arc::new(MockKeyRepo::new()),
            },
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(format!("/services/{id}"))
                    .body(Body::from(()))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        Ok(())
    }
}
