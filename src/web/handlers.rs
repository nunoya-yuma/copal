use axum::{
    extract::State,
    response::sse::{Event, Sse},
    Json,
};
use futures::{channel::mpsc, stream::Stream, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::agent::ChatStreamEvent;
use crate::web::AppState;

/// Request body for the chat endpoint
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    /// Optional session ID. If not provided, a new session will be created.
    pub session_id: Option<String>,
    /// The user's message
    pub message: String,
}

/// SSE event data sent to the client
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SseEventData {
    /// Text delta from the assistant
    Text { content: String },
    /// Stream completed, includes session_id for future requests
    Done { session_id: String },
    /// Error occurred during processing
    Error { message: String },
}

/// Internal function that returns a stream of SSE events
/// Separated for testability - tests can consume this stream directly
async fn chat_stream(
    state: Arc<AppState>,
    session_id: String,
    message: String,
) -> impl Stream<Item = Result<Event, std::convert::Infallible>> {
    let (mut tx, rx) = mpsc::channel::<Event>(100);

    tokio::spawn(async move {
        let mut response_text = String::new();
        let mut agent_stream = state
            .agent
            .stream_chat(&message, state.get_session(&session_id).unwrap().to_vec())
            .await;

        while let Some(event) = agent_stream.next().await {
            let sse_event = match event {
                ChatStreamEvent::TextDelta(text) => {
                    response_text.push_str(&text);
                    Event::default()
                        .json_data(SseEventData::Text { content: text })
                        .unwrap()
                }
                ChatStreamEvent::Done => {
                    state.add_assistant_message(&session_id, &response_text);
                    Event::default()
                        .json_data(SseEventData::Done {
                            session_id: session_id.clone(),
                        })
                        .unwrap()
                }
                ChatStreamEvent::Error(e) => Event::default()
                    .json_data(SseEventData::Error { message: e })
                    .unwrap(),
            };

            if tx.send(sse_event).await.is_err() {
                break;
            }
        }
    });

    rx.map(|event| Ok(event))
}

/// Chat handler that streams responses via Server-Sent Events (SSE)
///
/// # Flow
/// 1. Get or create session
/// 2. Add user message to conversation history
/// 3. Call chat_stream to get event stream
/// 4. Return as SSE response
pub async fn chat_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ChatRequest>,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    // Save user message to history
    let session_id = match req.session_id {
        Some(i) => i,
        None => state.create_session(),
    };
    state.add_user_message(&session_id, &req.message);

    // Get stream and wrap in SSE response
    let stream = chat_stream(state, session_id, req.message).await;
    Sse::new(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AnyAgent, WebFetch};
    use crate::web::AppState;

    #[tokio::test]
    #[ignore] // Requires LLM API key (not available in CI)
    async fn test_chat_handler_saves_assistant_response_to_history() {
        // Setup
        let agent = AnyAgent::from_env(WebFetch::new());
        let state = Arc::new(AppState::new(agent, "test-token".to_string()));
        let session_id = state.create_session();

        // Add user message (normally done by chat_handler)
        state.add_user_message(&session_id, "test message");

        // Call internal stream function (testable)
        let mut stream = chat_stream(
            state.clone(),
            session_id.clone(),
            "test message".to_string(),
        )
        .await;

        // Consume entire stream (simulate client)
        while let Some(_) = stream.next().await {}

        // Wait for spawned task to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Verify: history should have 2 messages (user + assistant)
        let history = state.get_session(&session_id).unwrap();
        assert_eq!(
            history.len(),
            2,
            "Should have user message + assistant response"
        );
    }

    #[tokio::test]
    #[ignore] // Requires LLM API key (not available in CI)
    async fn test_multi_turn_conversation_preserves_context() {
        let agent = AnyAgent::from_env(WebFetch::new());
        let state = Arc::new(AppState::new(agent, "test-token".to_string()));
        let session_id = state.create_session();

        // Add user message (normally done by chat_handler)
        state.add_user_message(&session_id, "test message");

        // Call internal stream function (testable)
        let mut stream_first = chat_stream(
            state.clone(),
            session_id.clone(),
            "test message".to_string(),
        )
        .await;

        // Consume entire stream (simulate client)
        while let Some(_) = stream_first.next().await {}

        // Add user message (normally done by chat_handler)
        state.add_user_message(&session_id, "test message");

        // Call internal stream function (testable)
        let mut stream_second = chat_stream(
            state.clone(),
            session_id.clone(),
            "test message".to_string(),
        )
        .await;

        // Consume entire stream (simulate client)
        while let Some(_) = stream_second.next().await {}
        // Wait for spawned task to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Verify: history should have 2 messages (user + assistant)
        let history = state.get_session(&session_id).unwrap();
        assert_eq!(
            history.len(),
            4,
            "Should have 4 messages (user message + assistant response * 2)"
        );
    }
}
