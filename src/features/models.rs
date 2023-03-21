//! Models.
//!
//! List and describe the various models available in the API.
//! You can refer to the [Models](https://platform.openai.com/docs/models/overview)
//! documentation to understand what models are available
//! and the differences between them.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Models {
    pub data: Vec<ModelEntry>,
    pub object: String,
}

#[derive(Debug, Deserialize)]
pub struct ModelEntry {
    pub id: String,
    pub object: String,
    pub owned_by: String,
    pub permission: Vec<String>,
}

pub trait ModelFeature {
    type Error: std::error::Error;

    /// List models.
    ///
    /// Lists the currently available models, and provides basic information
    /// about each one such as the owner and availability.
    async fn models(&self) -> Result<Models, Self::Error>;

    /// Retrieve model.
    ///
    /// Retrieves a model instance, providing basic information
    /// about the model such as the owner and permissioning.
    ///
    /// ## Parameters
    ///
    /// - `model_id`: The ID of the model to use for this request.
    async fn model(&self, model_id: &str) -> Result<ModelEntry, Self::Error>;
}
