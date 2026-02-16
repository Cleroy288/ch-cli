//! Persist Claude session_id to disk.
//!
//! Stores the active session_id in
//! `.rustean-index/claude_session.json` so
//! conversations survive restarts.

use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::claude::ClaudeSession;
use crate::indexer::state::INDEX_DIR_NAME;

/// Session file name inside the index directory
const SESSION_FILE: &str = "claude_session.json";

/// Serde target for the session file
#[derive(
	serde::Serialize, serde::Deserialize,
)]
struct SessionData {
	session_id: String,
}

/// Build the session file path for a project
fn session_path(project: &str) -> PathBuf {
	Path::new(project)
		.join(INDEX_DIR_NAME)
		.join(SESSION_FILE)
}

/// Save session_id to disk.
///
/// Creates the index directory if needed.
/// Silently ignores write failures (best-effort).
pub fn save_session(
	project: &str,
	session: &ClaudeSession,
) {
	let Some(sid) = &session.session_id else {
		return;
	};
	let path = session_path(project);
	let _ = fs::create_dir_all(
		path.parent().unwrap_or(Path::new(".")),
	);
	let data = SessionData {
		session_id: sid.clone(),
	};
	let json = serde_json::to_string(&data)
		.unwrap_or_default();
	let _ = fs::write(&path, json);
}

/// Load session_id from disk.
///
/// Returns a session with the stored id, or
/// a default empty session if file is missing.
pub fn load_session(
	project: &str,
) -> ClaudeSession {
	let path = session_path(project);
	let Ok(content) = fs::read_to_string(&path)
	else {
		return ClaudeSession::default();
	};
	let Ok(data) =
		serde_json::from_str::<SessionData>(&content)
	else {
		return ClaudeSession::default();
	};
	ClaudeSession {
		session_id: Some(data.session_id),
	}
}

/// Clear the persisted session (for /new command).
///
/// Deletes the session file from disk.
pub fn clear_session(project: &str) {
	let path = session_path(project);
	let _ = fs::remove_file(&path);
}
