use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;
use rig::completion::Message;

use super::ChatStreamEvent;

/// Provider-agnostic chat interface.
///
/// Abstracts over `AnyAgent` (real LLM) and `MockAgent` (test double),
/// enabling handler tests that don't require a live API key.
///
/// `#[async_trait]` is required because Rust's native `async fn` in traits
/// produces an unnameable `impl Future` type, which can't be stored in `dyn Trait`.
/// The macro desugars the async fn into `fn(...) -> Pin<Box<dyn Future>>`,
/// making the return type object-safe.
#[async_trait]
pub trait ChatAgent: Send + Sync {
    async fn stream_chat(
        &self,
        prompt: &str,
        history: Vec<Message>,
    ) -> Pin<Box<dyn Stream<Item = ChatStreamEvent> + Send>>;
}
