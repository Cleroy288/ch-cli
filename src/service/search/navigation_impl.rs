use crate::domain::errors::search::SearchError;
use crate::indexer::{
	AllUsages, IndexResult, SemanticGraph,
};

use super::cache_helpers::io_err;

use super::types_navigation::{
	DefinitionHit, ReferenceResult,
	UsageLocation,
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

/// Extract semantic graph or return error
pub(super) fn require_graph(
	result: &IndexResult,
) -> Result<&SemanticGraph, SearchError> {
	result
		.semantic_graph
		.as_deref()
		.ok_or_else(|| {
			io_err("Semantic graph not available")
		})
}
