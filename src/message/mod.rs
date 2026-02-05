//! Message module
//!
//! Handles message parsing, storage, and conversation history.
//!
//! # Modules
//! - `segment`: MessageSegment enum for text, file, and folder references
//! - `user_message`: UserMessage struct representing a complete message
//! - `history`: ConversationHistory for managing message history

pub mod history;
pub mod segment;
pub mod user_message;

// Re-export commonly used types
pub use history::ConversationHistory;
pub use segment::MessageSegment;
pub use user_message::UserMessage;
