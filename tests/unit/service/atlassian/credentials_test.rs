//! Tests for Atlassian credential persistence.

use rustean::service::atlassian::credentials::{
	has_credentials, load_credentials,
	save_credentials,
};
use rustean::service::atlassian::types::{
	AtlassianCredentials, BbCredentials,
	JiraCredentials,
};
use tempfile::TempDir;

/// load_credentials returns None for missing file
#[test]
fn load_missing_file_returns_none() {
	// Arrange
	let dir = TempDir::new().unwrap();

	// Act
	let result =
		load_credentials(dir.path());

	// Assert
	assert!(result.is_none());
}

/// has_credentials returns false when empty
#[test]
fn has_credentials_false_when_missing() {
	// Arrange
	let dir = TempDir::new().unwrap();

	// Act / Assert
	assert!(!has_credentials(dir.path()));
}

/// save then load roundtrips correctly
#[test]
fn save_and_load_roundtrip() {
	// Arrange
	let dir = TempDir::new().unwrap();
	let creds = AtlassianCredentials {
		bitbucket: Some(BbCredentials {
			email: "a@b.com".to_string(),
			token: "tok123".to_string(),
			workspace: "my-ws".to_string(),
			repo_slug: None,
		}),
		jira: Some(JiraCredentials {
			email: "c@d.com".to_string(),
			token: "jtok456".to_string(),
			base_url: "https://x.atlassian.net"
				.to_string(),
			cloud_id: "cloud-x".to_string(),
			project_key: None,
		}),
	};

	// Act
	save_credentials(dir.path(), &creds).unwrap();
	let loaded =
		load_credentials(dir.path()).unwrap();

	// Assert
	let bitbucket = loaded.bitbucket.unwrap();
	assert_eq!(bitbucket.email, "a@b.com");
	assert_eq!(bitbucket.workspace, "my-ws");
	let jira = loaded.jira.unwrap();
	assert_eq!(jira.cloud_id, "cloud-x");
	assert_eq!(
		jira.base_url,
		"https://x.atlassian.net",
	);
}

/// save with None sections roundtrips
#[test]
fn save_partial_creds_roundtrip() {
	// Arrange
	let dir = TempDir::new().unwrap();
	let creds = AtlassianCredentials {
		bitbucket: None,
		jira: None,
	};

	// Act
	save_credentials(dir.path(), &creds).unwrap();
	let loaded =
		load_credentials(dir.path()).unwrap();

	// Assert
	assert!(loaded.bitbucket.is_none());
	assert!(loaded.jira.is_none());
}

/// load returns None for corrupt config.json
#[test]
fn load_invalid_json_returns_none() {
	// Arrange
	let dir = TempDir::new().unwrap();
	let config_path =
		rustean::domain::data_paths_dirs::config_file(
			dir.path(),
		);
	std::fs::create_dir_all(
		config_path.parent().unwrap(),
	)
	.unwrap();
	std::fs::write(&config_path, "not json")
		.unwrap();

	// Act
	let result =
		load_credentials(dir.path());

	// Assert
	assert!(result.is_none());
}
