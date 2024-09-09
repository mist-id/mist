use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_garde::WithValidation;
use common::error::Error;
use db::models::key::{CreateKey, KeyKind};
use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::ApiState;

#[derive(Serialize, Deserialize)]
pub(crate) struct CreatePath {
    service_id: Uuid,
}

#[derive(Serialize, Deserialize, Validate)]
pub(crate) struct CreateBody {
    #[garde(skip)]
    kind: KeyKind,
    #[serde(rename = "key")]
    #[garde(ascii, length(bytes, min = 64, max = 64))]
    value: String,
    #[garde(range(min = 1))]
    priority: Option<i32>,
}

pub(crate) async fn handler(
    State(state): State<ApiState>,
    Path(path): Path<CreatePath>,
    WithValidation(body): WithValidation<Json<CreateBody>>,
) -> Result<impl IntoResponse, Error> {
    let key = state
        .repos
        .keys
        .create(
            &state.env.master_key,
            &CreateKey::new(
                body.kind.clone(),
                body.value.clone(),
                body.priority.unwrap_or(1),
                path.service_id,
            ),
        )
        .await?;

    Ok((StatusCode::CREATED, Json(key)))
}

#[cfg(test)]
mod tests {
    use std::{future::ready, sync::Arc};

    use anyhow::Result;
    use axum::{body::Body, extract::Request, http};
    use common::env::Environment;
    use db::{
        models::key::Key,
        repos::{definitions::MockDefinitionRepo, keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use secstr::SecVec;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{handlers::keys::router, state::Repos};

    use super::*;

    #[tokio::test]
    async fn creates() -> Result<()> {
        let master_key =
            SecVec::from("57a9f41af0c50e8b6560e5b65ad3b4111fa78a591ab6ce2f87d0b8c18d8ecd9");
        let service_id = Uuid::new_v4();
        let key = "625c3ba938c1947e85c6eb36af959e22239ff964a90fe176a9f6581329d9b827";

        let mut keys = MockKeyRepo::new();

        keys.expect_create()
            .with(
                eq(master_key.clone()),
                eq(CreateKey::new(KeyKind::Token, key, 1, service_id)),
            )
            .once()
            .returning(|_, _| Box::pin(ready(Ok(Key::default()))));

        let app = router().with_state(ApiState {
            env: Environment {
                master_key,
                ..Default::default()
            },
            repos: Repos {
                services: Arc::new(MockServiceRepo::new()),
                keys: Arc::new(keys),
                definitions: Arc::new(MockDefinitionRepo::new()),
            },
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri(format!("/services/{service_id}/keys"))
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(format!(
                        r#"
                        {{
                            "kind": "token",
                            "key": "{key}"
                        }}
                    "#,
                    )))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::CREATED);

        Ok(())
    }
}
