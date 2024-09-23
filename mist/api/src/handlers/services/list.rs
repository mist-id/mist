use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use mist_common::Result;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::state::ApiState;

#[derive(Deserialize, IntoParams)]
pub(crate) struct QueryParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[utoipa::path(
    tags = ["Services"],
    summary = "List services",
    get,
    path = "",
    params(QueryParams),
    responses(
        (status = 200, body = Vec<Service>)
    )
)]
pub(crate) async fn list_handler(
    State(state): State<ApiState>,
    Query(query): Query<QueryParams>,
) -> Result<impl IntoResponse> {
    let limit = query.limit.unwrap_or(10);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    let services = state
        .repos
        .services
        .list(limit as i64, offset as i64)
        .await?;

    Ok(Json(services))
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
        models::service::Service,
        repos::{keys::MockKeyRepo, services::MockServiceRepo},
    };
    use tower::ServiceExt;

    use super::*;

    use crate::{
        handlers::services::router,
        state::{ApiState, Repos},
    };

    #[tokio::test]
    async fn lists() -> Result<()> {
        let mut services = MockServiceRepo::new();

        services
            .expect_list()
            .once()
            .returning(|_, _| Box::pin(ready(Ok(vec![Service::default(), Service::default()]))));

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
                    .uri("/services")
                    .body(Body::from(()))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        Ok(())
    }
}
