//! Tests for config load service.

use std::fs;

use rustean::domain::config::IntegrationMode;
use rustean::domain::data_paths_dirs;
use rustean::service::config;

/// Load from missing file returns defaults
#[test]
fn load_missing_returns_defaults() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();

	// Act
	let cfg = config::load_config(dir.path());

	// Assert
	assert_eq!(cfg.claude.model, "sonnet");
	assert_eq!(
		cfg.claude.integration,
		IntegrationMode::Cli,
	);
}

/// try_load returns None when no file
#[test]
fn try_load_returns_none_when_missing() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();

	// Act + Assert
	assert!(
		config::try_load_config(dir.path())
			.is_none()
	);
}

/// Load valid config.json returns data
#[test]
fn load_valid_file_returns_data() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let cfg_path = data_paths_dirs::config_file(root);
	fs::create_dir_all(cfg_path.parent().unwrap())
		.unwrap();
	let json = r#"{"claude":{"model":"opus"}}"#;
	fs::write(&cfg_path, json).unwrap();

	// Act
	let cfg = config::load_config(root);

	// Assert
	assert_eq!(cfg.claude.model, "opus");
}

/// Load corrupt JSON returns defaults
#[test]
fn load_corrupt_json_returns_defaults() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let cfg_path = data_paths_dirs::config_file(root);
	fs::create_dir_all(cfg_path.parent().unwrap())
		.unwrap();
	fs::write(&cfg_path, "{{bad").unwrap();

	// Act
	let cfg = config::load_config(root);

	// Assert
	assert_eq!(cfg.claude.model, "sonnet");
}
