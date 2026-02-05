//! Documentation entry types for the doc generator.
//!
//! This module defines the core data structures for storing
//! generated documentation and cross-references.

use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::indexer::SymbolKind;

/// Status of documentation generation for an entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DocStatus {
	/// not yet processed
	#[default]
	Pending,
	/// LLM is currently generating doc
	Generating,
	/// doc generation complete
	Ready,
	/// generation failed
	Failed,
}

impl fmt::Display for DocStatus {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			DocStatus::Pending => "pending",
			DocStatus::Generating => "generating",
			DocStatus::Ready => "ready",
			DocStatus::Failed => "failed",
		};
		write!(f, "{}", s)
	}
}

/// Kind of reference between symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceKind {
	/// function or method call
	Call,
	/// type usage in signature, field, variable
	TypeUsage,
	/// import or use statement
	Import,
	/// trait implementation
	TraitImpl,
	/// derive macro usage
	Derive,
	/// field access on struct/enum
	FieldAccess,
	/// external crate dependency
	ExternalCrate,
	/// generic type parameter
	GenericParam,
	/// return type usage
	ReturnType,
	/// parameter type usage
	ParamType,
}

impl fmt::Display for ReferenceKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			ReferenceKind::Call => "call",
			ReferenceKind::TypeUsage => "type_usage",
			ReferenceKind::Import => "import",
			ReferenceKind::TraitImpl => "trait_impl",
			ReferenceKind::Derive => "derive",
			ReferenceKind::FieldAccess => "field_access",
			ReferenceKind::ExternalCrate => "external_crate",
			ReferenceKind::GenericParam => "generic_param",
			ReferenceKind::ReturnType => "return_type",
			ReferenceKind::ParamType => "param_type",
		};
		write!(f, "{}", s)
	}
}

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
	pub fn new(file_path: PathBuf, line: usize, ref_kind: ReferenceKind) -> Self {
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
	pub fn with_containing_symbol(mut self, symbol: String) -> Self {
		self.containing_symbol = Some(symbol);
		self
	}
}

/// Cross-reference links between symbols.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SymbolLinks {
	/// symbols this one depends on (calls, uses types from)
	pub depends_on: Vec<String>,
	/// symbols that depend on this one
	pub depended_by: Vec<String>,
	/// parent module/struct/impl
	pub parent: Option<String>,
	/// child symbols (for modules, structs, impls)
	pub children: Vec<String>,
	/// related external crates
	pub external_deps: Vec<String>,
}

impl SymbolLinks {
	/// Create empty links.
	pub fn new() -> Self {
		Self::default()
	}

	/// Add a dependency.
	pub fn add_depends_on(&mut self, symbol: String) {
		if !self.depends_on.contains(&symbol) {
			self.depends_on.push(symbol);
		}
	}

	/// Add a dependent.
	pub fn add_depended_by(&mut self, symbol: String) {
		if !self.depended_by.contains(&symbol) {
			self.depended_by.push(symbol);
		}
	}

	/// Add a child symbol.
	pub fn add_child(&mut self, symbol: String) {
		if !self.children.contains(&symbol) {
			self.children.push(symbol);
		}
	}

	/// Add external dependency.
	pub fn add_external_dep(&mut self, crate_name: String) {
		if !self.external_deps.contains(&crate_name) {
			self.external_deps.push(crate_name);
		}
	}
}

/// Documentation entry for a code symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocEntry {
	/// unique identifier (hash of file_path + symbol_name + line)
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
	/// last modification time of source file (unix timestamp)
	pub source_mtime: u64,
	/// embedding vector for the documentation
	pub doc_embedding: Option<Vec<f32>>,
}

impl DocEntry {
	/// Create a new doc entry from symbol info.
	pub fn new(
		name: String,
		kind: SymbolKind,
		file_path: PathBuf,
		line: usize,
	) -> Self {
		let id = Self::generate_id(&file_path, &name, line);
		Self {
			id,
			name,
			kind,
			file_path,
			line,
			user_comment: None,
			llm_doc: None,
			signature: None,
			code_snippet: String::new(),
			references: Vec::new(),
			links: SymbolLinks::new(),
			status: DocStatus::Pending,
			source_mtime: 0,
			doc_embedding: None,
		}
	}

	/// Generate unique ID for entry.
	pub fn generate_id(file_path: &PathBuf, name: &str, line: usize) -> String {
		use std::collections::hash_map::DefaultHasher;
		use std::hash::{Hash, Hasher};

		let mut hasher = DefaultHasher::new();
		file_path.hash(&mut hasher);
		name.hash(&mut hasher);
		line.hash(&mut hasher);
		format!("{:016x}", hasher.finish())
	}

	/// Builder: set user comment.
	pub fn with_user_comment(mut self, comment: String) -> Self {
		self.user_comment = Some(comment);
		self
	}

	/// Builder: set signature.
	pub fn with_signature(mut self, signature: String) -> Self {
		self.signature = Some(signature);
		self
	}

	/// Builder: set code snippet.
	pub fn with_code_snippet(mut self, snippet: String) -> Self {
		self.code_snippet = snippet;
		self
	}

	/// Builder: set source mtime.
	pub fn with_mtime(mut self, mtime: u64) -> Self {
		self.source_mtime = mtime;
		self
	}

	/// Check if entry needs regeneration.
	pub fn is_stale(&self, current_mtime: u64) -> bool {
		self.source_mtime < current_mtime
	}

	/// Check if doc is ready.
	pub fn is_ready(&self) -> bool {
		self.status == DocStatus::Ready
	}

	/// Mark as generating.
	pub fn mark_generating(&mut self) {
		self.status = DocStatus::Generating;
	}

	/// Mark as ready with generated doc.
	pub fn mark_ready(&mut self, llm_doc: String) {
		self.llm_doc = Some(llm_doc);
		self.status = DocStatus::Ready;
	}

	/// Mark as failed.
	pub fn mark_failed(&mut self) {
		self.status = DocStatus::Failed;
	}

	/// Get combined documentation (user + LLM).
	pub fn combined_doc(&self) -> String {
		let mut parts = Vec::new();
		if let Some(ref user) = self.user_comment {
			parts.push(user.clone());
		}
		if let Some(ref llm) = self.llm_doc {
			parts.push(llm.clone());
		}
		parts.join("\n\n")
	}

	/// Get text for embedding (name + signature + docs).
	pub fn embedding_text(&self) -> String {
		let mut parts = vec![
			format!("{} {}", self.kind, self.name),
		];
		if let Some(ref sig) = self.signature {
			parts.push(sig.clone());
		}
		if let Some(ref doc) = self.user_comment {
			parts.push(doc.clone());
		}
		if let Some(ref doc) = self.llm_doc {
			parts.push(doc.clone());
		}
		parts.join(" ")
	}
}

impl fmt::Display for DocEntry {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{} {} @ {}:{} [{}]",
			self.kind,
			self.name,
			self.file_path.display(),
			self.line,
			self.status
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_doc_entry_id_generation() {
		let path = PathBuf::from("src/main.rs");
		let entry1 = DocEntry::new(
			"main".to_string(),
			SymbolKind::Function,
			path.clone(),
			10,
		);
		let entry2 = DocEntry::new(
			"main".to_string(),
			SymbolKind::Function,
			path.clone(),
			10,
		);
		// same inputs should produce same ID
		assert_eq!(entry1.id, entry2.id);

		let entry3 = DocEntry::new(
			"main".to_string(),
			SymbolKind::Function,
			path,
			20, // different line
		);
		// different line should produce different ID
		assert_ne!(entry1.id, entry3.id);
	}

	#[test]
	fn test_doc_status_lifecycle() {
		let mut entry = DocEntry::new(
			"test".to_string(),
			SymbolKind::Function,
			PathBuf::from("test.rs"),
			1,
		);
		assert_eq!(entry.status, DocStatus::Pending);
		assert!(!entry.is_ready());

		entry.mark_generating();
		assert_eq!(entry.status, DocStatus::Generating);

		entry.mark_ready("Generated doc".to_string());
		assert_eq!(entry.status, DocStatus::Ready);
		assert!(entry.is_ready());
		assert_eq!(entry.llm_doc, Some("Generated doc".to_string()));
	}

	#[test]
	fn test_combined_doc() {
		let mut entry = DocEntry::new(
			"test".to_string(),
			SymbolKind::Function,
			PathBuf::from("test.rs"),
			1,
		);
		entry.user_comment = Some("User doc".to_string());
		entry.llm_doc = Some("LLM doc".to_string());

		let combined = entry.combined_doc();
		assert!(combined.contains("User doc"));
		assert!(combined.contains("LLM doc"));
	}

	#[test]
	fn test_symbol_links() {
		let mut links = SymbolLinks::new();
		links.add_depends_on("foo".to_string());
		links.add_depends_on("foo".to_string()); // duplicate
		links.add_depended_by("bar".to_string());

		assert_eq!(links.depends_on.len(), 1);
		assert_eq!(links.depended_by.len(), 1);
	}
}
