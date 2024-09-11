use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_garde::WithValidation;
use common::Result;
use db::models::service::UpdateService;
use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::ApiState;

#[derive(Deserialize)]
pub(crate) struct UpdatePath {
    id: Uuid,
}

#[derive(Serialize, Deserialize, Validate)]
pub(crate) struct UpdateBody {
    #[garde(ascii, length(min = 3, max = 25))]
    name: Option<String>,
    #[garde(url)]
    redirect_url: Option<String>,
    #[garde(url)]
    webhook_url: Option<String>,
}

pub(crate) async fn handler(
    State(state): State<ApiState>,
    Path(path): Path<UpdatePath>,
    WithValidation(body): WithValidation<Json<UpdateBody>>,
) -> Result<impl IntoResponse> {
    let response = Json(
        state
            .repos
            .services
            .update(
                &path.id,
                &UpdateService::new(
                    body.name.clone(),
                    body.redirect_url.clone(),
                    body.webhook_url.clone(),
                ),
            )
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
        models::service::{Service, UpdateService},
        repos::{definitions::MockDefinitionRepo, keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use tower::ServiceExt;

    use super::*;

    use crate::{handlers::services::router, state::Repos};

    #[tokio::test]
    async fn updates() -> Result<()> {
        let id = Uuid::new_v4();

        let mut services = MockServiceRepo::new();

        services
            .expect_update()
            .with(
                eq(id),
                eq(UpdateService::new(Some("ACME".into()), None, None)),
            )
            .once()
            .returning(|_, _| Box::pin(ready(Ok(Service::default()))));

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
                    .method(http::Method::PUT)
                    .uri(format!("/services/{id}"))
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"
                        {
                            "name": "ACME"
                        }
                    "#,
                    ))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        Ok(())
    }
}
