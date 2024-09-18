use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use common::Result;
use db::models::key::KeyKind;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;
use uuid::Uuid;

use crate::state::ApiState;

#[derive(Serialize, Deserialize, IntoParams)]
pub(crate) struct PathParams {
    service_id: Uuid,
}

#[derive(Serialize, Deserialize, IntoParams)]
pub(crate) struct QueryParams {
    kind: KeyKind,
}

#[utoipa::path(
    tags = ["Keys"],
    summary = "Get preferred key",
    get,
    path = "/services/{service_id}/keys/preferred",
    params(PathParams, QueryParams),
    responses(
        (status = 200, body = Key),
        (status = 404)
    )
)]
pub(crate) async fn preferred_handler(
    State(state): State<ApiState>,
    Path(path): Path<PathParams>,
    Query(query): Query<QueryParams>,
) -> Result<impl IntoResponse> {
    let key = state
        .repos
        .keys
        .preferred(&path.service_id, &query.kind)
        .await?;

    Ok(Json(key))
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
        let service_id = Uuid::new_v4();

        let mut keys = MockKeyRepo::new();

        keys.expect_preferred()
            .with(eq(service_id), eq(KeyKind::Token))
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
                    .method(http::Method::GET)
                    .uri(format!("/services/{service_id}/keys/preferred?kind=token"))
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(()))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        Ok(())
    }
}
