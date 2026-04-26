use std::fmt;

use super::kind::SymbolKind;
use super::location::CodeLocation;
use super::visibility::Visibility;

#[derive(Debug, Clone)]
pub struct Symbol {
	pub name: String,
	pub kind: SymbolKind,
	pub location: CodeLocation,
	pub visibility: Visibility,
	pub signature: Option<String>,
	pub doc_comment: Option<String>,
	/// e.g. "module::struct::method"
	pub fqn: Option<String>,
	/// For methods, fields, variants
	pub parent: Option<String>,
	/// Full content for doc chunks
	pub content: Option<String>,
}

impl Symbol {
	pub fn new(
		name: String,
		kind: SymbolKind,
		location: CodeLocation,
	) -> Self {
		Self {
			name,
			kind,
			location,
			visibility: Visibility::default(),
			signature: None,
			doc_comment: None,
			fqn: None,
			parent: None,
			content: None,
		}
	}
}

impl fmt::Display for Symbol {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let vis = if self.visibility == Visibility::Private {
			String::new()
		} else {
			format!("{} ", self.visibility)
		};
		write!(
			f,
			"{}{} {} @ {}",
			vis, self.kind, self.name, self.location
		)
	}
}
