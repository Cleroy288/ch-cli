//! Tests for memory helper factories.

use rustean::domain::memory::{
	AiResponse, UserInput,
};
use rustean::domain::memory_helpers;
use rustean::service::memory::id_gen;

/// generate_id returns a non-empty UUID-like string
#[test]
fn generate_id_non_empty() {
	// Act
	let id = id_gen::generate_id();

	// Assert
	assert!(!id.is_empty());
	assert!(id.contains('-'));
}

/// generate_id produces distinct values each call
#[test]
fn generate_id_unique() {
	// Act
	let id1 = id_gen::generate_id();
	let id2 = id_gen::generate_id();

	// Assert — extremely unlikely to collide
	assert_ne!(id1, id2);
}

/// current_timestamp is after 2024-01-01
#[test]
fn current_timestamp_reasonable() {
	// Act
	let ts = id_gen::current_timestamp();

	// Assert — after 2024-01-01
	assert!(ts > 1_704_067_200);
}

/// new_session_id starts with "s-"
#[test]
fn new_session_id_format() {
	// Act
	let sid = id_gen::new_session_id();

	// Assert
	assert!(sid.starts_with("s-"));
	assert!(sid.len() > 5);
}

/// new_interaction populates all required fields
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
	let item = id_gen::new_interaction(
		"sess-1", input, response,
	);

	// Assert
	assert_eq!(item.session_id, "sess-1");
	assert!(!item.id.is_empty());
	assert!(item.timestamp > 0);
}

/// response_type_label maps each variant correctly
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

/// response_from_label reconstructs each variant
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
