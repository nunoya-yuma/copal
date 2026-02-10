use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::agent::AnyAgent;
use crate::session::{ConversationHistory, DEFAULT_MAX_TURNS};

/// Shared application state for the web server.
/// Cloned across all request handlers via Axum's State extractor.
#[derive(Clone)]
pub struct AppState {
    /// The LLM agent (provider-agnostic)
    pub agent: Arc<AnyAgent>,
    /// In-memory session store (session_id -> conversation history)
    sessions: Arc<Mutex<HashMap<String, ConversationHistory>>>,
}

impl AppState {
    /// Create a new AppState with the given agent.
    pub fn new(agent: AnyAgent) -> Self {
        Self {
            agent: Arc::new(agent),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new session and return its ID.
    /// The session is initialized with empty conversation history.
    pub fn create_session(&self) -> String {
        // TODO(human): Implement session creation
        // 1. Generate a new UUID using uuid::Uuid::new_v4()
        // 2. Create a new ConversationHistory with DEFAULT_MAX_TURNS
        // 3. Lock the sessions HashMap and insert the new entry
        // 4. Return the session ID as a String
        todo!()
    }

    /// Get a reference to the conversation history for a session.
    /// Returns None if the session doesn't exist.
    pub fn get_session(&self, session_id: &str) -> Option<ConversationHistory> {
        // TODO(human): Implement session retrieval
        // 1. Lock the sessions HashMap
        // 2. Get the ConversationHistory for the given session_id
        // 3. Clone it (ConversationHistory needs to be cloneable - add derive if needed)
        // 4. Return Some(history) or None if not found
        todo!()
    }

    /// Add a user message to a session's conversation history.
    /// Creates the session if it doesn't exist.
    pub fn add_user_message(&self, session_id: &str, message: &str) {
        // TODO(human): Implement adding user message
        // 1. Lock the sessions HashMap
        // 2. Get or create the session's ConversationHistory
        // 3. Call history.add_user(message)
        // Hint: Use .entry(session_id.to_string()).or_insert_with(|| ...)
        todo!()
    }

    /// Add an assistant message to a session's conversation history.
    pub fn add_assistant_message(&self, session_id: &str, message: &str) {
        // TODO(human): Implement adding assistant message
        // 1. Lock the sessions HashMap
        // 2. Get the session's ConversationHistory (assume it exists)
        // 3. Call history.add_assistant(message)
        // If the session doesn't exist, this is a logic error - consider panic or error
        todo!()
    }
}
