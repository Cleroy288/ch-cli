//! Tests for memory domain types — serde round-trips.

use rustean::domain::memory::{
	AiResponse, FileChange, Interaction,
	UserInput,
};

use crate::helpers::factories::{
	make_ai_answer, make_user_input,
};

/// Test Interaction serializes and deserializes
#[test]
fn serialize_interaction_roundtrip() {
	// Arrange
	let interaction = Interaction {
		id: "test-123".to_string(),
		timestamp: 1000,
		session_id: "s-1".to_string(),
		input: make_user_input("hello"),
		response: make_ai_answer("world"),
	};
	let json =
		serde_json::to_string(&interaction).unwrap();

	// Act
	let parsed: Interaction =
		serde_json::from_str(&json).unwrap();

	// Assert
	assert_eq!(parsed.id, "test-123");
	assert_eq!(parsed.session_id, "s-1");
}

/// Test AiResponse::Answer serializes correctly
#[test]
fn serialize_answer_variant() {
	// Arrange
	let resp = make_ai_answer("hello");

	// Act
	let json =
		serde_json::to_string(&resp).unwrap();

	// Assert
	assert!(json.contains("\"type\":\"Answer\""));
	assert!(json.contains("hello"));
}

/// Test AiResponse::CodeChange with FileChange
#[test]
fn serialize_code_change_with_files() {
	// Arrange
	let change = FileChange {
		file: "src/main.rs".to_string(),
		action: "modify".to_string(),
		diff: Some("+line".to_string()),
	};
	let resp = AiResponse::CodeChange {
		text: "done".to_string(),
		changes: vec![change],
	};

	// Act
	let json =
		serde_json::to_string(&resp).unwrap();
	let parsed: AiResponse =
		serde_json::from_str(&json).unwrap();

	// Assert
	if let AiResponse::CodeChange {
		text, changes,
	} = parsed
	{
		assert_eq!(text, "done");
		assert_eq!(changes.len(), 1);
		assert_eq!(changes[0].file, "src/main.rs");
	} else {
		panic!("Expected CodeChange variant");
	}
}

/// Test UserInput with empty files list
#[test]
fn serialize_user_input_empty_files() {
	// Arrange
	let input = make_user_input("query");

	// Act
	let json =
		serde_json::to_string(&input).unwrap();
	let parsed: UserInput =
		serde_json::from_str(&json).unwrap();

	// Assert
	assert_eq!(parsed.text, "query");
	assert!(parsed.files.is_empty());
}

/// Test AiResponse::Question variant
#[test]
fn serialize_question_variant() {
	// Arrange
	let resp = AiResponse::Question {
		text: "what?".to_string(),
	};

	// Act
	let json =
		serde_json::to_string(&resp).unwrap();

	// Assert
	assert!(json.contains("\"type\":\"Question\""));
}
