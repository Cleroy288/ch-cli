//! DocEntry construction and ID generation.

use std::path::PathBuf;

use crate::indexer::SymbolKind;
use crate::retrieval::docgen::entry::DocEntry;
use crate::retrieval::docgen::entry_types::DocStatus;
use crate::retrieval::docgen::symbol_links::SymbolLinks;

/// Construction and ID generation.
impl DocEntry {
	/// Create a new doc entry from symbol info.
	pub fn new(
		name: String,
		kind: SymbolKind,
		file_path: PathBuf,
		line: usize,
	) -> Self {
		let entry_id =
			Self::generate_id(&file_path, &name, line);
		Self {
			id: entry_id, name, kind, file_path, line,
			user_comment: None, llm_doc: None,
			signature: None,
			code_snippet: String::new(),
			references: Vec::new(),
			links: SymbolLinks::new(),
			status: DocStatus::Pending,
			source_mtime: 0, doc_embedding: None,
		}
	}

	/// Generate unique ID for entry.
	pub fn generate_id(
		file_path: &PathBuf,
		name: &str,
		line: usize,
	) -> String {
		use std::collections::hash_map::DefaultHasher;
		use std::hash::{Hash, Hasher};
		let mut hasher = DefaultHasher::new();
		file_path.hash(&mut hasher);
		name.hash(&mut hasher);
		line.hash(&mut hasher);
		format!("{:016x}", hasher.finish())
	}
}
