use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use common::error::Error;
use db::models::key::KeyKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::ApiState;

#[derive(Serialize, Deserialize)]
pub(crate) struct PreferredPath {
    service_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct PreferredQuery {
    kind: KeyKind,
}

pub(crate) async fn handler(
    State(state): State<ApiState>,
    Path(path): Path<PreferredPath>,
    Query(query): Query<PreferredQuery>,
) -> Result<impl IntoResponse, Error> {
    let response = Json(
        state
            .repos
            .keys
            .preferred(&path.service_id, &query.kind)
            .await?,
    );

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
        models::key::Key,
        repos::{definitions::MockDefinitionRepo, keys::MockKeyRepo, services::MockServiceRepo},
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
                definitions: Arc::new(MockDefinitionRepo::new()),
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
