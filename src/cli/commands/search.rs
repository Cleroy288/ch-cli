//! Search command implementation.
//!
//! Thin handler: parse CLI args, detect query
//! type, delegate to service, format output.

use std::io::Write;
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

#[allow(clippy::struct_excessive_bools)]
/// Boolean flags for the search command
#[derive(Debug, Clone, Default)]
pub struct SearchCommandFlags {
	/// enable fuzzy matching
	pub fuzzy: bool,
	/// enable semantic search
	pub semantic: bool,
	/// enable context expansion
	pub context: bool,
	/// enable reranking
	pub rerank: bool,
	/// show full content
	pub full: bool,
}

/// Options for the search command
#[derive(Default)]
pub struct SearchCommandOptions<'opt> {
	/// max number of results
	pub limit: usize,
	/// filter by symbol kind
	pub kind: Option<&'opt str>,
	/// boolean flags
	pub flags: SearchCommandFlags,
}

/// Execute the `search` command
pub fn search_command(
	query: &str,
	opts: &SearchCommandOptions,
) -> CommandResult {
	if let Some(struct_q) =
		detect_structure_query(query)
	{
		return handle_structure_query(
			&struct_q.target,
		);
	}
	if let Some(caller_q) =
		detect_caller_query(query)
	{
		return route_caller_query(&caller_q);
	}
	run_text_search(query, opts)
}

/// Run a standard text/semantic search
fn run_text_search(
	query: &str,
	opts: &SearchCommandOptions,
) -> CommandResult {
	let service = DefaultSearchService::new();
	let search_opts = build_search_opts(opts)?;
	let result = service.search(
		query, Path::new("."), &search_opts,
	)?;
	display_search_result(
		query, &result, opts.flags.full,
	)
}

/// Build SearchOptions from command options
fn build_search_opts(
	opts: &SearchCommandOptions,
) -> Result<SearchOptions, CommandError> {
	let kind_filter = opts
		.kind
		.map(parse_symbol_kind)
		.transpose()?;
	let flags = &opts.flags;
	Ok(SearchOptions {
		limit: opts.limit,
		kind: kind_filter,
		flags: crate::service::search::types
			::SearchFlags {
			fuzzy: flags.fuzzy,
			semantic: flags.semantic,
			context: flags.context,
			rerank: flags.rerank,
			full: flags.full,
		},
	})
}

/// Display result: context XML or hit list
fn display_search_result(
	query: &str,
	result: &SearchResult,
	full: bool,
) -> CommandResult {
	if let Some(ref xml) = result.context_xml {
		let mut out = std::io::stdout().lock();
		writeln!(
			out,
			"Context-expanded results \
			for '{}':\n",
			query
		)?;
		writeln!(out, "{}", xml)?;
		return Ok(());
	}
	format_search_output(query, result, full);
	Ok(())
}

/// Route caller query to graph-based handler
fn route_caller_query(
	query: &crate::retrieval::query::CallerQuery,
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
	handle_caller_query(query, &graph)
}

/// Format and display search results to stdout
fn format_search_output(
	query: &str,
	result: &SearchResult,
	full: bool,
) {
	let mut out = std::io::stdout().lock();
	if result.hits.is_empty() {
		writeln!(
			out,
			"No results found for '{}'",
			query,
		)
		.ok();
		return;
	}
	writeln!(
		out, "Search results for '{}':\n", query
	)
	.ok();
	for (idx, hit) in
		result.hits.iter().enumerate()
	{
		format_semantic_hit(idx + 1, hit);
	}
	if full {
		print_full_content(&result.hits);
	}
}

/// Parse a symbol kind string into SymbolKind
pub fn parse_symbol_kind(
	kind_str: &str,
) -> Result<SymbolKind, CommandError> {
	let key = kind_str.to_lowercase();
	match key.as_str() {
		"function" | "fn" | "func" =>
			Ok(SymbolKind::Function),
		"struct" => Ok(SymbolKind::Struct),
		"enum" => Ok(SymbolKind::Enum),
		"trait" => Ok(SymbolKind::Trait),
		"impl" => Ok(SymbolKind::Impl),
		"method" => Ok(SymbolKind::Method),
		"const" | "constant" =>
			Ok(SymbolKind::Constant),
		"static" => Ok(SymbolKind::Static),
		"type" | "typealias" =>
			Ok(SymbolKind::TypeAlias),
		"module" | "mod" =>
			Ok(SymbolKind::Module),
		"macro" => Ok(SymbolKind::Macro),
		"field" => Ok(SymbolKind::Field),
		"variant" =>
			Ok(SymbolKind::EnumVariant),
		_ => Err(CommandError::InvalidKind(
			kind_str.to_string(),
		)),
	}
}
