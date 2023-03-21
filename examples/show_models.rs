use chatgpt_basic_api::{features::models::ModelFeature, openai::OpenAIBuilder};

#[tokio::main]
async fn main() {
    let token = std::env::var("OPENAI_TOKEN").expect("should specify OPENAI_TOKEN");

    let openai = OpenAIBuilder::new()
        .token(&token)
        .build()
        .expect("openai should buildable");

    let models = openai.models().await.expect("models should be fetched");

    println!("{models:#?}");
}
