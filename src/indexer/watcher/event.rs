//! File change event types.

use std::path::PathBuf;
use notify::EventKind;

/// A file change event
#[derive(Debug, Clone)]
pub struct FileChangeEvent {
	/// The paths that changed
	pub paths: Vec<PathBuf>,
	/// The kind of change
	pub kind: ChangeKind,
}

/// The kind of file change
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
	/// File was created
	Created,
	/// File was modified
	Modified,
	/// File was deleted
	Deleted,
	/// File was renamed (from/to)
	Renamed,
	/// Unknown change
	Other,
}

impl From<&EventKind> for ChangeKind {
	/// Converts a notify EventKind to ChangeKind
	fn from(kind: &EventKind) -> Self {
		match kind {
			EventKind::Create(_) => ChangeKind::Created,
			EventKind::Modify(_) => ChangeKind::Modified,
			EventKind::Remove(_) => ChangeKind::Deleted,
			_ => ChangeKind::Other,
		}
	}
}
