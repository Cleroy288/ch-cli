//! Tests for Jira project key cache persistence.

use rustean::domain::data_paths_dirs;
use rustean::service::tools::jira_project_cache;

/// Round-trip: save then load returns same keys
#[test]
fn save_then_load_returns_keys() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let keys =
		vec!["EVB".to_string(), "FOO".to_string()];

	// Act
	jira_project_cache::save_project_keys(
		root, &keys,
	);
	let loaded =
		jira_project_cache::load_project_keys(root);

	// Assert
	assert_eq!(loaded, Some(keys));
}

/// Load from missing config returns None
#[test]
fn load_missing_file_returns_none() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();

	// Act
	let loaded =
		jira_project_cache::load_project_keys(root);

	// Assert
	assert!(loaded.is_none());
}

/// Save creates data directory and config.json
#[test]
fn save_creates_data_directory() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let keys = vec!["ABC".to_string()];

	// Act
	jira_project_cache::save_project_keys(
		root, &keys,
	);

	// Assert
	let config = data_paths_dirs::config_file(root);
	assert!(config.exists());
}

/// Load after corrupt config returns None
#[test]
fn load_corrupted_json_returns_none() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let cfg_path = data_paths_dirs::config_file(root);
	std::fs::create_dir_all(
		cfg_path.parent().unwrap(),
	)
	.unwrap();
	std::fs::write(&cfg_path, "not json")
		.unwrap();

	// Act
	let loaded =
		jira_project_cache::load_project_keys(root);

	// Assert
	assert!(loaded.is_none());
}
