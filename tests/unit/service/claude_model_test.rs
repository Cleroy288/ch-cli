//! Unit tests for claude model persistence.

use rustean::domain::data_paths_dirs;
use rustean::service::claude::model::{
	is_valid_model, load_model, save_model,
	DEFAULT_MODEL,
};

#[test]
fn load_missing_file_returns_default() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let path = dir.path().to_str().unwrap();

	// Act
	let result = load_model(path);

	// Assert
	assert_eq!(result, DEFAULT_MODEL);
}

#[test]
fn save_and_load_roundtrip() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let path = dir.path().to_str().unwrap();

	// Act
	save_model(path, "opus");
	let result = load_model(path);

	// Assert
	assert_eq!(result, "opus");
}

/// Valid and invalid model names
#[test]
fn is_valid_model_cases() {
	let cases = [
		("haiku", true),
		("sonnet", true),
		("opus", true),
		("gpt4", false),
		("", false),
		("claude", false),
		("Sonnet", false),
		("HAIKU", false),
		("Opus", false),
	];
	for (input, expected) in cases {
		assert_eq!(
			is_valid_model(input), expected,
			"is_valid_model({input:?})",
		);
	}
}

#[test]
fn load_invalid_model_returns_default() {
	// Arrange — write invalid model in config
	let dir = tempfile::tempdir().unwrap();
	let cfg_path =
		data_paths_dirs::config_file(dir.path());
	std::fs::create_dir_all(
		cfg_path.parent().unwrap(),
	)
	.unwrap();
	std::fs::write(
		&cfg_path,
		r#"{"claude":{"model":"gpt4"}}"#,
	)
	.unwrap();
	let path = dir.path().to_str().unwrap();

	// Act
	let result = load_model(path);

	// Assert
	assert_eq!(result, DEFAULT_MODEL);
}

/// Corrupt config.json returns default model
#[test]
fn load_corrupt_json_returns_default() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let cfg_path =
		data_paths_dirs::config_file(dir.path());
	std::fs::create_dir_all(
		cfg_path.parent().unwrap(),
	)
	.unwrap();
	std::fs::write(
		&cfg_path,
		"not valid json {{{",
	)
	.unwrap();
	let path = dir.path().to_str().unwrap();

	// Act
	let result = load_model(path);

	// Assert
	assert_eq!(result, DEFAULT_MODEL);
}
