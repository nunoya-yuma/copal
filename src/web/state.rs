use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::agent::ChatAgent;
use crate::session::{ConversationHistory, DEFAULT_MAX_HISTORY_TURNS};

/// Shared application state for the web server.
/// Cloned across all request handlers via Axum's State extractor.
#[derive(Clone)]
pub struct AppState {
    /// The LLM agent (provider-agnostic, behind a trait object)
    pub agent: Arc<dyn ChatAgent>,
    /// Bearer token required for API access
    pub(crate) api_token: String,
    /// In-memory session store (session_id -> conversation history)
    sessions: Arc<Mutex<HashMap<String, ConversationHistory>>>,
}

impl AppState {
    /// Create a new AppState with the given agent and API token.
    pub fn new(agent: Arc<dyn ChatAgent>, api_token: String) -> Self {
        Self {
            agent,
            api_token,
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new session and return its ID.
    /// The session is initialized with empty conversation history.
    pub fn create_session(&self) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let new_history = ConversationHistory::new(DEFAULT_MAX_HISTORY_TURNS);
        {
            let mut locked = self.sessions.lock().unwrap();
            locked.insert(id.clone(), new_history);
        }
        id
    }

    /// Get a copy of the conversation history for a session.
    /// Returns None if the session doesn't exist.
    pub fn get_session(&self, session_id: &str) -> Option<ConversationHistory> {
        let locked = self.sessions.lock().unwrap();
        let history = match locked.get(session_id) {
            Some(h) => h,
            None => {
                return None;
            }
        };
        Some(history.clone())
    }

    /// Add a user message to a session's conversation history.
    /// Creates the session if it doesn't exist (fallback for flexibility).
    ///
    /// # Typical Usage Flow
    /// ```ignore
    /// // In chat handler:
    /// let session_id = match request.session_id {
    ///     Some(id) => id,                    // Existing session (continued conversation)
    ///     None => state.create_session(),    // New session (recommended: explicit creation)
    /// };
    /// state.add_user_message(&session_id, &request.message);
    /// ```
    ///
    /// The auto-create behavior provides flexibility for clients that generate their own UUIDs,
    /// but explicit `create_session()` is recommended for clearer lifecycle management.
    pub fn add_user_message(&self, session_id: &str, message: &str) {
        let mut locked = self.sessions.lock().unwrap();
        let history = locked
            .entry(session_id.to_string())
            .or_insert_with(|| ConversationHistory::new(DEFAULT_MAX_HISTORY_TURNS));
        history.add_user(message);
    }

    /// Add an assistant message to a session's conversation history.
    /// Assumes the session already exists (panics otherwise).
    ///
    /// # Expected Call Sequence
    /// ```ignore
    /// state.add_user_message(&session_id, "Hello");       // 1. User message (creates if needed)
    /// let history = state.get_session(&session_id).unwrap().to_vec();
    /// let stream = agent.stream_chat("Hello", history).await;  // 2. Get response
    /// // ... collect full response from stream ...
    /// state.add_assistant_message(&session_id, &response);     // 3. Save response
    /// ```
    ///
    /// This method does NOT auto-create because it's always called after `add_user_message`,
    /// which ensures the session exists. Missing session indicates a logic error.
    pub fn add_assistant_message(&self, session_id: &str, message: &str) {
        let mut locked = self.sessions.lock().unwrap();
        let history = locked
            .get_mut(session_id)
            .expect("session id does not exist");
        history.add_assistant(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::MockAgent;

    fn make_state() -> AppState {
        AppState::new(
            Arc::new(MockAgent::with_response("")),
            "test-token".to_string(),
        )
    }

    #[tokio::test]
    async fn test_create_new_session_and_get_history() {
        let state = make_state();
        let session_id = state.create_session();

        let history = state.get_session(session_id.as_str()).unwrap();

        assert!(history.is_empty());
    }

    #[tokio::test]
    async fn test_add_multiple_user_messages() {
        let state = make_state();
        let session_id = state.create_session();

        state.add_user_message(&session_id, "hello1");
        state.add_user_message(&session_id, "hello2");

        let locked = state.sessions.lock().unwrap();
        assert_eq!(locked.get(&session_id).unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_add_multiple_assistant_messages() {
        let state = make_state();
        let session_id = state.create_session();

        state.add_assistant_message(&session_id, "hello1");
        state.add_assistant_message(&session_id, "hello2");

        let locked = state.sessions.lock().unwrap();
        assert_eq!(locked.get(&session_id).unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_new_history_is_created_when_add_user_message_is_called_with_new_session_id() {
        let state = make_state();

        state.add_user_message("nonexistent_session_id", "hello1");

        let locked = state.sessions.lock().unwrap();
        assert_eq!(locked.get("nonexistent_session_id").unwrap().len(), 1);
    }

    #[tokio::test]
    #[should_panic(expected = "session id does not exist")]
    async fn test_add_assistant_message_panics_when_session_does_not_exist() {
        let state = make_state();

        // This should panic because the session doesn't exist
        state.add_assistant_message("nonexistent_session_id", "hello");
    }

    #[tokio::test]
    async fn test_typical_conversation_flow() {
        let state = make_state();
        let session_id = state.create_session();

        // Step 1: User sends a message
        state.add_user_message(&session_id, "What is Rust?");
        {
            let locked = state.sessions.lock().unwrap();
            assert_eq!(locked.get(&session_id).unwrap().len(), 1);
        }

        // Step 2: Retrieve history for LLM call (simulated here)
        let history = state.get_session(&session_id);
        assert!(history.is_some());
        assert_eq!(history.unwrap().len(), 1);

        // Step 3: Add assistant response after getting LLM output
        state.add_assistant_message(&session_id, "Rust is a systems programming language...");
        {
            let locked = state.sessions.lock().unwrap();
            assert_eq!(locked.get(&session_id).unwrap().len(), 2);
        }
    }

    #[tokio::test]
    async fn test_get_session_returns_independent_copy() {
        let state = make_state();
        let session_id = state.create_session();
        state.add_user_message(&session_id, "Hello");

        // Get a copy of the history
        let mut history_copy = state.get_session(&session_id).unwrap();

        // Modify the copy
        history_copy.add_user("This should not affect the original");

        // Verify the original session is unchanged
        let locked = state.sessions.lock().unwrap();
        assert_eq!(locked.get(&session_id).unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_multiple_sessions_are_independent() {
        let state = make_state();

        let session1 = state.create_session();
        let session2 = state.create_session();

        state.add_user_message(&session1, "Session 1 message 1");
        state.add_user_message(&session1, "Session 1 message 2");
        state.add_user_message(&session2, "Session 2 message 1");

        {
            let locked = state.sessions.lock().unwrap();
            assert_eq!(locked.get(&session1).unwrap().len(), 2);
            assert_eq!(locked.get(&session2).unwrap().len(), 1);
        }

        state.add_assistant_message(&session2, "Session 2 response");
        {
            let locked = state.sessions.lock().unwrap();
            assert_eq!(locked.get(&session1).unwrap().len(), 2);
            assert_eq!(locked.get(&session2).unwrap().len(), 2);
        }
    }
}
