//! DocEntry text retrieval methods.

use crate::retrieval::docgen::entry::DocEntry;

/// Text generation for display and embedding.
impl DocEntry {
	/// Get combined documentation (user + LLM).
	pub fn combined_doc(&self) -> String {
		let mut parts = Vec::new();
		if let Some(ref comment) = self.user_comment {
			parts.push(comment.clone());
		}
		if let Some(ref llm) = self.llm_doc {
			parts.push(llm.clone());
		}
		parts.join("\n\n")
	}

	/// Get text for embedding (name + sig + docs).
	pub fn embedding_text(&self) -> String {
		let mut parts = vec![
			format!("{} {}", self.kind, self.name),
		];
		if let Some(ref sig) = self.signature {
			parts.push(sig.clone());
		}
		if let Some(ref comment) = self.user_comment {
			parts.push(comment.clone());
		}
		if let Some(ref doc) = self.llm_doc {
			parts.push(doc.clone());
		}
		parts.join(" ")
	}
}
