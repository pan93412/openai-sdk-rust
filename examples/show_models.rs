use chatgpt_basic_api::{features::models::ModelFeature, openai::OpenAIBuilder};

#[tokio::main]
async fn main() {
    let token = std::env::var("OPENAI_TOKEN").expect("should specify OPENAI_TOKEN");

    // show_models [model]
    let model = std::env::args().nth(1);

    let openai = OpenAIBuilder::new()
        .token(&token)
        .build()
        .expect("openai should buildable");

    if let Some(model) = model {
        let model = openai.model(&model).await.expect("model should be fetched");

        println!("{model:#?}");
    } else {
        let models = openai.models().await.expect("models should be fetched");
        println!("{models:#?}");
    }
}
