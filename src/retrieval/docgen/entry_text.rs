//! DocEntry text retrieval methods.

use crate::retrieval::docgen::entry::DocEntry;

/// Text generation for display and embedding.
impl DocEntry {
	/// Get combined documentation (user + LLM).
	pub fn combined_doc(&self) -> String {
		let mut parts = Vec::new();
		if let Some(ref u) = self.user_comment {
			parts.push(u.clone());
		}
		if let Some(ref l) = self.llm_doc {
			parts.push(l.clone());
		}
		parts.join("\n\n")
	}

	/// Get text for embedding (name + sig + docs).
	pub fn embedding_text(&self) -> String {
		let mut parts = vec![
			format!("{} {}", self.kind, self.name),
		];
		if let Some(ref s) = self.signature {
			parts.push(s.clone());
		}
		if let Some(ref d) = self.user_comment {
			parts.push(d.clone());
		}
		if let Some(ref d) = self.llm_doc {
			parts.push(d.clone());
		}
		parts.join(" ")
	}
}
