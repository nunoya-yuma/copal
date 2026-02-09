use std::cmp::min;

use rig::message::{AssistantContent, Message, UserContent};
use rig::OneOrMany;

/// Default maximum number of conversation turns to keep
pub const DEFAULT_MAX_TURNS: usize = 20;

/// Manages conversation history for multi-turn dialogue
pub struct ConversationHistory {
    messages: Vec<Message>,
    max_turns: usize,
}

impl ConversationHistory {
    /// Create a new conversation history with specified max turns
    pub fn new(max_turns: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_turns: min(max_turns, DEFAULT_MAX_TURNS),
        }
    }

    /// Add a user message to the history
    pub fn add_user(&mut self, input: &str) {
        let user_message = Message::User {
            content: OneOrMany::one(UserContent::text(input)),
        };
        self.messages.push(user_message);
        self.trim_if_needed();
    }

    /// Add an assistant message to the history
    pub fn add_assistant(&mut self, response: &str) {
        let assistant_message = Message::Assistant {
            id: None,
            content: OneOrMany::one(AssistantContent::text(response)),
        };
        self.messages.push(assistant_message);
        self.trim_if_needed();
    }

    /// Get the conversation history as a slice
    pub fn as_slice(&self) -> &[Message] {
        &self.messages
    }

    /// Get the number of messages in history
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Clone the conversation history as a Vec
    pub fn to_vec(&self) -> Vec<Message> {
        self.messages.clone()
    }

    /// Trim old messages if history exceeds max turns
    fn trim_if_needed(&mut self) {
        if self.messages.len() > self.max_turns * 2 {
            self.messages.drain(0..2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_empty_history() {
        let sut = ConversationHistory::new(2);
        assert!(sut.is_empty());
        assert_eq!(sut.max_turns, 2);
    }
    #[test]
    fn test_new_creates_when_max_turns_is_at_maximum() {
        let sut = ConversationHistory::new(DEFAULT_MAX_TURNS);
        assert_eq!(sut.max_turns, DEFAULT_MAX_TURNS);
    }

    #[test]
    fn test_new_creates_when_exceeding_the_maximum_max_turns() {
        let sut = ConversationHistory::new(DEFAULT_MAX_TURNS + 1);
        assert_eq!(sut.max_turns, DEFAULT_MAX_TURNS);
    }

    #[test]
    fn test_add_user_message() {
        let mut sut = ConversationHistory::new(2);
        sut.add_user("hello1");
        sut.add_user("hello2");
        sut.add_user("hello3");

        assert_eq!(sut.len(), 3);
    }

    #[test]
    fn test_add_assistant_message() {
        let mut sut = ConversationHistory::new(2);
        sut.add_assistant("hello1");
        sut.add_assistant("hello2");
        sut.add_assistant("hello3");

        assert_eq!(sut.len(), 3);
    }

    #[test]
    fn test_trim_removes_oldest_when_exceeding_max() {
        let mut sut = ConversationHistory::new(2);
        sut.add_user("user1");
        sut.add_assistant("assistant1");
        sut.add_user("user2");
        sut.add_assistant("assistant2");

        sut.add_user("user3");

        assert_eq!(sut.len(), 3);
    }

    #[test]
    fn test_trim_preserves_newest_messages() {
        let mut sut = ConversationHistory::new(2);
        sut.add_user("user1");
        sut.add_assistant("assistant1");
        sut.add_user("user2");
        sut.add_assistant("assistant2");
        sut.add_user("user3");
        sut.add_assistant("assistant3");

        // After adding 3 turns with max_turns=2, oldest turn should be removed
        assert_eq!(sut.len(), 4);

        // Verify the content: user1/assistant1 should be gone
        let messages = sut.as_slice();
        assert!(matches!(&messages[0], Message::User { .. }));
        assert!(matches!(&messages[1], Message::Assistant { .. }));
        assert!(matches!(&messages[2], Message::User { .. }));
        assert!(matches!(&messages[3], Message::Assistant { .. }));

        // Verify text content using helper
        assert_eq!(extract_user_text(&messages[0]), Some("user2".to_string()));
        assert_eq!(
            extract_assistant_text(&messages[1]),
            Some("assistant2".to_string())
        );
        assert_eq!(extract_user_text(&messages[2]), Some("user3".to_string()));
        assert_eq!(
            extract_assistant_text(&messages[3]),
            Some("assistant3".to_string())
        );
    }

    /// Helper to extract text from User message
    fn extract_user_text(msg: &Message) -> Option<String> {
        match msg {
            Message::User { content } => match content.first_ref() {
                UserContent::Text(text) => Some(text.text.clone()),
                _ => None,
            },
            _ => None,
        }
    }

    /// Helper to extract text from Assistant message
    fn extract_assistant_text(msg: &Message) -> Option<String> {
        match msg {
            Message::Assistant { content, .. } => match content.first_ref() {
                AssistantContent::Text(text) => Some(text.text.clone()),
                _ => None,
            },
            _ => None,
        }
    }
}
