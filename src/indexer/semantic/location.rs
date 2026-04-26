use std::path::Path;

use super::types::{Definition, SymbolReference};
use super::SemanticGraph;

impl SemanticGraph {
	/// Find definition at a specific location (for "go to definition")
	pub fn definition_at(
		&self,
		file: &Path,
		line: usize,
	) -> Option<&Definition> {
		self.definitions_by_file.get(file).and_then(|defs| {
			defs.iter().find(|def| def.symbol.location.line == line)
		})
	}

	pub fn definitions_in_file(&self, file: &Path) -> Vec<&Definition> {
		self.definitions_by_file
			.get(file)
			.map(|defs| defs.iter().collect())
			.unwrap_or_default()
	}

	pub fn references_in_file(&self, file: &Path) -> Vec<&SymbolReference> {
		self.references_by_file
			.get(file)
			.map(|refs| refs.iter().collect())
			.unwrap_or_default()
	}
}
