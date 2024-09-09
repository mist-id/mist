use std::sync::Arc;

use axum::extract::FromRef;
use common::env::Environment;
use db::repos::{definitions::DefinitionRepo, keys::KeyRepo, services::ServiceRepo};

#[derive(Clone)]
pub(crate) struct Repos {
    pub(crate) services: Arc<dyn ServiceRepo>,
    pub(crate) keys: Arc<dyn KeyRepo>,
    pub(crate) definitions: Arc<dyn DefinitionRepo>,
}

#[derive(Clone)]
pub(crate) struct ApiState {
    pub(crate) env: Environment,
    pub(crate) repos: Repos,
}

impl FromRef<ApiState> for () {
    fn from_ref(_: &ApiState) -> Self {}
}
