use std::env;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use futures::Stream;
use futures::StreamExt;
use rig::agent::Agent;
use rig::agent::MultiTurnStreamItem;
use rig::completion::Message;
use rig::providers::gemini;
use rig::providers::ollama;
use rig::providers::openai::responses_api::ResponsesCompletionModel;
use rig::streaming::StreamedAssistantContent;
use rig::streaming::StreamingChat;

use super::any_agent::AnyAgent;
use super::research_tool::ResearchTool;
use super::{
    create_gemini_router_agent, create_ollama_router_agent, create_openai_router_agent,
    default_model, ChatAgent, ChatStreamEvent, WebFetch,
};

/// A RouterAgent that orchestrates specialized tools (including a ResearchTool sub-agent).
///
/// Unlike AnyAgent which directly provides web tools to one LLM, RouterAgent adds a
/// ResearchTool whose inner agent handles deep multi-step investigation. The outer LLM
/// acts as a dispatcher, choosing between quick web_search, full research_tool, or
/// direct answers based on the user's intent.
pub enum RouterAgent {
    Ollama(Agent<ollama::CompletionModel>),
    Gemini(Agent<gemini::completion::CompletionModel>),
    OpenAi(Agent<ResponsesCompletionModel>),
}

impl RouterAgent {
    /// Create a RouterAgent from environment configuration.
    ///
    /// Builds two-level agent hierarchy:
    /// 1. Inner AnyAgent (web_search + web_fetch + pdf_read) for deep research
    /// 2. Outer RouterAgent with ResearchTool wrapping the inner agent
    pub fn from_env() -> Self {
        let provider = env::var("LLM_PROVIDER").unwrap_or_else(|_| "ollama".to_string());
        let model = env::var("LLM_MODEL").unwrap_or_else(|_| default_model(&provider).to_string());

        // Build inner research agent (shares the same provider/model)
        let web_fetch = WebFetch::new();
        let inner_agent = AnyAgent::from_env(web_fetch.clone());
        let research_tool = ResearchTool::new(Arc::new(inner_agent));

        match provider.as_str() {
            "openai" => {
                let api_key =
                    env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY required for OpenAI");
                Self::OpenAi(create_openai_router_agent(
                    &api_key,
                    &model,
                    research_tool,
                    web_fetch,
                ))
            }
            "gemini" => {
                let api_key =
                    env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY required for Gemini");
                Self::Gemini(create_gemini_router_agent(
                    &api_key,
                    &model,
                    research_tool,
                    web_fetch,
                ))
            }
            _ => Self::Ollama(create_ollama_router_agent(&model, research_tool, web_fetch)),
        }
    }

    fn map_stream<R: Send + 'static>(
        stream: rig::agent::StreamingResult<R>,
    ) -> Pin<Box<dyn Stream<Item = ChatStreamEvent> + Send>> {
        let mapped = stream.filter_map(|item| async {
            match item {
                Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(
                    text,
                ))) => Some(ChatStreamEvent::TextDelta(text.text)),
                Ok(MultiTurnStreamItem::StreamAssistantItem(
                    StreamedAssistantContent::ToolCall { tool_call, .. },
                )) => Some(ChatStreamEvent::ToolCall {
                    name: tool_call.function.name,
                }),
                Ok(MultiTurnStreamItem::FinalResponse(_)) => Some(ChatStreamEvent::Done),
                Err(e) => Some(ChatStreamEvent::Error(e.to_string())),
                _ => None,
            }
        });
        Box::pin(mapped)
    }
}

#[async_trait]
impl ChatAgent for RouterAgent {
    async fn stream_chat(
        &self,
        prompt: &str,
        history: Vec<Message>,
    ) -> Pin<Box<dyn Stream<Item = ChatStreamEvent> + Send>> {
        match self {
            RouterAgent::Ollama(agent) => {
                Self::map_stream(agent.stream_chat(prompt, history).await)
            }
            RouterAgent::Gemini(agent) => {
                Self::map_stream(agent.stream_chat(prompt, history).await)
            }
            RouterAgent::OpenAi(agent) => {
                Self::map_stream(agent.stream_chat(prompt, history).await)
            }
        }
    }
}
