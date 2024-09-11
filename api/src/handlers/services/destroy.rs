use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::Result;
use serde::Deserialize;
use uuid::Uuid;

use crate::state::ApiState;

#[derive(Deserialize)]
pub(crate) struct DestroyPath {
    id: Uuid,
}

pub(crate) async fn handler(
    State(state): State<ApiState>,
    Path(path): Path<DestroyPath>,
) -> Result<impl IntoResponse> {
    let response = Json(state.repos.services.destroy(&path.id).await?);

    Ok(response)
}

#[cfg(test)]
mod tests {
    use std::{future::ready, sync::Arc};

    use axum::{
        body::Body,
        extract::Request,
        http::{self, StatusCode},
    };
    use db::{
        models::service::Service,
        repos::{definitions::MockDefinitionRepo, keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use tower::ServiceExt;

    use super::*;

    use crate::{handlers::services::router, state::Repos};

    #[tokio::test]
    async fn destroys() -> Result<()> {
        let id = Uuid::new_v4();

        let mut services = MockServiceRepo::new();

        services
            .expect_destroy()
            .with(eq(id))
            .once()
            .returning(|_| Box::pin(ready(Ok(Service::default()))));

        let app = router().with_state(ApiState {
            env: common::env::Environment::default(),
            repos: Repos {
                services: Arc::new(services),
                keys: Arc::new(MockKeyRepo::new()),
                definitions: Arc::new(MockDefinitionRepo::new()),
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
