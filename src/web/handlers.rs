use axum::{
    extract::State,
    response::sse::{Event, Sse},
    Json,
};
use futures::{channel::mpsc, stream::Stream, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// TODO(human): You'll need this for implementation
#[allow(unused_imports)]
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
    // TODO(human): Implement channel-based streaming with history persistence
    //
    // Current implementation: Direct stream mapping (no history save)
    // Target implementation: Channel-based with text accumulation
    //
    // Steps to implement (TDD GREEN phase):
    // 1. Create mpsc channel: let (mut tx, rx) = mpsc::channel::<Event>(100);
    // 2. Clone Arc<AppState> and session_id for 'static lifetime
    // 3. Spawn tokio task to:
    //    - Consume agent stream
    //    - Accumulate text in String (like cli/repl.rs line 74)
    //    - Send SSE events to channel
    //    - On Done: call state.add_assistant_message()
    //    - Handle client disconnect (tx.send().is_err())
    // 4. Return Sse::new(rx.map(|event| Ok(event)))
    //
    // Reference:
    // - cli/repl.rs lines 62-95 for accumulation pattern
    // - Plan file for detailed implementation guide
    //
    // Remove the code below and implement the channel-based approach:

    let history = state.get_session(&session_id).unwrap().to_vec();
    let stream = state.agent.stream_chat(&message, history).await;
    let mapped = stream.map(move |item| {
        let event = match item {
            ChatStreamEvent::TextDelta(text) => Event::default()
                .json_data(SseEventData::Text { content: text })
                .unwrap(),
            ChatStreamEvent::Done => Event::default()
                .json_data(SseEventData::Done {
                    session_id: session_id.clone(),
                })
                .unwrap(),
            ChatStreamEvent::Error(e) => Event::default()
                .json_data(SseEventData::Error { message: e })
                .unwrap(),
        };
        Ok(event)
    });

    mapped
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
    async fn test_chat_handler_saves_assistant_response_to_history() {
        // Setup
        let agent = AnyAgent::from_env(WebFetch::new());
        let state = Arc::new(AppState::new(agent));
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
    #[ignore] // Remove this when implementing
    async fn test_multi_turn_conversation_preserves_context() {
        // TODO(human): Test multi-turn conversation history
        // Same pattern as first test, but call chat_handler twice with same session_id
        // After both turns complete, verify history.len() == 4

        let agent = AnyAgent::from_env(WebFetch::new());
        let state = Arc::new(AppState::new(agent));
        let session_id = state.create_session();

        // TODO(human): Call chat_handler twice, then assert history.len() == 4

        panic!("TODO(human): Implement this test");
    }
}
