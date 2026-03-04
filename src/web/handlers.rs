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
    /// If true, inject a research preamble instructing the agent to investigate
    /// the topic with web_search + web_fetch and return a structured report.
    pub research_mode: Option<bool>,
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
    /// The agent invoked a tool (e.g. web_search, web_fetch)
    ToolUse { tool_name: String },
}

/// Build the research preamble that instructs the agent to investigate a topic.
fn research_prompt(topic: &str) -> String {
    format!(
        "あなたは調査アシスタントです。以下のトピックについて徹底的に調査してください。\n\n\
         手順:\n\
         1. web_search ツールで複数のクエリを実行し、信頼性の高いソースを特定する\n\
         2. web_fetch ツールで上位3〜5件のページを取得し、内容を精読する\n\
         3. 以下の構造でMarkdownレポートを作成する:\n\
            ## 概要\n\
            ## 主要な発見\n\
            ## 詳細分析\n\
            ## ソース一覧（URLと概要）\n\n\
         トピック: {topic}",
    )
}

/// Internal function that returns a stream of SSE events
/// Separated for testability - tests can consume this stream directly
async fn chat_stream(
    state: Arc<AppState>,
    session_id: String,
    message: String,
    research_mode: bool,
) -> impl Stream<Item = Result<Event, std::convert::Infallible>> {
    let (mut tx, rx) = mpsc::channel::<Event>(100);

    tokio::spawn(async move {
        let prompt = if research_mode {
            research_prompt(&message)
        } else {
            message
        };

        let mut response_text = String::new();
        let mut agent_stream = state
            .agent
            .stream_chat(&prompt, state.get_session(&session_id).unwrap().to_vec())
            .await;

        while let Some(event) = agent_stream.next().await {
            let sse_event = match event {
                ChatStreamEvent::TextDelta(text) => {
                    response_text.push_str(&text);
                    Event::default()
                        .json_data(SseEventData::Text { content: text })
                        .unwrap()
                }
                ChatStreamEvent::ToolCall { name } => Event::default()
                    .json_data(SseEventData::ToolUse { tool_name: name })
                    .unwrap(),
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

    rx.map(Ok)
}

/// Verify handler that confirms a Bearer token is valid
///
/// This handler itself does nothing—the `require_bearer_token` middleware
/// already rejected any invalid tokens before reaching here.
/// Returning 200 OK is sufficient to tell the client "your token works".
pub async fn verify_handler() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
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

    let research_mode = req.research_mode.unwrap_or(false);
    // Get stream and wrap in SSE response
    let stream = chat_stream(state, session_id, req.message, research_mode).await;
    Sse::new(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{ChatStreamEvent, MockAgent};
    use crate::web::AppState;

    fn make_state(agent: MockAgent) -> Arc<AppState> {
        Arc::new(AppState::new(Arc::new(agent), "test-token".to_string()))
    }

    #[tokio::test]
    async fn test_chat_saves_assistant_response_to_history() {
        let state = make_state(MockAgent::with_response("Hello from mock!"));
        let session_id = state.create_session();

        state.add_user_message(&session_id, "test message");

        let mut stream = chat_stream(
            state.clone(),
            session_id.clone(),
            "test message".to_string(),
            false,
        )
        .await;

        // Consume entire stream (simulate client)
        while stream.next().await.is_some() {}

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
    async fn test_multi_turn_conversation_preserves_context() {
        // Two queued responses: one per turn
        let state = make_state(MockAgent::new(vec![
            vec![
                ChatStreamEvent::TextDelta("First response".to_string()),
                ChatStreamEvent::Done,
            ],
            vec![
                ChatStreamEvent::TextDelta("Second response".to_string()),
                ChatStreamEvent::Done,
            ],
        ]));
        let session_id = state.create_session();

        // Turn 1
        state.add_user_message(&session_id, "first message");
        let mut s1 = chat_stream(
            state.clone(),
            session_id.clone(),
            "first message".to_string(),
            false,
        )
        .await;
        while s1.next().await.is_some() {}

        // Turn 2
        state.add_user_message(&session_id, "second message");
        let mut s2 = chat_stream(
            state.clone(),
            session_id.clone(),
            "second message".to_string(),
            false,
        )
        .await;
        while s2.next().await.is_some() {}

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let history = state.get_session(&session_id).unwrap();
        assert_eq!(
            history.len(),
            4,
            "Should have 4 messages (user+assistant × 2)"
        );
    }

    #[tokio::test]
    async fn test_chat_propagates_agent_error_event() {
        let state = make_state(MockAgent::with_error("llm exploded"));
        let session_id = state.create_session();

        state.add_user_message(&session_id, "test message");

        let mut stream = chat_stream(
            state.clone(),
            session_id.clone(),
            "test message".to_string(),
            false,
        )
        .await;

        // Collect all SSE events to find the error event
        let mut found_error = false;
        while let Some(Ok(event)) = stream.next().await {
            // The Event's data should contain the error JSON
            let data = format!("{:?}", event);
            if data.contains("llm exploded") {
                found_error = true;
            }
        }

        assert!(found_error, "Should have received an error SSE event");
    }

    #[test]
    fn test_research_prompt_contains_topic() {
        let sut = research_prompt("量子コンピュータ");

        assert!(
            sut.contains("量子コンピュータ"),
            "Prompt should contain the topic"
        );
    }

    #[test]
    fn test_research_prompt_contains_tool_instructions() {
        let sut = research_prompt("任意のトピック");

        assert!(
            sut.contains("web_search"),
            "Prompt should mention web_search tool"
        );
        assert!(
            sut.contains("web_fetch"),
            "Prompt should mention web_fetch tool"
        );
    }

    #[test]
    fn test_research_prompt_contains_report_structure() {
        let sut = research_prompt("任意のトピック");

        assert!(
            sut.contains("## 概要"),
            "Prompt should define report structure"
        );
        assert!(
            sut.contains("## ソース一覧"),
            "Prompt should request sources section"
        );
    }

    #[tokio::test]
    async fn test_tool_call_event_emits_tool_use_sse_event() {
        let state = make_state(MockAgent::new(vec![vec![
            ChatStreamEvent::ToolCall {
                name: "web_search".to_string(),
            },
            ChatStreamEvent::Done,
        ]]));
        let session_id = state.create_session();

        state.add_user_message(&session_id, "test");

        let mut stream =
            chat_stream(state.clone(), session_id.clone(), "test".to_string(), false).await;

        let mut found_tool_use = false;
        while let Some(Ok(event)) = stream.next().await {
            let data = format!("{:?}", event);
            if data.contains("web_search") {
                found_tool_use = true;
            }
        }

        assert!(found_tool_use, "Should have emitted a tool_use SSE event");
    }
}
