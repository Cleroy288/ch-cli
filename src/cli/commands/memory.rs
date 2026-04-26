use std::io::Write;
use std::path::Path;

use crate::domain::memory::{Interaction, UserInput};
use crate::domain::memory_helpers;
use crate::service::memory::id_gen;
use crate::service::{
	DefaultMemoryService, MemoryService,
};
use crate::ui::strings::cli_messages;

use super::error::CommandResult;
use super::memory_display;

/// Execute `memory show` command
pub fn memory_show_command(
	limit: usize,
	session: Option<&str>,
) -> CommandResult {
	let svc = DefaultMemoryService::default();
	let root = Path::new(".");
	let mut out = std::io::stdout().lock();
	let mut items =
		svc.show_recent(root, limit)?;
	if let Some(sid) = session {
		items = filter_session(items, sid);
	}
	memory_display::print_interactions(
		&mut out, &items,
	)?;
	Ok(())
}

/// Execute `memory search` command
pub fn memory_search_command(
	query: &str,
	limit: usize,
) -> CommandResult {
	let svc = DefaultMemoryService::default();
	let root = Path::new(".");
	let mut out = std::io::stdout().lock();

	writeln!(
		out,
		"{} for '{query}':\n",
		cli_messages::MEMORY_SEARCH_HEADER,
	)?;
	let hits = svc.search(root, query, limit)?;
	memory_display::print_search_hits(
		&mut out, &hits,
	)?;
	Ok(())
}

/// Execute `memory stats` command
pub fn memory_stats_command() -> CommandResult {
	let svc = DefaultMemoryService::default();
	let root = Path::new(".");
	let mut out = std::io::stdout().lock();

	let stats = svc.stats(root)?;
	memory_display::print_stats(
		&mut out, &stats,
	)?;
	Ok(())
}

/// Execute `memory add` command
pub fn memory_add_command(
	input_text: &str,
	response_text: &str,
	response_type: &str,
) -> CommandResult {
	let svc = DefaultMemoryService::default();
	let root = Path::new(".");
	let mut out = std::io::stdout().lock();

	let user_input = UserInput {
		text: input_text.to_string(),
		command: None,
		files: Vec::new(),
	};
	let response = memory_helpers::response_from_label(
		response_type,
		response_text.to_string(),
	);
	let session = id_gen::new_session_id();
	let interaction =
		id_gen::new_interaction(
			&session, user_input, response,
		);
	svc.add(root, &interaction)?;
	writeln!(out, "{}", cli_messages::MEMORY_ADDED)?;
	Ok(())
}

pub fn filter_session(
	items: Vec<Interaction>,
	session_id: &str,
) -> Vec<Interaction> {
	items
		.into_iter()
		.filter(|i| i.session_id == session_id)
		.collect()
}
