use std::pin::Pin;
use std::sync::Mutex;

use async_trait::async_trait;
use futures::Stream;
use rig::completion::Message;

use super::{ChatAgent, ChatStreamEvent};

/// A test double for `ChatAgent` that replays pre-configured responses.
///
/// Each call to `stream_chat` pops and returns the next `Vec<ChatStreamEvent>`
/// from the internal queue. Use the constructor helpers to set up expected
/// responses before running tests.
pub struct MockAgent {
    /// Queue of response event sequences. Each element is one full response.
    responses: Mutex<Vec<Vec<ChatStreamEvent>>>,
}

impl MockAgent {
    /// Create a MockAgent with multiple queued responses.
    /// Responses are returned in order (FIFO).
    pub fn new(responses: Vec<Vec<ChatStreamEvent>>) -> Self {
        Self {
            responses: Mutex::new(responses),
        }
    }

    /// Create a MockAgent that emits a single text response followed by Done.
    pub fn with_response(text: &str) -> Self {
        Self::new(vec![vec![
            ChatStreamEvent::TextDelta(text.to_string()),
            ChatStreamEvent::Done,
        ]])
    }

    /// Create a MockAgent that emits a single Error event.
    pub fn with_error(msg: &str) -> Self {
        Self::new(vec![vec![ChatStreamEvent::Error(msg.to_string())]])
    }
}

#[async_trait]
impl ChatAgent for MockAgent {
    async fn stream_chat(
        &self,
        _prompt: &str,
        _history: Vec<Message>,
    ) -> Pin<Box<dyn Stream<Item = ChatStreamEvent> + Send>> {
        let events = {
            let mut locked = self.responses.lock().unwrap();
            if locked.is_empty() {
                vec![]
            } else {
                locked.remove(0)
            }
        };
        Box::pin(futures::stream::iter(events))
    }
}
