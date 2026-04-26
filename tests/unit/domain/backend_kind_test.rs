use rustean::domain::backend_kind::BackendKind;

#[test]
fn serde_roundtrip_claude() {
	// Arrange
	let kind = BackendKind::ClaudeCode;

	// Act
	let json = serde_json::to_string(&kind).unwrap();
	let back: BackendKind =
		serde_json::from_str(&json).unwrap();

	// Assert
	assert_eq!(back, BackendKind::ClaudeCode);
	assert_eq!(json, "\"claude_code\"");
}

#[test]
fn serde_roundtrip_gemini() {
	// Arrange
	let kind = BackendKind::GeminiCli;

	// Act
	let json = serde_json::to_string(&kind).unwrap();
	let back: BackendKind =
		serde_json::from_str(&json).unwrap();

	// Assert
	assert_eq!(back, BackendKind::GeminiCli);
	assert_eq!(json, "\"gemini_cli\"");
}

#[test]
fn default_is_claude() {
	// Arrange / Act
	let kind = BackendKind::default();

	// Assert
	assert_eq!(kind, BackendKind::ClaudeCode);
}

#[test]
fn display_claude_returns_claude() {
	// Arrange / Act
	let text = BackendKind::ClaudeCode.to_string();

	// Assert
	assert_eq!(text, "claude");
}

#[test]
fn display_gemini_returns_gemini() {
	// Arrange / Act
	let text = BackendKind::GeminiCli.to_string();

	// Assert
	assert_eq!(text, "gemini");
}

#[test]
fn binary_name_claude() {
	// Assert
	assert_eq!(
		BackendKind::ClaudeCode.binary_name(),
		"claude",
	);
}

#[test]
fn binary_name_gemini() {
	// Assert
	assert_eq!(
		BackendKind::GeminiCli.binary_name(),
		"gemini",
	);
}
