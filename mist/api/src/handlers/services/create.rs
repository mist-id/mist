use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_garde::WithValidation;
use garde::Validate;
use mist_common::Result;
use mist_db::models::{
    definition::{CreateDefinition, Value},
    service::CreateService,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::state::ApiState;

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[schema(as = CreateServicePayload)]
pub(crate) struct Payload {
    #[garde(ascii, length(min = 3, max = 25))]
    name: String,
    #[garde(url)]
    redirect_url: String,
    #[garde(url)]
    logout_url: String,
    #[garde(url)]
    webhook_url: String,
    #[garde(skip)]
    profile: Option<Value>,
}

#[utoipa::path(
    tags = ["Services"],
    summary = "Create service",
    post,
    path = "",
    request_body = CreateServicePayload,
    responses(
        (status = 201, body = Service)
    )
)]
pub(crate) async fn create_handler(
    State(state): State<ApiState>,
    WithValidation(payload): WithValidation<Json<Payload>>,
) -> Result<impl IntoResponse> {
    let service = state
        .repos
        .services
        .create(
            &state.env.master_key,
            &CreateService::builder()
                .name(&payload.name)
                .redirect_url(&payload.redirect_url)
                .logout_url(&payload.logout_url)
                .webhook_url(&payload.webhook_url)
                .build(),
            &payload.profile.as_ref().map(|profile| {
                CreateDefinition::builder()
                    .name("default")
                    .value(profile.clone())
                    .is_default(true)
                    .build()
            }),
        )
        .await?;

    Ok((StatusCode::CREATED, Json(service)))
}

#[cfg(test)]
mod tests {
    use std::{future::ready, sync::Arc};

    use axum::{body::Body, extract::Request, http};
    use mist_common::env::Environment;
    use mist_db::{
        models::service::{Service, ServiceId},
        repos::{keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use secstr::SecVec;
    use tower::ServiceExt;

    use crate::{handlers::services::router, state::Repos};

    use super::*;

    #[tokio::test]
    async fn creates() -> Result<()> {
        let master_key =
            SecVec::from("d7456538654523fa190c520767911eb965c561b5d0eed95cd4d8250ec9105f66");
        let service_id = ServiceId::new();

        let mut services = MockServiceRepo::new();

        services
            .expect_create()
            .with(
                eq(master_key.clone()),
                eq(CreateService::builder()
                    .name("ACME")
                    .redirect_url("https://ac.me")
                    .logout_url("https://ac.me")
                    .webhook_url("https://ac.me/hooks")
                    .build()),
                eq(Some(
                    CreateDefinition::builder()
                        .name("default")
                        .value(Value::default())
                        .is_default(true)
                        .build(),
                )),
            )
            .once()
            .returning(move |_, _, _| {
                Box::pin(ready(Ok(Service {
                    id: service_id,
                    ..Default::default()
                })))
            });

        let app = router().with_state(ApiState {
            env: Environment {
                master_key,
                ..Default::default()
            },
            repos: Repos {
                services: Arc::new(services),
                keys: Arc::new(MockKeyRepo::new()),
            },
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/services")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"
                        {
                            "name": "ACME",
                            "redirect_url": "https://ac.me",
                            "logout_url": "https://ac.me",
                            "webhook_url": "https://ac.me/hooks",
                            "profile": { "fields": [] }
                        }
                    "#,
                    ))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::CREATED);

        Ok(())
    }
}
