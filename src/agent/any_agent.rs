use std::env;
use std::pin::Pin;

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

use super::{
    create_gemini_agent, create_ollama_agent, create_openai_agent, default_model, WebFetch,
};

/// Provider-agnostic stream event emitted by `AnyAgent::stream_chat`.
/// Strips away the provider-specific response type `R` from rig-core's
/// `MultiTurnStreamItem<R>`, keeping only what consumers (SSE, CLI) need.
#[derive(Debug, Clone)]
pub enum ChatStreamEvent {
    /// A text fragment from the assistant's response
    TextDelta(String),
    /// The stream has completed successfully
    Done,
    /// An error occurred during streaming
    Error(String),
}

/// A type-erased agent that wraps any supported LLM provider.
/// Allows storing a single agent in shared state regardless of provider.
pub enum AnyAgent {
    Ollama(Agent<ollama::CompletionModel>),
    Gemini(Agent<gemini::completion::CompletionModel>),
    OpenAi(Agent<ResponsesCompletionModel>),
}

impl AnyAgent {
    /// Create an AnyAgent from environment configuration.
    /// Reads LLM_PROVIDER and LLM_MODEL env vars plus provider-specific API keys.
    pub fn from_env(web_fetch: WebFetch) -> Self {
        let provider = env::var("LLM_PROVIDER").unwrap_or_else(|_| "ollama".to_string());
        let model = env::var("LLM_MODEL").unwrap_or_else(|_| default_model(&provider).to_string());

        match provider.as_str() {
            "openai" => {
                let api_key =
                    env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY required for OpenAI");
                Self::OpenAi(create_openai_agent(&api_key, &model, web_fetch))
            }
            "gemini" => {
                let api_key =
                    env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY required for Gemini");
                Self::Gemini(create_gemini_agent(&api_key, &model, web_fetch))
            }
            _ => Self::Ollama(create_ollama_agent(&model, web_fetch)),
        }
    }

    /// Stream a chat response, converting provider-specific stream items
    /// into provider-agnostic `ChatStreamEvent`s.
    ///
    /// # Arguments
    /// * `prompt` - The user's message
    /// * `history` - Conversation history (cloned from ConversationHistory::to_vec())
    pub async fn stream_chat(
        &self,
        prompt: &str,
        history: Vec<Message>,
    ) -> Pin<Box<dyn Stream<Item = ChatStreamEvent> + Send>> {
        match self {
            AnyAgent::Ollama(agent) => Self::map_stream(agent.stream_chat(prompt, history).await),
            AnyAgent::Gemini(agent) => Self::map_stream(agent.stream_chat(prompt, history).await),
            AnyAgent::OpenAi(agent) => Self::map_stream(agent.stream_chat(prompt, history).await),
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
                Ok(MultiTurnStreamItem::FinalResponse(_)) => Some(ChatStreamEvent::Done),
                Err(e) => Some(ChatStreamEvent::Error(e.to_string())),
                _ => None, // tool calls etc. skip(don't yield)
            }
        });
        Box::pin(mapped)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_stream_chat_response() {
        let web_fetch = WebFetch::new();
        let agent = AnyAgent::from_env(web_fetch);

        let mut stream = agent.stream_chat("hello", vec![]).await;

        let mut got_text = false;
        let mut got_done = false;

        while let Some(event) = stream.next().await {
            match event {
                ChatStreamEvent::TextDelta(text) => {
                    println!("{}", text);
                    got_text = true;
                }
                ChatStreamEvent::Done => {
                    got_done = true;
                }
                ChatStreamEvent::Error(e) => {
                    panic!("Unexpected error: {}", e);
                }
            }
        }

        assert!(got_text, "Should have received at least one text delta");
        assert!(got_done, "Should have received Done event");
    }
}
