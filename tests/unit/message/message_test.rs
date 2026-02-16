//! Unit tests for the message module
//!
//! Covers MessageSegment, UserMessage, and ConversationHistory.

use rustean::message::{ConversationHistory, MessageSegment, UserMessage};

// ============================================================================
// MessageSegment tests
// ============================================================================

/// debug_string returns formatted text for a Text segment
#[test]
fn segment_debug_string_text() {
    let segment = MessageSegment::Text("hello world".to_string()); // text segment to test
    let result = segment.debug_string(); // debug output

    assert_eq!(result, "Text: \"hello world\"");
}

/// debug_string returns formatted text for a FileReference segment
#[test]
fn segment_debug_string_file_reference() {
    let segment = MessageSegment::FileReference {
        // file reference segment
        full_path: "/home/user/main.rs".to_string(),
        display_name: "main.rs".to_string(),
    };
    let result = segment.debug_string(); // debug output

    assert_eq!(result, "File: main.rs (/home/user/main.rs)");
}

/// debug_string returns formatted text for a FolderReference segment
#[test]
fn segment_debug_string_folder_reference() {
    let segment = MessageSegment::FolderReference {
        // folder reference segment
        full_path: "/home/user/src".to_string(),
        display_name: "src".to_string(),
    };
    let result = segment.debug_string(); // debug output

    assert_eq!(result, "Folder: src (/home/user/src)");
}

/// display_text returns raw text for a Text segment
#[test]
fn segment_display_text_text() {
    let segment = MessageSegment::Text("some text".to_string()); // text segment
    let result = segment.display_text(); // display output

    assert_eq!(result, "some text");
}

/// display_text returns display_name for a FileReference segment
#[test]
fn segment_display_text_file_reference() {
    let segment = MessageSegment::FileReference {
        // file reference segment
        full_path: "/home/user/lib.rs".to_string(),
        display_name: "lib.rs".to_string(),
    };
    let result = segment.display_text(); // display output

    assert_eq!(result, "lib.rs");
}

/// display_text returns display_name for a FolderReference segment
#[test]
fn segment_display_text_folder_reference() {
    let segment = MessageSegment::FolderReference {
        // folder reference segment
        full_path: "/home/user/tests".to_string(),
        display_name: "tests".to_string(),
    };
    let result = segment.display_text(); // display output

    assert_eq!(result, "tests");
}

/// debug_string handles empty text
#[test]
fn segment_debug_string_empty_text() {
    let segment = MessageSegment::Text(String::new()); // empty text segment
    let result = segment.debug_string(); // debug output

    assert_eq!(result, "Text: \"\"");
}

/// display_text handles empty text
#[test]
fn segment_display_text_empty_text() {
    let segment = MessageSegment::Text(String::new()); // empty text segment
    let result = segment.display_text(); // display output

    assert_eq!(result, "");
}

// ============================================================================
// UserMessage tests
// ============================================================================

/// Helper: build a Vec of segments with text, file, and folder references
fn make_mixed_segments() -> Vec<MessageSegment> {
    vec![
        MessageSegment::Text("look at ".to_string()),
        MessageSegment::FileReference {
            full_path: "/src/main.rs".to_string(),
            display_name: "main.rs".to_string(),
        },
        MessageSegment::Text(" and ".to_string()),
        MessageSegment::FolderReference {
            full_path: "/src/utils".to_string(),
            display_name: "utils".to_string(),
        },
        MessageSegment::FileReference {
            full_path: "/src/lib.rs".to_string(),
            display_name: "lib.rs".to_string(),
        },
    ]
}

/// new creates a UserMessage with correct fields and a valid timestamp
#[test]
fn user_message_new() {
    let segments = vec![MessageSegment::Text("hello".to_string())]; // single text segment
    let raw = "hello".to_string(); // raw input

    let msg = UserMessage::new(segments, raw);

    assert_eq!(msg.raw_input, "hello");
    assert_eq!(msg.segments.len(), 1);
    assert!(msg.timestamp > 0);
}

/// as_display_string joins all segment display texts
#[test]
fn user_message_as_display_string() {
    let segments = make_mixed_segments(); // mixed segment list
    let raw = "look at main.rs and utils lib.rs".to_string(); // raw input

    let msg = UserMessage::new(segments, raw);
    let result = msg.as_display_string(); // joined display string

    assert_eq!(result, "look at main.rs and utilslib.rs");
}

/// as_display_string returns empty string for empty segments
#[test]
fn user_message_as_display_string_empty() {
    let segments: Vec<MessageSegment> = vec![]; // no segments
    let raw = String::new(); // empty raw input

    let msg = UserMessage::new(segments, raw);
    let result = msg.as_display_string(); // display output

    assert_eq!(result, "");
}

/// debug_string shows indexed segment details
#[test]
fn user_message_debug_string() {
    let segments = vec![
        // two segments: text and file
        MessageSegment::Text("hi ".to_string()),
        MessageSegment::FileReference {
            full_path: "/a.rs".to_string(),
            display_name: "a.rs".to_string(),
        },
    ];
    let raw = "hi a.rs".to_string(); // raw input

    let msg = UserMessage::new(segments, raw);
    let result = msg.debug_string(); // debug output

    assert!(result.contains("Message Segments:"));
    assert!(result.contains("[0] Text: \"hi \""));
    assert!(result.contains("[1] File: a.rs (/a.rs)"));
}

/// file_count returns the number of FileReference segments
#[test]
fn user_message_file_count() {
    let segments = make_mixed_segments(); // mixed segments with 2 files
    let raw = "test".to_string(); // raw input

    let msg = UserMessage::new(segments, raw);
    let count = msg.file_count(); // file reference count

    assert_eq!(count, 2);
}

/// file_count returns 0 when no file references exist
#[test]
fn user_message_file_count_none() {
    let segments = vec![MessageSegment::Text("no files".to_string())]; // text only
    let raw = "no files".to_string(); // raw input

    let msg = UserMessage::new(segments, raw);
    let count = msg.file_count(); // file reference count

    assert_eq!(count, 0);
}

/// folder_count returns the number of FolderReference segments
#[test]
fn user_message_folder_count() {
    let segments = make_mixed_segments(); // mixed segments with 1 folder
    let raw = "test".to_string(); // raw input

    let msg = UserMessage::new(segments, raw);
    let count = msg.folder_count(); // folder reference count

    assert_eq!(count, 1);
}

/// folder_count returns 0 when no folder references exist
#[test]
fn user_message_folder_count_none() {
    let segments = vec![MessageSegment::Text("no folders".to_string())]; // text only
    let raw = "no folders".to_string(); // raw input

    let msg = UserMessage::new(segments, raw);
    let count = msg.folder_count(); // folder reference count

    assert_eq!(count, 0);
}

/// file_paths returns full paths for all FileReference segments
#[test]
fn user_message_file_paths() {
    let segments = make_mixed_segments(); // mixed segments
    let raw = "test".to_string(); // raw input

    let msg = UserMessage::new(segments, raw);
    let paths = msg.file_paths(); // extracted file paths

    assert_eq!(paths, vec!["/src/main.rs", "/src/lib.rs"]);
}

/// file_paths returns empty vec when no file references exist
#[test]
fn user_message_file_paths_empty() {
    let segments = vec![MessageSegment::Text("text only".to_string())]; // text only
    let raw = "text only".to_string(); // raw input

    let msg = UserMessage::new(segments, raw);
    let paths = msg.file_paths(); // extracted file paths

    assert!(paths.is_empty());
}

/// folder_paths returns full paths for all FolderReference segments
#[test]
fn user_message_folder_paths() {
    let segments = make_mixed_segments(); // mixed segments
    let raw = "test".to_string(); // raw input

    let msg = UserMessage::new(segments, raw);
    let paths = msg.folder_paths(); // extracted folder paths

    assert_eq!(paths, vec!["/src/utils"]);
}

/// folder_paths returns empty vec when no folder references exist
#[test]
fn user_message_folder_paths_empty() {
    let segments = vec![MessageSegment::Text("text only".to_string())]; // text only
    let raw = "text only".to_string(); // raw input

    let msg = UserMessage::new(segments, raw);
    let paths = msg.folder_paths(); // extracted folder paths

    assert!(paths.is_empty());
}

// ============================================================================
// ConversationHistory tests
// ============================================================================

/// Helper: create a simple UserMessage with given text
fn make_message(text: &str) -> UserMessage {
    let segments = vec![MessageSegment::Text(text.to_string())]; // single text segment
    UserMessage::new(segments, text.to_string())
}

/// new creates an empty history with default max (100)
#[test]
fn history_new() {
    let history = ConversationHistory::new(); // default history

    assert!(history.is_empty());
    assert_eq!(history.len(), 0);
}

/// with_capacity creates an empty history with custom max
#[test]
fn history_with_capacity() {
    let history = ConversationHistory::with_capacity(5); // capped at 5

    assert!(history.is_empty());
    assert_eq!(history.len(), 0);
}

/// add_message inserts a message and increments len
#[test]
fn history_add_message() {
    let mut history = ConversationHistory::new(); // empty history
    let msg = make_message("first"); // test message

    history.add_message(msg);

    assert_eq!(history.len(), 1);
    assert!(!history.is_empty());
}

/// add_message evicts oldest when max_messages exceeded
#[test]
fn history_add_message_eviction() {
    let mut history = ConversationHistory::with_capacity(2); // max 2 messages

    history.add_message(make_message("one"));
    history.add_message(make_message("two"));
    history.add_message(make_message("three")); // should evict "one"

    assert_eq!(history.len(), 2);

    let msgs = history.messages(); // remaining messages
    assert_eq!(msgs[0].raw_input, "two");
    assert_eq!(msgs[1].raw_input, "three");
}

/// messages returns the internal deque reference
#[test]
fn history_messages() {
    let mut history = ConversationHistory::new(); // empty history
    history.add_message(make_message("alpha"));
    history.add_message(make_message("beta"));

    let msgs = history.messages(); // message deque

    assert_eq!(msgs.len(), 2);
    assert_eq!(msgs[0].raw_input, "alpha");
    assert_eq!(msgs[1].raw_input, "beta");
}

/// last_message returns None when history is empty
#[test]
fn history_last_message_empty() {
    let history = ConversationHistory::new(); // empty history
    let last = history.last_message(); // should be None

    assert!(last.is_none());
}

/// last_message returns the most recently added message
#[test]
fn history_last_message() {
    let mut history = ConversationHistory::new(); // empty history
    history.add_message(make_message("first"));
    history.add_message(make_message("second"));

    let last = history.last_message().unwrap(); // most recent message

    assert_eq!(last.raw_input, "second");
}

/// len returns correct count
#[test]
fn history_len() {
    let mut history = ConversationHistory::new(); // empty history

    assert_eq!(history.len(), 0);

    history.add_message(make_message("a"));
    assert_eq!(history.len(), 1);

    history.add_message(make_message("b"));
    assert_eq!(history.len(), 2);
}

/// is_empty returns true only when history has no messages
#[test]
fn history_is_empty() {
    let mut history = ConversationHistory::new(); // empty history

    assert!(history.is_empty());

    history.add_message(make_message("msg"));

    assert!(!history.is_empty());
}

/// clear removes all messages
#[test]
fn history_clear() {
    let mut history = ConversationHistory::new(); // history with messages
    history.add_message(make_message("one"));
    history.add_message(make_message("two"));

    history.clear();

    assert!(history.is_empty());
    assert_eq!(history.len(), 0);
    assert!(history.last_message().is_none());
}

/// debug_string formats all messages with indices and counts
#[test]
fn history_debug_string() {
    let mut history = ConversationHistory::new(); // history for debug output

    let segments = vec![
        // message with a file reference
        MessageSegment::Text("check ".to_string()),
        MessageSegment::FileReference {
            full_path: "/x.rs".to_string(),
            display_name: "x.rs".to_string(),
        },
    ];
    let msg = UserMessage::new(segments, "check x.rs".to_string());
    history.add_message(msg);

    let result = history.debug_string(); // full debug output

    assert!(result.contains("Conversation History (1 messages):"));
    assert!(result.contains("=== Message 1 ==="));
    assert!(result.contains("Raw: \"check x.rs\""));
    assert!(result.contains("Files: 1, Folders: 0"));
}

/// debug_string shows correct count for empty history
#[test]
fn history_debug_string_empty() {
    let history = ConversationHistory::new(); // empty history
    let result = history.debug_string(); // debug output

    assert!(result.contains("Conversation History (0 messages):"));
}

/// Default trait implementation works same as new
#[test]
fn history_default_trait() {
    let history = ConversationHistory::default(); // via Default trait

    assert!(history.is_empty());
    assert_eq!(history.len(), 0);
}
