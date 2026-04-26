use std::path::PathBuf;

use crate::indexer::{Symbol, SymbolKind};

#[derive(Debug, Clone)]
pub struct DefinitionHit {
	pub symbol: Symbol,
	pub fqn: String,
}

#[derive(Debug, Clone)]
pub struct ReferenceResult {
	pub definitions: Vec<UsageLocation>,
	pub references: Vec<UsageLocation>,
}

#[derive(Debug, Clone)]
pub struct UsageLocation {
	pub file: PathBuf,
	pub line: usize,
}

#[derive(Debug, Clone, Default)]
pub struct SymbolListOptions {
	pub file: Option<String>,
	pub kind: Option<SymbolKind>,
}

#[derive(Debug, Clone)]
pub struct SymbolEntry {
	pub symbol: Symbol,
	pub fqn: String,
}

#[derive(Debug, Clone)]
pub struct StructureResult {
	pub target: String,
	pub modules: Vec<ModuleStructure>,
	/// fallback: dirs available when no modules found
	pub available_dirs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ModuleStructure {
	pub path: PathBuf,
	pub submodules: Vec<SubmoduleInfo>,
	pub reexport_count: usize,
}

#[derive(Debug, Clone)]
pub struct SubmoduleInfo {
	pub name: String,
	pub is_public: bool,
	pub doc: Option<String>,
}
