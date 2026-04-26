use std::path::Path;
use std::sync::Arc;

use crate::indexer::semantic::SemanticGraph;
use crate::service::{
	DefaultSearchService, SearchService,
};

use crate::cli::commands::error::{
	CommandError, CommandResult,
};
use super::info_display;
use super::types::InfoDisplayOpts;

pub fn info_command(
	symbol: &str,
	opts: &InfoDisplayOpts,
) -> CommandResult {
	let service = DefaultSearchService::new();
	let defs = service.find_definition(
		symbol, Path::new("."),
	)?;
	let mut out = std::io::stdout().lock();
	if defs.is_empty() {
		use std::io::Write;
		writeln!(
			out, "Symbol '{}' not found.", symbol
		)?;
		return Ok(());
	}

	let sec = &opts.sections;
	let need_graph =
		sec.callers || sec.callees || sec.refs;
	let graph = if need_graph {
		build_graph()?
	} else {
		None
	};

	info_display::display_all_defs(
		&defs, &graph, opts,
	)
}

type GraphResult =
	Result<Option<Arc<SemanticGraph>>, CommandError>;

fn build_graph() -> GraphResult {
	use crate::indexer::IndexManager;
	let manager = IndexManager::new()
		.with_semantic_analysis();
	let result =
		manager.index_project(Path::new("."))?;
	Ok(result.semantic_graph)
}
