//! Navigation service logic (definition, refs, symbols).

use std::path::Path;

use crate::domain::errors::search::SearchError;
use crate::indexer::{
	AllUsages, Definition, IndexResult,
	SemanticGraph,
};
use crate::retrieval::query::{
	find_module_structure, list_source_directories,
	ModuleInfo,
};

use super::types_navigation::{
	DefinitionHit, ModuleStructure,
	ReferenceResult, StructureResult,
	SubmoduleInfo, SymbolEntry,
	SymbolListOptions, UsageLocation,
};

/// Find symbol definitions by name
pub(super) fn find_definition(
	symbol: &str,
	result: &IndexResult,
) -> Result<Vec<DefinitionHit>, SearchError> {
	let graph = require_graph(result)?;
	let defs = graph.find_definitions(symbol);

	Ok(defs
		.into_iter()
		.map(|def| DefinitionHit {
			symbol: def.symbol.clone(),
			fqn: def.fqn.clone(),
		})
		.collect())
}

/// Find all references to a symbol
pub(super) fn find_references(
	symbol: &str,
	result: &IndexResult,
	include_def: bool,
) -> Result<ReferenceResult, SearchError> {
	let graph = require_graph(result)?;
	let usages = graph.find_all_usages(symbol);

	let definitions =
		collect_definitions(&usages, include_def);
	let references = usages
		.references
		.into_iter()
		.map(|loc| UsageLocation {
			file: loc.file,
			line: loc.line,
		})
		.collect();

	Ok(ReferenceResult {
		definitions,
		references,
	})
}

/// Collect definition locations if requested
fn collect_definitions(
	usages: &AllUsages,
	include_def: bool,
) -> Vec<UsageLocation> {
	if !include_def {
		return vec![];
	}
	usages
		.definitions
		.iter()
		.map(|loc| UsageLocation {
			file: loc.file.clone(),
			line: loc.line,
		})
		.collect()
}

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

/// Resolve definitions by kind or all symbols
fn resolve_defs<'graph>(
	graph: &'graph SemanticGraph,
	opts: &SymbolListOptions,
) -> Vec<&'graph Definition> {
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

/// Filter definitions by file path if specified
fn filter_by_file<'graph>(
	defs: Vec<&'graph Definition>,
	opts: &SymbolListOptions,
) -> Vec<&'graph Definition> {
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
		return Ok(empty_structure(target, path));
	}

	let converted = convert_modules(modules);
	Ok(StructureResult {
		target: target.to_string(),
		modules: converted,
		available_dirs: vec![],
	})
}

/// Build empty structure with available dirs
fn empty_structure(
	target: &str,
	path: &Path,
) -> StructureResult {
	let dirs = list_source_directories(path);
	StructureResult {
		target: target.to_string(),
		modules: vec![],
		available_dirs: dirs,
	}
}

/// Convert raw modules to ModuleStructure DTOs
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

/// Extract semantic graph or return error
fn require_graph(
	result: &IndexResult,
) -> Result<&SemanticGraph, SearchError> {
	result.semantic_graph.as_deref().ok_or_else(|| {
		SearchError::IoError(std::io::Error::other(
			"Semantic graph not available",
		))
	})
}
