use db::models::user::UserId;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::Meta;

#[derive(Serialize, Deserialize)]
pub(crate) struct Request {
    pub(crate) id: UserId,
    pub(crate) identifier: String,
    pub(crate) profile: Map<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Response {
    pub(crate) meta: Meta,
    pub(crate) complete: bool,
}
