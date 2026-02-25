use anyhow::Result;
use rig::{
    client::{CompletionClient, Nothing},
    completion::{message::AssistantContent, CompletionModel, CompletionRequest},
    providers::ollama,
    OneOrMany,
};

use super::LlmClient;

/// RigClient wraps Rig library to implement LlmClient trait
pub struct RigClient {
    model: String,
}

impl RigClient {
    pub fn new(model: &str) -> Self {
        Self {
            model: model.to_string(),
        }
    }
}

impl LlmClient for RigClient {
    async fn complete(&self, prompt: &str) -> Result<String> {
        // Create client using builder pattern (rig-core 0.28+)
        let client: ollama::Client = ollama::Client::builder()
            .api_key(Nothing)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create Ollama client: {}", e))?;

        let comp_model = client.completion_model(&self.model);

        // Build the user message
        let user_message = rig::message::Message::User {
            content: OneOrMany::one(rig::message::UserContent::text(prompt)),
        };

        let req = CompletionRequest {
            model: None,
            output_schema: None,
            preamble: None,
            chat_history: OneOrMany::one(user_message),
            documents: vec![],
            tools: vec![],
            temperature: Some(0.7),
            max_tokens: None,
            tool_choice: None,
            additional_params: None,
        };

        // Parse response
        let llm_response = comp_model.completion(req).await?;
        let response_contents = llm_response
            .choice
            .iter()
            .filter_map(|c| match c {
                AssistantContent::Text(text) => Some(text.text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        Ok(response_contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_rig_client_with_ollama() {
        let client = RigClient::new("llama3.2");
        let response = client.complete("Say hello").await.unwrap();

        assert!(!response.is_empty());
    }
}
