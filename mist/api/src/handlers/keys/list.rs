use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use mist_common::Result;
use mist_db::models::service::ServiceId;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::state::ApiState;

#[derive(Serialize, Deserialize, IntoParams)]
pub(crate) struct PathParams {
    service_id: ServiceId,
}

#[derive(Deserialize, IntoParams)]
pub(crate) struct QueryParams {
    pub is_active: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[utoipa::path(
    tags = ["Keys"],
    summary = "List keys",
    get,
    path = "",
    params(PathParams, QueryParams),
    responses(
        (status = 200, body = Vec<Key>)
    )
)]
pub(crate) async fn list_handler(
    State(state): State<ApiState>,
    Path(path): Path<PathParams>,
    Query(query): Query<QueryParams>,
) -> Result<impl IntoResponse> {
    let limit = query.limit.unwrap_or(10);
    let offset = (query.page.unwrap_or(1) - 1) * limit;
    let is_active = query.is_active.unwrap_or(true);

    let keys = state
        .repos
        .keys
        .list(&path.service_id, is_active, limit as i64, offset as i64)
        .await?;

    Ok(Json(keys))
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
        models::key::Key,
        repos::{keys::MockKeyRepo, services::MockServiceRepo},
    };
    use tower::ServiceExt;

    use super::*;

    use crate::{handlers::keys::router, state::Repos};

    #[tokio::test]
    async fn lists() -> Result<()> {
        let service_id = ServiceId::new();

        let mut keys = MockKeyRepo::new();

        keys.expect_list()
            .once()
            .returning(|_, _, _, _| Box::pin(ready(Ok(vec![Key::default()]))));

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
                    .uri(format!("/services/{service_id}/keys"))
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(()))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        Ok(())
    }
}
