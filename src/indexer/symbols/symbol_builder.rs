use super::symbol::Symbol;
use super::visibility::Visibility;

impl Symbol {
	/// Builder method to set content
	pub fn with_content(mut self, content: String) -> Self {
		self.content = Some(content);
		self
	}

	/// Builder method to set visibility
	pub fn with_visibility(mut self, visibility: Visibility) -> Self {
		self.visibility = visibility;
		self
	}

	/// Builder method to set signature
	pub fn with_signature(mut self, signature: String) -> Self {
		self.signature = Some(signature);
		self
	}

	/// Builder method to set doc comment
	pub fn with_doc_comment(mut self, doc: String) -> Self {
		self.doc_comment = Some(doc);
		self
	}

	/// Builder method to set parent
	pub fn with_parent(mut self, parent: String) -> Self {
		self.parent = Some(parent);
		self
	}
}
