//! The implementation of [`ModelFeature`].

use crate::{
    features::models::ModelFeature,
    openai::{openai_uri, OpenAI},
    structure::models::{ModelEntry, Models},
};

impl ModelFeature for OpenAI {
    type Error = crate::openai::Error;

    async fn models(&self) -> Result<Models, Self::Error> {
        let url = openai_uri("v1", "models")?;

        let response = self.client.get(url).send().await?;
        let models = response.json().await?;

        Ok(models)
    }

    async fn model(&self, model_id: &str) -> Result<ModelEntry, Self::Error> {
        let url = openai_uri("v1", "model")?.join(model_id)?;

        let response = self.client.get(url).send().await?;
        let models = response.json().await?;

        Ok(models)
    }
}
