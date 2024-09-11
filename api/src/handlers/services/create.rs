use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_garde::WithValidation;
use common::Result;
use db::models::{
    definition::{CreateDefinition, Value},
    service::CreateService,
};
use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::state::ApiState;

#[derive(Serialize, Deserialize, Validate)]
pub(crate) struct CreateBody {
    #[garde(ascii, length(min = 3, max = 25))]
    name: String,
    #[garde(url)]
    redirect_url: String,
    #[garde(url)]
    webhook_url: String,
    #[garde(skip)]
    profile: Value,
}

pub(crate) async fn handler(
    State(state): State<ApiState>,
    WithValidation(body): WithValidation<Json<CreateBody>>,
) -> Result<impl IntoResponse> {
    let service = state
        .repos
        .services
        .create(&CreateService::new(
            body.name.clone(),
            body.redirect_url.clone(),
            body.webhook_url.clone(),
        ))
        .await?;

    state
        .repos
        .definitions
        .create(&CreateDefinition::new(
            "default",
            body.profile.clone(),
            true,
            service.id,
        ))
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
            .with(eq(CreateService::new(
                "ACME",
                "https://ac.me",
                "https://ac.me/hooks",
            )))
            .once()
            .returning(move |_| {
                Box::pin(ready(Ok(Service {
                    id: service_id,
                    ..Default::default()
                })))
            });

        definitions
            .expect_create()
            .with(eq(CreateDefinition::new(
                "default",
                Value::default(),
                true,
                service_id,
            )))
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
