//! Tests for MCP server registration check.

use rustean::startup::mcp_registration::{
	has_old_entry, has_valid_entry,
};

/// Valid config with matching binary path
#[test]
fn has_valid_entry_matching_path_returns_true() {
	// Arrange
	let json = r#"{
		"mcpServers": {
			"rustean": {
				"type": "stdio",
				"command": "/usr/bin/rustean",
				"args": ["mcp-server"]
			}
		}
	}"#;

	// Act
	let result =
		has_valid_entry(json, "/usr/bin/rustean");

	// Assert
	assert!(result);
}

/// Config with different binary path
#[test]
fn has_valid_entry_wrong_path_returns_false() {
	// Arrange
	let json = r#"{
		"mcpServers": {
			"rustean": {
				"type": "stdio",
				"command": "/old/path/rustean",
				"args": ["mcp-server"]
			}
		}
	}"#;

	// Act
	let result =
		has_valid_entry(json, "/new/path/rustean");

	// Assert
	assert!(!result);
}

/// Empty config file
#[test]
fn has_valid_entry_empty_json_returns_false() {
	// Arrange / Act
	let result =
		has_valid_entry("{}", "/usr/bin/rustean");

	// Assert
	assert!(!result);
}

/// Invalid JSON content
#[test]
fn has_valid_entry_invalid_json_returns_false() {
	// Arrange / Act
	let result = has_valid_entry(
		"not json", "/usr/bin/rustean",
	);

	// Assert
	assert!(!result);
}

/// Config with other servers but not rustean
#[test]
fn has_valid_entry_other_server_returns_false() {
	// Arrange
	let json = r#"{
		"mcpServers": {
			"other-server": {
				"type": "stdio",
				"command": "/usr/bin/rustean"
			}
		}
	}"#;

	// Act
	let result =
		has_valid_entry(json, "/usr/bin/rustean");

	// Assert
	assert!(!result);
}

/// Old `rustean-memory` entry is detected
#[test]
fn has_old_entry_present_returns_true() {
	// Arrange
	let json = r#"{
		"mcpServers": {
			"rustean-memory": {
				"type": "stdio",
				"command": "/usr/bin/rustean"
			}
		}
	}"#;

	// Act / Assert
	assert!(has_old_entry(json));
}

/// No old entry when only `rustean` exists
#[test]
fn has_old_entry_absent_returns_false() {
	// Arrange
	let json = r#"{
		"mcpServers": {
			"rustean": {
				"type": "stdio",
				"command": "/usr/bin/rustean"
			}
		}
	}"#;

	// Act / Assert
	assert!(!has_old_entry(json));
}

/// Invalid JSON returns false
#[test]
fn has_old_entry_invalid_json_returns_false() {
	// Arrange / Act / Assert
	assert!(!has_old_entry("not json"));
}
