//! Tests for Claude session persistence.

use rustean::domain::claude::ClaudeSession;
use rustean::service::claude::{
	clear_session, load_session, save_session,
};

/// Save and load round-trips session_id
#[test]
fn save_then_load_returns_session_id() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let path = dir.path().display().to_string();
	let session = ClaudeSession {
		session_id: Some("sess-abc".to_string()),
	};

	// Act
	save_session(&path, &session);
	let loaded = load_session(&path);

	// Assert
	assert_eq!(
		loaded.session_id.as_deref(),
		Some("sess-abc"),
	);
}

/// Load from missing file returns empty session
#[test]
fn load_missing_file_returns_default() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let path = dir.path().display().to_string();

	// Act
	let loaded = load_session(&path);

	// Assert
	assert!(loaded.session_id.is_none());
}

/// Clear removes the session file
#[test]
fn clear_removes_persisted_session() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let path = dir.path().display().to_string();
	let session = ClaudeSession {
		session_id: Some("sess-xyz".to_string()),
	};
	save_session(&path, &session);

	// Act
	clear_session(&path);
	let loaded = load_session(&path);

	// Assert
	assert!(loaded.session_id.is_none());
}

/// Save with None session_id is a no-op
#[test]
fn save_none_session_is_noop() {
	// Arrange
	let dir = tempfile::tempdir().unwrap();
	let path = dir.path().display().to_string();
	let session = ClaudeSession::default();

	// Act
	save_session(&path, &session);
	let loaded = load_session(&path);

	// Assert
	assert!(loaded.session_id.is_none());
}
