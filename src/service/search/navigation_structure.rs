use std::path::Path;

use crate::domain::errors::search::SearchError;
use crate::indexer::{Definition, SemanticGraph};

use super::structure_search::{
	find_module_structure, list_source_directories,
	ModuleInfo,
};
use super::types_navigation::{
	ModuleStructure, StructureResult,
	SubmoduleInfo, SymbolEntry,
	SymbolListOptions,
};

use super::navigation_impl::require_graph;
use crate::indexer::IndexResult;

/// List symbols with optional file/kind filters
pub(super) fn list_symbols(
	result: &IndexResult,
	opts: &SymbolListOptions,
) -> Result<Vec<SymbolEntry>, SearchError> {
	let graph = require_graph(result)?;
	let defs = resolve_defs(graph, opts);
	let filtered = filter_by_file(defs, opts);

	Ok(filtered
		.into_iter()
		.map(|def| SymbolEntry {
			symbol: def.symbol.clone(),
			fqn: def.fqn.clone(),
		})
		.collect())
}

fn resolve_defs<'g>(
	graph: &'g SemanticGraph,
	opts: &SymbolListOptions,
) -> Vec<&'g Definition> {
	if let Some(ref kind) = opts.kind {
		graph.find_by_kind(*kind)
	} else {
		graph
			.all_symbol_names()
			.into_iter()
			.flat_map(|name| {
				graph.find_definitions(name)
			})
			.collect()
	}
}

fn filter_by_file<'g>(
	defs: Vec<&'g Definition>,
	opts: &SymbolListOptions,
) -> Vec<&'g Definition> {
	let Some(ref file) = opts.file else {
		return defs;
	};
	let file_path = Path::new(file);
	defs.into_iter()
		.filter(|def| {
			def.symbol
				.location
				.file
				.ends_with(file_path)
		})
		.collect()
}

/// Find module structure for a target
pub(super) fn find_structure(
	target: &str,
	path: &Path,
) -> Result<StructureResult, SearchError> {
	let modules =
		find_module_structure(path, target);

	if modules.is_empty() {
		let dirs = list_source_directories(path);
		return Ok(StructureResult {
			target: target.to_string(),
			modules: vec![],
			available_dirs: dirs,
		});
	}

	let converted = convert_modules(modules);
	Ok(StructureResult {
		target: target.to_string(),
		modules: converted,
		available_dirs: vec![],
	})
}

fn convert_modules(
	modules: Vec<ModuleInfo>,
) -> Vec<ModuleStructure> {
	modules
		.into_iter()
		.map(|mod_info| ModuleStructure {
			path: mod_info.path,
			submodules: mod_info
				.submodules
				.into_iter()
				.map(|sub| SubmoduleInfo {
					name: sub.name,
					is_public: sub.is_public,
					doc: sub.doc,
				})
				.collect(),
			reexport_count: mod_info
				.reexports
				.len(),
		})
		.collect()
}
