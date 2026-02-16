//! From trait implementations for FilePath.

use std::path::PathBuf;

use super::FilePath;

impl From<PathBuf> for FilePath {
	fn from(path: PathBuf) -> Self {
		Self(path)
	}
}

impl From<String> for FilePath {
	fn from(val: String) -> Self {
		Self(PathBuf::from(val))
	}
}

impl From<&str> for FilePath {
	fn from(val: &str) -> Self {
		Self(PathBuf::from(val))
	}
}
