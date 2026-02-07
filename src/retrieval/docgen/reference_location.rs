//! Reference location tracking.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::retrieval::docgen::entry_types::ReferenceKind;

/// Location where a symbol is referenced.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceLocation {
	/// file where reference occurs
	pub file_path: PathBuf,
	/// line number (1-indexed)
	pub line: usize,
	/// surrounding code context (3-5 lines)
	pub context: String,
	/// what kind of reference
	pub ref_kind: ReferenceKind,
	/// module path (e.g., "retrieval::hybrid::embedding")
	pub module_path: Option<String>,
	/// name of the containing function/struct (if any)
	pub containing_symbol: Option<String>,
}

impl ReferenceLocation {
	/// Create a new reference location.
	pub fn new(
		file_path: PathBuf,
		line: usize,
		ref_kind: ReferenceKind,
	) -> Self {
		Self {
			file_path,
			line,
			context: String::new(),
			ref_kind,
			module_path: None,
			containing_symbol: None,
		}
	}

	/// Builder: set context.
	pub fn with_context(mut self, context: String) -> Self {
		self.context = context;
		self
	}

	/// Builder: set module path.
	pub fn with_module_path(mut self, path: String) -> Self {
		self.module_path = Some(path);
		self
	}

	/// Builder: set containing symbol.
	pub fn with_containing_symbol(
		mut self,
		symbol: String,
	) -> Self {
		self.containing_symbol = Some(symbol);
		self
	}
}

