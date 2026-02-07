//! From/AsRef implementations for FileName.

use super::FileName;

impl From<String> for FileName {
	fn from(s: String) -> Self {
		Self(s)
	}
}

impl From<&str> for FileName {
	fn from(s: &str) -> Self {
		Self(s.to_string())
	}
}

impl AsRef<str> for FileName {
	fn as_ref(&self) -> &str {
		&self.0
	}
}
