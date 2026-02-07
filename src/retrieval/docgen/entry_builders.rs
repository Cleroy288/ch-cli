//! DocEntry builder methods.

use crate::retrieval::docgen::entry::DocEntry;

/// Builder methods for fluent construction.
impl DocEntry {
	/// Builder: set user comment.
	pub fn with_user_comment(mut self, c: String) -> Self {
		self.user_comment = Some(c);
		self
	}

	/// Builder: set signature.
	pub fn with_signature(mut self, s: String) -> Self {
		self.signature = Some(s);
		self
	}

	/// Builder: set code snippet.
	pub fn with_code_snippet(mut self, s: String) -> Self {
		self.code_snippet = s;
		self
	}

	/// Builder: set source mtime.
	pub fn with_mtime(mut self, mtime: u64) -> Self {
		self.source_mtime = mtime;
		self
	}
}
