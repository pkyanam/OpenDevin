//! Conversation history — message storage for a session.
//!
//! Reconstruction skeleton.

use serde::{Deserialize, Serialize};

/// A single conversation message.
///
/// TODO(reconstruction): the original `chisel-agent/src/conversation_history.rs`
/// stores/loads the per-session message history (message-forest crate).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Default)]
pub struct ConversationHistory {
    pub messages: Vec<Message>,
}

impl ConversationHistory {
    pub fn push(&mut self, msg: Message) {
        self.messages.push(msg);
    }
}