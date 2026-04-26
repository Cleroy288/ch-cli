pub mod history;
mod history_ops;
pub mod segment;
pub mod user_message;
mod user_message_paths;

// Re-export commonly used types
pub use history::ConversationHistory;
pub use segment::MessageSegment;
pub use user_message::UserMessage;
