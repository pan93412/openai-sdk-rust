pub use crate::openai::OpenAI;

pub struct ChatGPT<'a> {
    openai: &'a OpenAI,
    prompt: String,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    frequency_penalty: f32,
    presence_penalty: f32,
    stop: Vec<String>,
}

pub trait ChatGPTFeature {
    fn chatgpt_models(&self) -> Result<Models, Error>;
}
