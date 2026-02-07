//! Unit tests for ui::components::debug — migrated from inline tests

use ch_cli::message::{
    ConversationHistory, MessageSegment, UserMessage,
};
use ch_cli::ui::components::debug::DebugInfoType;
use ch_cli::ui::components::debug_render::build_message_history_lines;

/// DebugInfoType::MessageHistory has correct name
#[test]
fn test_debug_info_type_name() {
    let debug_type = DebugInfoType::MessageHistory;
    assert_eq!(debug_type.name(), "Message History");
}

/// DebugInfoType::default returns MessageHistory
#[test]
fn test_debug_info_type_default() {
    let default_type = DebugInfoType::default();
    assert_eq!(
        default_type,
        DebugInfoType::MessageHistory,
    );
}

/// Empty history returns placeholder lines
#[test]
fn test_build_message_history_lines_empty() {
    let history = ConversationHistory::new();
    let lines = build_message_history_lines(&history);

    assert!(lines.len() >= 2);
}

/// Non-empty history returns content lines
#[test]
fn test_build_message_history_lines_messages() {
    let mut history = ConversationHistory::new();
    let message = UserMessage::new(
        vec![MessageSegment::Text(
            "test".to_string(),
        )],
        "test".to_string(),
    );
    history.add_message(message);

    let lines = build_message_history_lines(&history);

    assert!(lines.len() > 2);
}
