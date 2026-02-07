//! Symbol struct representing a code element extracted from source.

use std::fmt;

use super::kind::SymbolKind;
use super::location::CodeLocation;
use super::visibility::Visibility;

/// A code symbol extracted from source code.
#[derive(Debug, Clone)]
pub struct Symbol {
	/// Name of the symbol
	pub name: String,
	/// Kind of symbol
	pub kind: SymbolKind,
	/// Location in source code
	pub location: CodeLocation,
	/// Visibility modifier
	pub visibility: Visibility,
	/// Function/method signature (if applicable)
	pub signature: Option<String>,
	/// Documentation comment (if any)
	pub doc_comment: Option<String>,
	/// Fully qualified name (e.g., "module::struct::method")
	pub fqn: Option<String>,
	/// Parent symbol name (for methods, fields, variants)
	pub parent: Option<String>,
	/// Full content for documentation chunks
	pub content: Option<String>,
}

impl Symbol {
	/// Create a new Symbol with required fields
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
