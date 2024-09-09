use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use common::error::Error;
use db::models::key::UpdateKey;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::ApiState;

#[derive(Serialize, Deserialize)]
pub(crate) struct UpdatePath {
    id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct UpdateBody {
    #[serde(rename = "active")]
    is_active: Option<bool>,
}

pub(crate) async fn handler(
    State(state): State<ApiState>,
    Path(path): Path<UpdatePath>,
    Json(body): Json<UpdateBody>,
) -> Result<impl IntoResponse, Error> {
    let key = state
        .repos
        .keys
        .update(&path.id, &UpdateKey::new(body.is_active))
        .await?;

    Ok((StatusCode::OK, Json(key)))
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
        models::key::{Key, UpdateKey},
        repos::{definitions::MockDefinitionRepo, keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use tower::ServiceExt;

    use super::*;

    use crate::{handlers::keys::router, state::Repos};

    #[tokio::test]
    async fn updates() -> Result<()> {
        let id = Uuid::new_v4();

        let mut keys = MockKeyRepo::new();

        keys.expect_update()
            .with(eq(id), eq(UpdateKey::new(Some(true))))
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
                    .method(http::Method::PUT)
                    .uri(format!("/keys/{id}"))
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"
                        {
                            "active": true
                        }
                    "#,
                    ))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        Ok(())
    }
}
