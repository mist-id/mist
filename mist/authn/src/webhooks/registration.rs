use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use uuid::Uuid;

use super::Meta;

#[derive(Serialize, Deserialize)]
pub(crate) struct Request {
    pub(crate) id: Uuid,
    pub(crate) identifier: String,
    pub(crate) profile: Map<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Response {
    pub(crate) meta: Meta,
    pub(crate) complete: bool,
}
