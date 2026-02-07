//! Symbol containment and relationship finding.

use std::collections::HashSet;

use crate::indexer::semantic::SemanticGraph;
use crate::indexer::Symbol;

/// Find the symbol that contains a given location.
pub(crate) fn find_containing_symbol(
	symbols: &[Symbol],
	location: &crate::indexer::CodeLocation,
) -> Option<String> {
	symbols
		.iter()
		.filter(|s| {
			s.location.file == location.file
				&& s.location.line <= location.line
				&& matches!(
					s.kind,
					crate::indexer::SymbolKind::Function
						| crate::indexer::SymbolKind::Method
						| crate::indexer::SymbolKind::Impl
				)
		})
		.max_by_key(|s| s.location.line)
		.map(|s| s.name.clone())
}

/// Find symbols that this symbol depends on.
pub(crate) fn find_dependencies(
	graph: &SemanticGraph,
	symbols: &[Symbol],
	symbol_name: &str,
) -> Vec<String> {
	let symbol =
		match symbols.iter().find(|s| s.name == symbol_name) {
			Some(s) => s,
			None => return Vec::new(),
		};

	let refs =
		graph.references_in_file(&symbol.location.file);

	let mut deps = HashSet::new();
	for ref_loc in refs {
		// references within ~100 lines after the symbol
		let in_body =
			ref_loc.location.line > symbol.location.line
				&& ref_loc.location.line
					< symbol.location.line + 100;
		if !in_body {
			continue;
		}
		if !graph.find_definitions(&ref_loc.name).is_empty()
		{
			deps.insert(ref_loc.name.clone());
		}
	}

	deps.into_iter().collect()
}

/// Find child symbols (methods, variants, etc.).
pub(crate) fn find_children(
	symbols: &[Symbol],
	parent_name: &str,
) -> Vec<String> {
	symbols
		.iter()
		.filter(|s| {
			s.parent
				.as_ref()
				.map(|p| p == parent_name)
				.unwrap_or(false)
		})
		.map(|s| s.name.clone())
		.collect()
}

/// Extract external crate dependencies from code snippet.
pub(crate) fn extract_external_deps(
	code: &str,
) -> Vec<String> {
	let linker =
		crate::retrieval::docgen::linker::DocLinker::new();
	linker.extract_crates(code)
}
