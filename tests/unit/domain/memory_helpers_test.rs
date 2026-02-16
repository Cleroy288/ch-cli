//! Tests for memory helper factories.

use rustean::domain::memory::{
	AiResponse, UserInput,
};
use rustean::domain::memory_helpers;

/// Test generate_id returns non-empty string
#[test]
fn generate_id_non_empty() {
	// Act
	let id = memory_helpers::generate_id();

	// Assert
	assert!(!id.is_empty());
	assert!(id.contains('-'));
}

/// Test generate_id returns unique values
#[test]
fn generate_id_unique() {
	// Act
	let id1 = memory_helpers::generate_id();
	let id2 = memory_helpers::generate_id();

	// Assert — extremely unlikely to collide
	assert_ne!(id1, id2);
}

/// Test current_timestamp returns reasonable value
#[test]
fn current_timestamp_reasonable() {
	// Act
	let ts = memory_helpers::current_timestamp();

	// Assert — after 2024-01-01
	assert!(ts > 1_704_067_200);
}

/// Test new_session_id format
#[test]
fn new_session_id_format() {
	// Act
	let sid = memory_helpers::new_session_id();

	// Assert
	assert!(sid.starts_with("s-"));
	assert!(sid.len() > 5);
}

/// Test new_interaction builds correctly
#[test]
fn new_interaction_builds_fields() {
	// Arrange
	let input = UserInput {
		text: "test".to_string(),
		command: None,
		files: Vec::new(),
	};
	let response = AiResponse::Answer {
		text: "reply".to_string(),
	};

	// Act
	let item = memory_helpers::new_interaction(
		"sess-1", input, response,
	);

	// Assert
	assert_eq!(item.session_id, "sess-1");
	assert!(!item.id.is_empty());
	assert!(item.timestamp > 0);
}

/// Test response_type_label for all variants
#[test]
fn response_type_label_all_variants() {
	let cases = [
		(
			AiResponse::Answer {
				text: String::new(),
			},
			"answer",
		),
		(
			AiResponse::Question {
				text: String::new(),
			},
			"question",
		),
		(
			AiResponse::CodeChange {
				text: String::new(),
				changes: Vec::new(),
			},
			"code_change",
		),
	];
	for (response, expected) in &cases {
		assert_eq!(
			memory_helpers::response_type_label(
				response,
			),
			*expected,
		);
	}
}

/// Test response_from_label for all variants
#[test]
fn response_from_label_all_labels() {
	let cases = [
		("answer", "hi"),
		("question", "what?"),
		("code_change", "done"),
		("unknown", "fallback"),
	];
	for (label, text) in &cases {
		let resp =
			memory_helpers::response_from_label(
				label,
				text.to_string(),
			);
		let actual_label =
			memory_helpers::response_type_label(
				&resp,
			);
		if *label == "unknown" {
			assert_eq!(actual_label, "answer");
		} else {
			assert_eq!(actual_label, *label);
		}
	}
}
