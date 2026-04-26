use std::path::PathBuf;

use fxhash::FxHashMap;

use super::types::{
	Definition, SemanticStats, SymbolReference,
};

type Def = Definition;
type SymRef = SymbolReference;

pub struct SemanticGraph {
	pub(super) definitions_by_name:
		FxHashMap<String, Vec<Def>>,
	pub(super) definitions_by_file:
		FxHashMap<PathBuf, Vec<Def>>,
	pub(super) references_by_name:
		FxHashMap<String, Vec<SymRef>>,
	pub(super) references_by_file:
		FxHashMap<PathBuf, Vec<SymRef>>,
	pub(super) scope_parents:
		FxHashMap<String, String>,
}

impl SemanticGraph {
	pub fn new() -> Self {
		Self {
			definitions_by_name:
				FxHashMap::default(),
			definitions_by_file:
				FxHashMap::default(),
			references_by_name:
				FxHashMap::default(),
			references_by_file:
				FxHashMap::default(),
			scope_parents: FxHashMap::default(),
		}
	}

	pub fn stats(&self) -> SemanticStats {
		SemanticStats {
			total_definitions: self
				.definitions_by_name
				.values()
				.map(|defs| defs.len())
				.sum(),
			total_references: self
				.references_by_name
				.values()
				.map(|refs| refs.len())
				.sum(),
			unique_symbols: self
				.definitions_by_name
				.len(),
			files_analyzed: self
				.definitions_by_file
				.len(),
		}
	}
}

impl Default for SemanticGraph {
	fn default() -> Self {
		Self::new()
	}
}
