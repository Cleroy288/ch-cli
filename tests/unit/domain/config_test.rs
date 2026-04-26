//! Tests for unified config domain types.

use rustean::domain::config::{
	CacheConfig, ClaudeConfig, IntegrationMode,
	RusteanConfig, SetupState,
};

/// Default RusteanConfig has sonnet model
#[test]
fn default_config_has_sonnet_model() {
	// Arrange + Act
	let config = RusteanConfig::default();

	// Assert
	assert_eq!(config.claude.model, "sonnet");
}

/// Default IntegrationMode is Cli
#[test]
fn default_integration_mode_is_cli() {
	// Arrange + Act
	let mode = IntegrationMode::default();

	// Assert
	assert_eq!(mode, IntegrationMode::Cli);
}

/// Serde roundtrip preserves all fields
#[test]
fn serde_roundtrip_full_config() {
	// Arrange
	let config = RusteanConfig {
		credentials: None,
		aikido: None,
		claude: ClaudeConfig {
			backend: Default::default(),
			model: "opus".into(),
			effort: "high".into(),
			session_id: Some("sess-1".into()),
			integration: IntegrationMode::Cli,
		},
		agent: None,
		cache: CacheConfig::default(),
		setup: SetupState::default(),
	};

	// Act
	let json =
		serde_json::to_string(&config).unwrap();
	let loaded: RusteanConfig =
		serde_json::from_str(&json).unwrap();

	// Assert
	assert_eq!(loaded.claude.model, "opus");
	assert_eq!(
		loaded.claude.session_id.as_deref(),
		Some("sess-1"),
	);
}

/// Api variant serializes with api_key
#[test]
fn serde_api_mode_roundtrip() {
	// Arrange
	let mode = IntegrationMode::Api {
		api_key: "sk-test".into(),
	};

	// Act
	let json =
		serde_json::to_string(&mode).unwrap();
	let loaded: IntegrationMode =
		serde_json::from_str(&json).unwrap();

	// Assert
	assert_eq!(loaded, mode);
}

/// Missing fields deserialize to defaults
#[test]
fn deserialize_empty_object_uses_defaults() {
	// Arrange
	let json = "{}";

	// Act
	let config: RusteanConfig =
		serde_json::from_str(json).unwrap();

	// Assert
	assert_eq!(config.claude.model, "sonnet");
	assert!(config.credentials.is_none());
	assert!(config.agent.is_none());
}
