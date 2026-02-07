//! Documentation entry for a code symbol.
//!
//! Core DocEntry struct definition.

use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::indexer::SymbolKind;
use crate::retrieval::docgen::entry_types::DocStatus;
use crate::retrieval::docgen::reference_location::ReferenceLocation;
use crate::retrieval::docgen::symbol_links::SymbolLinks;

/// Documentation entry for a code symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocEntry {
	/// unique identifier (hash of file + name + line)
	pub id: String,
	/// symbol name
	pub name: String,
	/// symbol kind (Function, Struct, Impl, etc.)
	pub kind: SymbolKind,
	/// file path
	pub file_path: PathBuf,
	/// line number (1-indexed)
	pub line: usize,
	/// user-written doc comment (/// or //!)
	pub user_comment: Option<String>,
	/// LLM-generated documentation
	pub llm_doc: Option<String>,
	/// function/method signature
	pub signature: Option<String>,
	/// source code snippet (for context)
	pub code_snippet: String,
	/// places where this symbol is referenced
	pub references: Vec<ReferenceLocation>,
	/// cross-reference links to other symbols
	pub links: SymbolLinks,
	/// generation status
	pub status: DocStatus,
	/// source file modification time (unix timestamp)
	pub source_mtime: u64,
	/// embedding vector for the documentation
	pub doc_embedding: Option<Vec<f32>>,
}

impl fmt::Display for DocEntry {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f, "{} {} @ {}:{} [{}]",
			self.kind, self.name,
			self.file_path.display(),
			self.line, self.status
		)
	}
}

