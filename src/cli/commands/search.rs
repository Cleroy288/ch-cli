//! Search command implementation.
//!
//! Thin handler: parse CLI args, detect query
//! type, delegate to service, format output.

use std::path::Path;

use crate::indexer::SymbolKind;
use crate::retrieval::query::{
	detect_caller_query, detect_structure_query,
};
use crate::service::search::types::{
	SearchOptions, SearchResult,
};
use crate::service::{
	DefaultSearchService, SearchService,
};

use super::error::{CommandError, CommandResult};
use super::search_callers::handle_caller_query;
use super::search_helpers::{
	handle_structure_query, print_full_content,
};
use super::search_semantic::format_semantic_hit;

/// Execute the `search` command
pub fn search_command(
	query: &str,
	limit: usize,
	fuzzy: bool,
	kind: Option<&str>,
	semantic: bool,
	context: bool,
	rerank: bool,
	full: bool,
) -> CommandResult {
	if let Some(sq) =
		detect_structure_query(query)
	{
		return handle_structure_query(&sq.target);
	}
	if let Some(cq) =
		detect_caller_query(query)
	{
		return route_caller_query(&cq);
	}

	let service = DefaultSearchService::new();
	let kind_filter = kind
		.map(parse_symbol_kind)
		.transpose()?;
	let opts = SearchOptions {
		limit,
		fuzzy,
		kind: kind_filter,
		semantic,
		context,
		rerank,
		full,
	};
	let result = service.search(
		query, Path::new("."), &opts,
	)?;

	if let Some(ref xml) = result.context_xml {
		println!(
			"Context-expanded results for '{}':\n",
			query
		);
		println!("{}", xml);
		return Ok(());
	}
	format_search_output(query, &result, full);
	Ok(())
}

/// Route caller query to graph-based handler
fn route_caller_query(
	cq: &crate::retrieval::query::CallerQuery,
) -> CommandResult {
	use crate::indexer::IndexManager;
	let manager =
		IndexManager::new().with_semantic_analysis();
	let result = manager.index_project(".")?;
	let graph = result
		.semantic_graph
		.ok_or_else(|| {
			CommandError::IndexError(
				"Semantic graph not available"
					.into(),
			)
		})?;
	handle_caller_query(cq, &graph)
}

/// Format and display search results to stdout
fn format_search_output(
	query: &str,
	result: &SearchResult,
	full: bool,
) {
	if result.hits.is_empty() {
		println!(
			"No results found for '{}'", query
		);
		return;
	}
	println!("Search results for '{}':\n", query);
	for (i, hit) in result.hits.iter().enumerate() {
		format_semantic_hit(i + 1, hit);
	}
	if full {
		print_full_content(&result.hits);
	}
}

/// Parse a symbol kind string into SymbolKind
pub fn parse_symbol_kind(
	kind_str: &str,
) -> Result<SymbolKind, CommandError> {
	match kind_str.to_lowercase().as_str() {
		"function" | "fn" | "func" => {
			Ok(SymbolKind::Function)
		}
		"struct" => Ok(SymbolKind::Struct),
		"enum" => Ok(SymbolKind::Enum),
		"trait" => Ok(SymbolKind::Trait),
		"impl" => Ok(SymbolKind::Impl),
		"method" => Ok(SymbolKind::Method),
		"const" | "constant" => {
			Ok(SymbolKind::Constant)
		}
		"static" => Ok(SymbolKind::Static),
		"type" | "typealias" => {
			Ok(SymbolKind::TypeAlias)
		}
		"module" | "mod" => {
			Ok(SymbolKind::Module)
		}
		"macro" => Ok(SymbolKind::Macro),
		"field" => Ok(SymbolKind::Field),
		"variant" => {
			Ok(SymbolKind::EnumVariant)
		}
		_ => Err(CommandError::InvalidKind(
			kind_str.to_string(),
		)),
	}
}

