use axum::{
    extract::State,
    response::sse::{Event, Sse},
    Json,
};
use futures::{stream::Stream, StreamExt};
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

    // Send message to agent and get response
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
