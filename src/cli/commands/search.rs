//! Search command implementation.
//!
//! Thin handler: parse CLI args, detect query
//! type, delegate to service, format output.

use std::path::Path;

use crate::indexer::SymbolKind;
use crate::service::search::types::{
	SearchFlags, SearchOptions,
};
use crate::service::{
	DefaultSearchService, SearchService,
};

use super::error::{CommandError, CommandResult};
use super::search_callers::handle_caller_query;
use super::search_format::format_search_output;

/// Boolean flags for the search command
#[derive(Debug, Clone, Default)]
pub struct SearchCommandFlags {
	/// enable fuzzy matching
	pub fuzzy: bool,
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
	super::search_strategy::dispatch(query, opts)
}

/// Run a standard text/semantic search
pub(crate) fn run_text_search(
	query: &str,
	opts: &SearchCommandOptions,
) -> CommandResult {
	let service = DefaultSearchService::new();
	let search_opts = build_search_opts(opts)?;
	let result = service.search(
		query, Path::new("."), &search_opts,
	)?;
	format_search_output(
		query, &result, opts.flags.full,
	);
	Ok(())
}

/// Build SearchOptions from command options
fn build_search_opts(
	opts: &SearchCommandOptions,
) -> Result<SearchOptions, CommandError> {
	let kind = opts
		.kind
		.map(parse_symbol_kind)
		.transpose()?;
	Ok(SearchOptions {
		limit: opts.limit,
		kind,
		flags: SearchFlags {
			fuzzy: opts.flags.fuzzy,
			full: opts.flags.full,
		},
	})
}

/// Route caller query to graph-based handler
pub(crate) fn route_caller_query(
	query: &crate::service::search::caller
		::CallerQuery,
) -> CommandResult {
	use crate::indexer::IndexManager;
	let result = IndexManager::new()
		.with_semantic_analysis()
		.index_project(".")?;
	let graph =
		result.semantic_graph.ok_or_else(|| {
			CommandError::IndexUnavailable(
				"Semantic graph unavailable"
					.into(),
			)
		})?;
	handle_caller_query(query, &graph)
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
