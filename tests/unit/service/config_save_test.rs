//! Tests for config save service.

use rustean::domain::config::IntegrationMode;
use rustean::domain::data_paths_dirs;
use rustean::service::config;

/// Save then load returns same data
#[test]
fn save_load_roundtrip() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let mut cfg = config::load_config(root);
	cfg.claude.model = "opus".into();

	// Act
	config::save_config(root, &cfg).unwrap();
	let loaded = config::load_config(root);

	// Assert
	assert_eq!(loaded.claude.model, "opus");
}

/// update_config mutates only targeted field
#[test]
fn update_config_partial_mutation() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	config::update_config(root, |cfg| {
		cfg.claude.model = "haiku".into();
	})
	.unwrap();

	// Act — change integration, keep model
	config::update_config(root, |cfg| {
		cfg.claude.integration =
			IntegrationMode::Api {
				api_key: "sk-x".into(),
			};
	})
	.unwrap();
	let loaded = config::load_config(root);

	// Assert — model untouched
	assert_eq!(loaded.claude.model, "haiku");
	assert_eq!(
		loaded.claude.integration,
		IntegrationMode::Api {
			api_key: "sk-x".into(),
		},
	);
}

/// Save creates data directory if missing
#[test]
fn save_creates_directory() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let cfg = config::load_config(root);

	// Act
	config::save_config(root, &cfg).unwrap();

	// Assert
	let path = data_paths_dirs::config_file(root);
	assert!(path.exists());
}
