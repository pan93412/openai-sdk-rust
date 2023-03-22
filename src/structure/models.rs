//! The data structure of [`crate::features::models`].

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Models {
    pub data: Vec<ModelEntry>,
    pub object: String,

    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Deserialize)]
pub struct ModelEntry {
    pub id: String,
    pub object: String,
    pub owned_by: String,

    // fixme
    pub permission: Value,

    #[serde(flatten)]
    pub extra: Value,
}
