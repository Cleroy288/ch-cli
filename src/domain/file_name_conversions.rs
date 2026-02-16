//! From/AsRef implementations for FileName.

use super::FileName;

impl From<String> for FileName {
	fn from(val: String) -> Self {
		Self(val)
	}
}

impl From<&str> for FileName {
	fn from(val: &str) -> Self {
		Self(val.to_string())
	}
}

impl AsRef<str> for FileName {
	fn as_ref(&self) -> &str {
		&self.0
	}
}
