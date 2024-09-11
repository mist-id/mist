use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use common::Result;
use serde::Deserialize;

use crate::state::ApiState;

#[derive(Deserialize)]
pub(crate) struct ListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub(crate) async fn handler(
    State(state): State<ApiState>,
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse> {
    let limit = query.limit.unwrap_or(10);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    let response = Json(
        state
            .repos
            .services
            .list(limit as i64, offset as i64)
            .await?,
    );

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
        models::service::Service,
        repos::{definitions::MockDefinitionRepo, keys::MockKeyRepo, services::MockServiceRepo},
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
                definitions: Arc::new(MockDefinitionRepo::new()),
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
