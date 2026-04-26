//! Tests for setup state persistence.

use std::fs;

use rustean::domain::config::SetupState;
use rustean::domain::data_paths;
use rustean::domain::data_paths_dirs;
use rustean::service::config;

/// Helper: create project data directory
fn make_data_dir(
	root: &std::path::Path,
) -> std::path::PathBuf {
	let data = data_paths::data_dir(root);
	fs::create_dir_all(&data).unwrap();
	data
}

/// mark_setup_done persists a single flag
#[test]
fn mark_setup_done_persists_flag() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	make_data_dir(root);

	// Act
	config::update_config(root, |cfg| {
		cfg.setup.credentials = true;
	})
	.unwrap();

	// Assert
	let cfg = config::load_config(root);
	assert!(cfg.setup.credentials);
	assert!(!cfg.setup.agent);
}

/// Multiple flags can be set independently
#[test]
fn multiple_flags_set_independently() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	make_data_dir(root);

	// Act
	config::update_config(root, |cfg| {
		cfg.setup.agent = true;
	})
	.unwrap();
	config::update_config(root, |cfg| {
		cfg.setup.indexing = true;
	})
	.unwrap();

	// Assert
	let cfg = config::load_config(root);
	assert!(cfg.setup.agent);
	assert!(cfg.setup.indexing);
	assert!(!cfg.setup.credentials);
}

/// SetupState defaults to all false
#[test]
fn default_setup_state_is_all_false() {
	// Arrange + Act
	let state = SetupState::default();

	// Assert
	assert!(!state.credentials);
	assert!(!state.agent);
	assert!(!state.integration);
	assert!(!state.repo_scan);
	assert!(!state.indexing);
}

/// Serde roundtrip preserves setup flags
#[test]
fn serde_roundtrip_setup_flags() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	make_data_dir(root);
	config::update_config(root, |cfg| {
		cfg.setup.credentials = true;
		cfg.setup.repo_scan = true;
	})
	.unwrap();

	// Act — reload from disk
	let cfg = config::load_config(root);

	// Assert
	assert!(cfg.setup.credentials);
	assert!(cfg.setup.repo_scan);
	assert!(!cfg.setup.agent);
}

/// Missing setup key deserializes to defaults
#[test]
fn missing_setup_key_defaults_to_false() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let cfg_path = data_paths_dirs::config_file(root);
	fs::create_dir_all(cfg_path.parent().unwrap())
		.unwrap();
	// Write config without setup key
	let json = r#"{"claude":{"model":"sonnet"}}"#;
	fs::write(&cfg_path, json).unwrap();

	// Act
	let cfg = config::load_config(root);

	// Assert
	assert!(!cfg.setup.credentials);
	assert!(!cfg.setup.indexing);
}
