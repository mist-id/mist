use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_garde::WithValidation;
use common::Result;
use db::models::service::{ServiceId, UpdateService};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::state::ApiState;

#[derive(Deserialize, IntoParams)]
pub(crate) struct PathParams {
    id: ServiceId,
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[schema(as = UpdateServicePayload)]
pub(crate) struct Payload {
    #[garde(ascii, length(min = 3, max = 25))]
    name: Option<String>,
    #[garde(url)]
    redirect_url: Option<String>,
    #[garde(url)]
    logout_url: Option<String>,
    #[garde(url)]
    webhook_url: Option<String>,
}

#[utoipa::path(
    tags = ["Services"],
    summary = "Update service",
    put,
    path = "/{id}",
    params(PathParams),
    request_body = UpdateServicePayload,
    responses(
        (status = 200, body = Service),
        (status = 404)
    )
)]
pub(crate) async fn update_handler(
    State(state): State<ApiState>,
    Path(path): Path<PathParams>,
    WithValidation(payload): WithValidation<Json<Payload>>,
) -> Result<impl IntoResponse> {
    let service = state
        .repos
        .services
        .update(
            &path.id,
            &UpdateService::builder()
                .maybe_name(payload.name.clone())
                .maybe_redirect_url(payload.redirect_url.clone())
                .maybe_logout_url(payload.logout_url.clone())
                .maybe_webhook_url(payload.webhook_url.clone())
                .build(),
        )
        .await?;

    Ok(Json(service))
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
        repos::{keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use tower::ServiceExt;

    use super::*;

    use crate::{handlers::services::router, state::Repos};

    #[tokio::test]
    async fn updates() -> Result<()> {
        let id = ServiceId::new();

        let mut services = MockServiceRepo::new();

        services
            .expect_update()
            .with(
                eq(id),
                eq(UpdateService::builder().maybe_name("ACME".into()).build()),
            )
            .once()
            .returning(|_, _| Box::pin(ready(Ok(Service::default()))));

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
