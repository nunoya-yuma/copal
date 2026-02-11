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

/// Chat handler that streams responses via Server-Sent Events (SSE)
///
/// # Flow
/// 1. Get or create session
/// 2. Add user message to conversation history
/// 3. Get conversation history for LLM context
/// 4. Stream chat response from AnyAgent
/// 5. Convert ChatStreamEvent to SSE Event
/// 6. Add final assistant response to conversation history
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
    let stream = state.agent.stream_chat(&req.message, history).await;
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

    Sse::new(mapped)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AnyAgent, WebFetch};
    use crate::web::AppState;

    #[tokio::test]
    #[ignore] // Remove this when implementing
    async fn test_chat_handler_saves_assistant_response_to_history() {
        // TODO(human): Implement this test as part of TDD RED phase
        //
        // Expected flow:
        // 1. Create AppState with AnyAgent
        // 2. Create a new session
        // 3. Call chat_handler with a test message
        // 4. Consume the entire SSE stream (simulate client)
        // 5. Wait for spawned task to complete
        // 6. Assert: history.len() should be 2 (user + assistant)
        //
        // Reference: src/web/state.rs tests (lines 97-244) for patterns
        //
        // Current expected failure: assert_eq! will show 1 (user only) vs 2 (expected)

        let agent = AnyAgent::from_env(WebFetch::new());
        let state = Arc::new(AppState::new(agent));
        let session_id = state.create_session();

        let req = ChatRequest {
            session_id: Some(session_id.clone()),
            message: "テストメッセージ".to_string(),
        };

        // TODO(human): Call chat_handler here
        // let sse_stream = chat_handler(State(state.clone()), Json(req)).await;

        // TODO(human): Consume stream
        // let mut stream = sse_stream.into_inner();
        // while let Some(_) = stream.next().await {}

        // TODO(human): Wait for async task
        // tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // TODO(human): Verify history
        // let history = state.get_session(&session_id).unwrap();
        // assert_eq!(history.len(), 2, "Should have user message + assistant response");

        panic!("TODO(human): Implement this test");
    }

    #[tokio::test]
    #[ignore] // Remove this when implementing
    async fn test_multi_turn_conversation_preserves_context() {
        // TODO(human): Implement this test for multi-turn conversation
        //
        // Expected flow:
        // 1. Create session
        // 2. Send first message, consume stream, wait
        // 3. Send second message with same session_id, consume stream, wait
        // 4. Assert: history.len() should be 4 (user1, assistant1, user2, assistant2)
        //
        // This tests that conversation context is preserved across multiple turns

        let agent = AnyAgent::from_env(WebFetch::new());
        let state = Arc::new(AppState::new(agent));
        let session_id = state.create_session();

        // TODO(human): First turn
        // let req1 = ChatRequest { ... };
        // ... consume stream, wait ...

        // TODO(human): Second turn
        // let req2 = ChatRequest { ... };
        // ... consume stream, wait ...

        // TODO(human): Verify
        // let history = state.get_session(&session_id).unwrap();
        // assert_eq!(history.len(), 4);

        panic!("TODO(human): Implement this test");
    }
}
