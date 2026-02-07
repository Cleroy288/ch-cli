//! From trait implementations for FilePath.

use std::path::PathBuf;

use super::FilePath;

impl From<PathBuf> for FilePath {
	fn from(path: PathBuf) -> Self {
		Self(path)
	}
}

impl From<String> for FilePath {
	fn from(s: String) -> Self {
		Self(PathBuf::from(s))
	}
}

impl From<&str> for FilePath {
	fn from(s: &str) -> Self {
		Self(PathBuf::from(s))
	}
}
