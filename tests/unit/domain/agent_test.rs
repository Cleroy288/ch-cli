//! Tests for domain agent types.

use rustean::domain::agent::{
	AgentKind, AgentPreference,
};

/// AgentKind serializes to snake_case
#[test]
fn agent_kind_serializes_to_snake_case() {
	// Act
	let json = serde_json::to_string(
		&AgentKind::ClaudeCode,
	)
	.unwrap();

	// Assert
	assert_eq!(json, "\"claude_code\"");
}

/// AgentKind deserializes from snake_case
#[test]
fn agent_kind_deserializes_from_snake_case() {
	// Act
	let kind: AgentKind =
		serde_json::from_str("\"claude_code\"")
			.unwrap();

	// Assert
	assert_eq!(kind, AgentKind::ClaudeCode);
}

/// AgentKind label returns human-readable text
#[test]
fn agent_kind_label_is_readable() {
	assert_eq!(
		AgentKind::ClaudeCode.label(),
		"Claude Code",
	);
}

/// AgentPreference round-trips through JSON
#[test]
fn preference_roundtrips_through_json() {
	// Arrange
	let pref = AgentPreference {
		kind: AgentKind::ClaudeCode,
	};

	// Act
	let json =
		serde_json::to_string(&pref).unwrap();
	let back: AgentPreference =
		serde_json::from_str(&json).unwrap();

	// Assert
	assert_eq!(back.kind, AgentKind::ClaudeCode);
}

/// AgentKind implements Copy
#[test]
fn agent_kind_is_copy() {
	let kind = AgentKind::ClaudeCode;
	let copy = kind;
	assert_eq!(kind, copy);
}
