use rustean::domain::backend_kind::BackendKind;
use rustean::service::backend::{
	ClaudeBackend, CliBackend, GeminiBackend,
	backend_for_kind,
};

#[test]
fn claude_backend_name() {
	// Arrange
	let backend = ClaudeBackend;

	// Assert
	assert_eq!(backend.name(), "claude");
	assert_eq!(backend.kind(), BackendKind::ClaudeCode);
}

#[test]
fn claude_backend_models() {
	// Arrange
	let backend = ClaudeBackend;

	// Assert
	let models = backend.valid_models();
	assert!(models.contains(&"sonnet"));
	assert!(models.contains(&"opus"));
	assert!(models.contains(&"haiku"));
	assert_eq!(backend.default_model(), "sonnet");
}

#[test]
fn claude_supports_sessions_and_effort() {
	// Arrange
	let backend = ClaudeBackend;

	// Assert
	assert!(backend.supports_sessions());
	assert!(backend.supports_effort());
}

#[test]
fn gemini_backend_name() {
	// Arrange
	let backend = GeminiBackend;

	// Assert
	assert_eq!(backend.name(), "gemini");
	assert_eq!(backend.kind(), BackendKind::GeminiCli);
}

#[test]
fn gemini_stub_returns_empty_models() {
	// Arrange
	let backend = GeminiBackend;

	// Assert — stub has no models yet
	assert!(backend.valid_models().is_empty());
	assert_eq!(backend.default_model(), "");
}

#[test]
fn gemini_stub_execute_returns_error() {
	// Arrange
	let backend = GeminiBackend;

	// Act
	let result = backend.execute(
		"test", None, "", "", None,
	);

	// Assert
	assert!(result.is_err());
}

#[test]
fn gemini_no_sessions_no_effort() {
	// Arrange
	let backend = GeminiBackend;

	// Assert
	assert!(!backend.supports_sessions());
	assert!(!backend.supports_effort());
}

#[test]
fn backend_for_kind_claude() {
	// Arrange / Act
	let backend =
		backend_for_kind(BackendKind::ClaudeCode);

	// Assert
	assert_eq!(backend.name(), "claude");
}

#[test]
fn backend_for_kind_gemini() {
	// Arrange / Act
	let backend =
		backend_for_kind(BackendKind::GeminiCli);

	// Assert
	assert_eq!(backend.name(), "gemini");
}

// -- Effort × Model validation --

#[test]
fn claude_opus_supports_max_effort() {
	// Arrange
	let backend = ClaudeBackend;

	// Act
	let efforts = backend.valid_efforts("opus");

	// Assert
	assert!(efforts.contains(&"max"));
	assert!(efforts.contains(&"low"));
	assert_eq!(efforts.len(), 4);
}

#[test]
fn claude_sonnet_rejects_max_effort() {
	// Arrange
	let backend = ClaudeBackend;

	// Act
	let efforts = backend.valid_efforts("sonnet");

	// Assert
	assert!(!efforts.contains(&"max"));
	assert_eq!(efforts.len(), 3);
}

#[test]
fn claude_haiku_has_no_effort() {
	// Arrange
	let backend = ClaudeBackend;

	// Act
	let efforts = backend.valid_efforts("haiku");

	// Assert
	assert!(efforts.is_empty());
}

#[test]
fn claude_opus_default_effort_is_medium() {
	// Arrange
	let backend = ClaudeBackend;

	// Assert
	assert_eq!(
		backend.default_effort_for_model("opus"),
		"medium",
	);
}

#[test]
fn claude_sonnet_default_effort_is_high() {
	// Arrange
	let backend = ClaudeBackend;

	// Assert
	assert_eq!(
		backend.default_effort_for_model("sonnet"),
		"high",
	);
}

#[test]
fn gemini_stub_no_efforts() {
	// Arrange
	let backend = GeminiBackend;

	// Assert
	assert!(
		backend.valid_efforts("any").is_empty()
	);
	assert_eq!(
		backend.default_effort_for_model("any"),
		"",
	);
}
