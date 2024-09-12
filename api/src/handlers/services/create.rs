use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_garde::WithValidation;
use common::Result;
use db::models::{
    definition::{CreateDefinition, Value},
    service::CreateService,
};
use garde::Validate;
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
    profile: Value,
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
            &CreateService::builder()
                .name(&payload.name)
                .redirect_url(&payload.redirect_url)
                .logout_url(&payload.logout_url)
                .webhook_url(&payload.webhook_url)
                .build(),
        )
        .await?;

    state
        .repos
        .definitions
        .create(
            &CreateDefinition::builder()
                .name("default")
                .value(payload.profile.clone())
                .is_default(true)
                .service_id(service.id)
                .build(),
        )
        .await?;

    Ok((StatusCode::CREATED, Json(service)))
}

#[cfg(test)]
mod tests {
    use std::{future::ready, sync::Arc};

    use axum::{body::Body, extract::Request, http};
    use common::env::Environment;
    use db::{
        models::{definition::Definition, service::Service},
        repos::{definitions::MockDefinitionRepo, keys::MockKeyRepo, services::MockServiceRepo},
    };
    use mockall::predicate::*;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{handlers::services::router, state::Repos};

    use super::*;

    #[tokio::test]
    async fn creates() -> Result<()> {
        let service_id = Uuid::new_v4();

        let mut services = MockServiceRepo::new();
        let mut definitions = MockDefinitionRepo::new();

        services
            .expect_create()
            .with(eq(CreateService::builder()
                .name("ACME")
                .redirect_url("https://ac.me")
                .logout_url("https://ac.me")
                .webhook_url("https://ac.me/hooks")
                .build()))
            .once()
            .returning(move |_| {
                Box::pin(ready(Ok(Service {
                    id: service_id,
                    ..Default::default()
                })))
            });

        definitions
            .expect_create()
            .with(eq(CreateDefinition::builder()
                .name("default")
                .value(Value::default())
                .is_default(true)
                .service_id(service_id)
                .build()))
            .once()
            .returning(|_| Box::pin(ready(Ok(Definition::default()))));

        let app = router().with_state(ApiState {
            env: Environment::default(),
            repos: Repos {
                services: Arc::new(services),
                keys: Arc::new(MockKeyRepo::new()),
                definitions: Arc::new(definitions),
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
