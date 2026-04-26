use std::path::PathBuf;
use notify::EventKind;

/// A file change detected by the watcher
#[derive(Debug, Clone)]
pub struct FileChangeEvent {
	pub paths: Vec<PathBuf>,
	pub kind: ChangeKind,
}

/// Categorized file system change
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
	Created,
	Modified,
	Deleted,
	Other,
}

impl From<&EventKind> for ChangeKind {
	fn from(kind: &EventKind) -> Self {
		match kind {
			EventKind::Create(_) => Self::Created,
			EventKind::Modify(_) => Self::Modified,
			EventKind::Remove(_) => Self::Deleted,
			_ => Self::Other,
		}
	}
}
