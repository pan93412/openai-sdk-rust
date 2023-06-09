//! Models.
//!
//! List and describe the various models available in the API.
//! You can refer to the [Models](https://platform.openai.com/docs/models/overview)
//! documentation to understand what models are available
//! and the differences between them.

use crate::structure::{
    models::{ModelEntry, Models},
    response::RespResult,
};

pub trait ModelFeature {
    type Error: std::error::Error;

    /// List models.
    ///
    /// Lists the currently available models, and provides basic information
    /// about each one such as the owner and availability.
    async fn models(&self) -> RespResult<Models, Self::Error>;

    /// Retrieve model.
    ///
    /// Retrieves a model instance, providing basic information
    /// about the model such as the owner and permissioning.
    ///
    /// ## Parameters
    ///
    /// - `model_id`: The ID of the model to use for this request.
    async fn model(&self, model_id: &str) -> RespResult<ModelEntry, Self::Error>;
}
