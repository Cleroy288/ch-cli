//! Navigation service logic (definition, refs, symbols).

use std::path::Path;

use crate::domain::errors::search::SearchError;
use crate::indexer::IndexManager;
use crate::retrieval::query::{
	find_module_structure, list_source_directories,
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
	path: &Path,
) -> Result<Vec<DefinitionHit>, SearchError> {
	let graph = build_semantic_graph(path)?;
	let defs = graph.find_definitions(symbol);

	Ok(defs
		.into_iter()
		.map(|d| DefinitionHit {
			symbol: d.symbol.clone(),
			fqn: d.fqn.clone(),
		})
		.collect())
}

/// Find all references to a symbol
pub(super) fn find_references(
	symbol: &str,
	path: &Path,
	include_def: bool,
) -> Result<ReferenceResult, SearchError> {
	let graph = build_semantic_graph(path)?;
	let usages = graph.find_all_usages(symbol);

	let definitions = if include_def {
		usages
			.definitions
			.into_iter()
			.map(|loc| UsageLocation {
				file: loc.file,
				line: loc.line,
			})
			.collect()
	} else {
		vec![]
	};

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

/// List symbols with optional file/kind filters
pub(super) fn list_symbols(
	path: &Path,
	opts: &SymbolListOptions,
) -> Result<Vec<SymbolEntry>, SearchError> {
	let graph = build_semantic_graph(path)?;

	let defs = if let Some(ref kind) = opts.kind {
		graph.find_by_kind(*kind)
	} else {
		graph
			.all_symbol_names()
			.into_iter()
			.flat_map(|n| graph.find_definitions(n))
			.collect()
	};

	let filtered: Vec<_> =
		if let Some(ref file) = opts.file {
			let fp = Path::new(file);
			defs.into_iter()
				.filter(|d| {
					d.symbol
						.location
						.file
						.ends_with(fp)
				})
				.collect()
		} else {
			defs
		};

	Ok(filtered
		.into_iter()
		.map(|d| SymbolEntry {
			symbol: d.symbol.clone(),
			fqn: d.fqn.clone(),
		})
		.collect())
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

	let converted: Vec<_> = modules
		.into_iter()
		.map(|m| ModuleStructure {
			path: m.path,
			submodules: m
				.submodules
				.into_iter()
				.map(|s| SubmoduleInfo {
					name: s.name,
					is_public: s.is_public,
					doc: s.doc,
				})
				.collect(),
			reexport_count: m.reexports.len(),
		})
		.collect();

	Ok(StructureResult {
		target: target.to_string(),
		modules: converted,
		available_dirs: vec![],
	})
}

/// Build semantic graph from project index
fn build_semantic_graph(
	path: &Path,
) -> Result<
	crate::indexer::SemanticGraph,
	SearchError,
> {
	let manager = IndexManager::new()
		.with_semantic_analysis();
	let result =
		manager.index_project(path).map_err(|e| {
			SearchError::Io(std::io::Error::new(
				std::io::ErrorKind::Other,
				e.to_string(),
			))
		})?;
	result.semantic_graph.ok_or_else(|| {
		SearchError::Io(std::io::Error::new(
			std::io::ErrorKind::Other,
			"Semantic graph not available",
		))
	})
}
