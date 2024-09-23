use std::sync::Arc;

use axum::extract::FromRef;
use mist_common::env::Environment;
use mist_db::repos::{keys::KeyRepo, services::ServiceRepo};

#[derive(Clone)]
pub(crate) struct Repos {
    pub(crate) services: Arc<dyn ServiceRepo>,
    pub(crate) keys: Arc<dyn KeyRepo>,
}

#[derive(Clone)]
pub(crate) struct ApiState {
    pub(crate) env: Environment,
    pub(crate) repos: Repos,
}

impl FromRef<ApiState> for () {
    fn from_ref(_: &ApiState) -> Self {}
}
