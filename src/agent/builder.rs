use rig::agent::Agent;
use rig::client::{CompletionClient, Nothing};
use rig::providers::openai::responses_api::ResponsesCompletionModel;
use rig::providers::{gemini, ollama, openai};

use super::{PdfRead, WebFetch, WebSearch};

const PREAMBLE: &str =
    "You are a research assistant that helps users gather and summarize information from the web";

/// Create an Ollama-based research agent
pub fn create_ollama_agent(model: &str, web_fetch: WebFetch) -> Agent<ollama::CompletionModel> {
    let client = ollama::Client::builder()
        .api_key(Nothing)
        .build()
        .expect("Failed to create Ollama client");

    client
        .agent(model)
        .preamble(PREAMBLE)
        .tool(web_fetch)
        .tool(WebSearch)
        .tool(PdfRead)
        .build()
}

/// Create a Gemini-based research agent
pub fn create_gemini_agent(
    api_key: &str,
    model: &str,
    web_fetch: WebFetch,
) -> Agent<gemini::completion::CompletionModel> {
    let client = gemini::Client::new(api_key).expect("Failed to create Gemini client");

    client
        .agent(model)
        .preamble(PREAMBLE)
        .tool(web_fetch)
        .tool(WebSearch)
        .tool(PdfRead)
        .build()
}

/// Create an OpenAI-based research agent
pub fn create_openai_agent(
    api_key: &str,
    model: &str,
    web_fetch: WebFetch,
) -> Agent<ResponsesCompletionModel> {
    let client: rig::client::Client<openai::OpenAIResponsesExt> =
        openai::Client::new(api_key).expect("Failed to create OpenAI client");
    client
        .agent(model)
        .preamble(PREAMBLE)
        .tool(web_fetch)
        .tool(WebSearch)
        .tool(PdfRead)
        .build()
}

/// Get the default model name for a given provider
pub fn default_model(provider: &str) -> &'static str {
    match provider {
        "gemini" => gemini::completion::GEMINI_2_5_FLASH,
        "openai" => openai::completion::GPT_4_1_MINI,
        _ => "qwen3",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use rig::completion::Prompt;
    use rig::providers::{gemini, openai};

    #[test]
    fn test_default_model_ollama() {
        assert_eq!(default_model("ollama"), "qwen3");
    }

    #[test]
    fn test_default_model_gemini() {
        assert_eq!(default_model("gemini"), "gemini-2.5-flash");
    }

    #[test]
    fn test_default_model_openai() {
        assert_eq!(default_model("openai"), "gpt-4.1-mini");
    }

    #[test]
    fn test_default_model_unknown_fallback() {
        assert_eq!(default_model("unknown"), "qwen3");
    }

    #[tokio::test]
    #[ignore]
    async fn test_ollama_agent_with_web_fetch() {
        let agent = create_ollama_agent("qwen3", WebFetch::new());
        let response = agent
            .prompt("Fetch https://example.com and **summarize** it shortly")
            .await
            .unwrap();

        println!("{}", response);
        assert!(!response.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_gemini_agent_with_web_fetch() {
        dotenv().ok();

        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY required");
        let agent = create_gemini_agent(
            &api_key,
            gemini::completion::GEMINI_2_5_FLASH,
            WebFetch::new(),
        );
        let response = agent
            .prompt("Fetch https://example.com and **summarize** it shortly")
            .await
            .unwrap();

        println!("{}", response);
        assert!(!response.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_openai_agent_with_web_fetch() {
        dotenv().ok();

        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY required");
        let agent =
            create_openai_agent(&api_key, openai::completion::GPT_4_1_MINI, WebFetch::new());
        let response = agent
            .prompt("Fetch https://example.com and **summarize** it shortly")
            .await
            .unwrap();

        println!("{}", response);
        assert!(!response.is_empty());
    }
}
