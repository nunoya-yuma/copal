use rig::agent::Agent;
use rig::client::{CompletionClient, Nothing};
use rig::providers::openai::responses_api::ResponsesCompletionModel;
use rig::providers::{gemini, ollama, openai};

use super::research_tool::ResearchTool;
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
        .default_max_turns(10)
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
        .default_max_turns(10)
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
        .default_max_turns(10)
        .tool(web_fetch)
        .tool(WebSearch)
        .tool(PdfRead)
        .build()
}

/// System prompt that defines the RouterAgent's tool-selection strategy.
///
/// The router sees all tools (research_tool, web_search, web_fetch, pdf_read)
/// and must choose the right one based on the user's intent:
/// - Deep investigation → research_tool
/// - Quick lookup      → web_search
/// - Specific URL      → web_fetch
/// - PDF document      → pdf_read
/// - General chat      → no tool
const ROUTER_PREAMBLE: &str = "\
You are an intelligent assistant that routes user requests to the most appropriate tool.\n\
\n\
Available tools and when to use them:\n\
- research_tool: Use for in-depth research requiring multiple sources. \
  Triggers a full investigation across web pages and returns a structured report. \
  Use when the user wants thorough analysis, comparisons, or comprehensive understanding.\n\
- web_search: Use for quick factual lookups, current events, or brief information needs \
  that don't require reading full pages.\n\
- web_fetch: Use when the user provides a specific URL to read or when you need \
  to retrieve a known page.\n\
- pdf_read: Use when the user provides a path to a PDF file to read.\n\
\n\
For general conversation, questions you can answer from your knowledge, or simple \
clarifications — respond directly without using any tool.";

/// Create an Ollama-based router agent with all routing tools
pub fn create_ollama_router_agent(
    model: &str,
    research_tool: ResearchTool,
    web_fetch: WebFetch,
) -> Agent<ollama::CompletionModel> {
    let client = ollama::Client::builder()
        .api_key(Nothing)
        .build()
        .expect("Failed to create Ollama client");

    client
        .agent(model)
        .preamble(ROUTER_PREAMBLE)
        .default_max_turns(10)
        .tool(research_tool)
        .tool(web_fetch)
        .tool(WebSearch)
        .tool(PdfRead)
        .build()
}

/// Create a Gemini-based router agent with all routing tools
pub fn create_gemini_router_agent(
    api_key: &str,
    model: &str,
    research_tool: ResearchTool,
    web_fetch: WebFetch,
) -> Agent<gemini::completion::CompletionModel> {
    let client = gemini::Client::new(api_key).expect("Failed to create Gemini client");

    client
        .agent(model)
        .preamble(ROUTER_PREAMBLE)
        .default_max_turns(10)
        .tool(research_tool)
        .tool(web_fetch)
        .tool(WebSearch)
        .tool(PdfRead)
        .build()
}

/// Create an OpenAI-based router agent with all routing tools
pub fn create_openai_router_agent(
    api_key: &str,
    model: &str,
    research_tool: ResearchTool,
    web_fetch: WebFetch,
) -> Agent<ResponsesCompletionModel> {
    let client: rig::client::Client<openai::OpenAIResponsesExt> =
        openai::Client::new(api_key).expect("Failed to create OpenAI client");

    client
        .agent(model)
        .preamble(ROUTER_PREAMBLE)
        .default_max_turns(10)
        .tool(research_tool)
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
