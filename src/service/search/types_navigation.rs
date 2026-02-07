//! DTOs for navigation service operations.

use std::path::PathBuf;

use crate::indexer::{Symbol, SymbolKind};

/// A definition location
#[derive(Debug, Clone)]
pub struct DefinitionHit {
	/// symbol definition
	pub symbol: Symbol,
	/// fully qualified name
	pub fqn: String,
}

/// Result of a reference search
#[derive(Debug, Clone)]
pub struct ReferenceResult {
	/// definition locations
	pub definitions: Vec<UsageLocation>,
	/// reference locations
	pub references: Vec<UsageLocation>,
}

/// A usage location
#[derive(Debug, Clone)]
pub struct UsageLocation {
	/// file path
	pub file: PathBuf,
	/// line number
	pub line: usize,
}

/// Options for listing symbols
#[derive(Debug, Clone, Default)]
pub struct SymbolListOptions {
	/// filter by file path
	pub file: Option<String>,
	/// filter by symbol kind
	pub kind: Option<SymbolKind>,
}

/// A symbol entry in a listing
#[derive(Debug, Clone)]
pub struct SymbolEntry {
	/// symbol data
	pub symbol: Symbol,
	/// fully qualified name
	pub fqn: String,
}

/// Structure query result
#[derive(Debug, Clone)]
pub struct StructureResult {
	/// target being queried
	pub target: String,
	/// found modules
	pub modules: Vec<ModuleStructure>,
	/// available directories (if no modules found)
	pub available_dirs: Vec<String>,
}

/// Module structure info
#[derive(Debug, Clone)]
pub struct ModuleStructure {
	/// module file path
	pub path: PathBuf,
	/// submodules
	pub submodules: Vec<SubmoduleInfo>,
	/// re-export count
	pub reexport_count: usize,
}

/// Submodule info
#[derive(Debug, Clone)]
pub struct SubmoduleInfo {
	/// module name
	pub name: String,
	/// whether public
	pub is_public: bool,
	/// doc comment
	pub doc: Option<String>,
}
