//! Tests for config migration (6 files → config.json).

use std::fs;

use rustean::domain::data_paths;
use rustean::domain::data_paths_dirs;
use rustean::service::config;
use rustean::startup::migration_config;

/// Helper: create project data directory
fn make_data_dir(
	root: &std::path::Path,
) -> std::path::PathBuf {
	let data = data_paths::data_dir(root);
	fs::create_dir_all(&data).unwrap();
	data
}

/// Migration from old credentials.json
#[test]
fn migrates_credentials_file() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let data = make_data_dir(root);
	let creds = r#"{
		"bitbucket":{"email":"a@b.c",
		"token":"t","workspace":"ws"},
		"jira":{"email":"a@b.c","token":"t",
		"base_url":"https://x.atlassian.net",
		"cloud_id":"cid"}
	}"#;
	fs::write(
		data.join("credentials.json"),
		creds,
	)
	.unwrap();

	// Act
	migration_config::migrate_to_unified_config(
		root,
	);

	// Assert
	let cfg = config::load_config(root);
	assert!(cfg.credentials.is_some());
}

/// Migration skips when config.json exists
#[test]
fn skips_when_config_exists() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let cfg_path = data_paths_dirs::config_file(root);
	fs::create_dir_all(cfg_path.parent().unwrap())
		.unwrap();
	let json = r#"{"claude":{"model":"opus"}}"#;
	fs::write(&cfg_path, json).unwrap();

	// Act
	migration_config::migrate_to_unified_config(
		root,
	);
	let cfg = config::load_config(root);

	// Assert — kept existing config
	assert_eq!(cfg.claude.model, "opus");
}

/// Migration is idempotent (second run is no-op)
#[test]
fn idempotent_rerun() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let data = make_data_dir(root);
	let agent = r#"{"kind":"claude_code"}"#;
	fs::write(data.join("agent.json"), agent)
		.unwrap();

	// Act — run twice
	migration_config::migrate_to_unified_config(
		root,
	);
	migration_config::migrate_to_unified_config(
		root,
	);

	// Assert
	let cfg = config::load_config(root);
	assert!(cfg.agent.is_some());
}

/// Migration deletes old files
#[test]
fn deletes_old_files_after_migration() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let root = dir.path();
	let data = make_data_dir(root);
	fs::write(data.join("agent.json"), "{}")
		.unwrap();

	// Act
	migration_config::migrate_to_unified_config(
		root,
	);

	// Assert
	assert!(!data.join("agent.json").exists());
	let cfg_path = data_paths_dirs::config_file(root);
	assert!(cfg_path.exists());
}
