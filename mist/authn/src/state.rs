use std::sync::Arc;

use async_nats::{jetstream::Context, Client};
use common::env::Environment;
use db::repos::{
    identifiers::IdentifierRepo, keys::KeyRepo, services::ServiceRepo, users::UserRepo,
};
use fred::prelude::RedisClient;

#[derive(Clone)]
pub(crate) struct Repos {
    pub(crate) keys: Arc<dyn KeyRepo>,
    pub(crate) services: Arc<dyn ServiceRepo>,
    pub(crate) users: Arc<dyn UserRepo>,
    pub(crate) identifiers: Arc<dyn IdentifierRepo>,
}

#[derive(Clone)]
pub(crate) struct AuthnState {
    pub(crate) env: Environment,
    pub(crate) repos: Repos,
    pub(crate) redis: RedisClient,
    pub(crate) nats: Client,
    pub(crate) jetstream: Context,
}
