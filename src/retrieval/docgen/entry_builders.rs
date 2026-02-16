//! DocEntry builder methods.

use crate::retrieval::docgen::entry::DocEntry;

/// Builder methods for fluent construction.
impl DocEntry {
	/// Builder: set user comment.
	pub fn with_user_comment(
		mut self, comment: String,
	) -> Self {
		self.user_comment = Some(comment);
		self
	}

	/// Builder: set signature.
	pub fn with_signature(
		mut self, sig: String,
	) -> Self {
		self.signature = Some(sig);
		self
	}

	/// Builder: set code snippet.
	pub fn with_code_snippet(
		mut self, snippet: String,
	) -> Self {
		self.code_snippet = snippet;
		self
	}

	/// Builder: set source mtime.
	pub fn with_mtime(mut self, mtime: u64) -> Self {
		self.source_mtime = mtime;
		self
	}
}
