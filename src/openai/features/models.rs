//! The implementation of [`ModelFeature`].

use crate::{
    features::models::ModelFeature,
    openai::{openai_uri, OpenAI},
    structure::{
        models::{ModelEntry, Models},
        response::RespResult,
    },
};

impl ModelFeature for OpenAI {
    type Error = crate::openai::Error;

    async fn models(&self) -> RespResult<Models, Self::Error> {
        let url = openai_uri("v1", "models")?;

        let response = self.client.get(url).send().await?;
        let models = response.json().await?;

        Ok(models)
    }

    async fn model(&self, model_id: &str) -> RespResult<ModelEntry, Self::Error> {
        let url = {
            let mut u = openai_uri("v1", "models")?;

            u.path_segments_mut()
                .expect("not a valid base")
                .push(model_id);

            u
        };

        let response = self.client.get(url).send().await?;
        let models = response.json().await?;

        Ok(models)
    }
}
