//! Display helpers for memory commands.

use std::io::Write;

use crate::domain::memory::Interaction;
use crate::domain::memory_helpers;
use crate::indexer::memory::types::{
	MemoryHit, MemoryStats,
};
use crate::ui::strings::cli_messages;

/// Max chars to show in truncated text
const TRUNCATE_LEN: usize = 120;

/// Print a list of interactions
pub fn print_interactions(
	out: &mut impl Write,
	interactions: &[Interaction],
) -> std::io::Result<()> {
	if interactions.is_empty() {
		writeln!(
			out, "{}", cli_messages::MEMORY_NO_RESULTS,
		)?;
		return Ok(());
	}
	for (idx, item) in
		interactions.iter().enumerate()
	{
		print_interaction(out, idx, item)?;
	}
	Ok(())
}

/// Print a single interaction entry
pub fn print_interaction(
	out: &mut impl Write,
	idx: usize,
	interaction: &Interaction,
) -> std::io::Result<()> {
	let rtype = memory_helpers::response_type_label(
		&interaction.response,
	);
	writeln!(
		out,
		"  {}. [{}] {} ({})",
		idx + 1,
		rtype,
		truncate_text(
			&interaction.input.text, TRUNCATE_LEN,
		),
		&interaction.session_id,
	)?;
	Ok(())
}

/// Print search hits with scores
pub fn print_search_hits(
	out: &mut impl Write,
	hits: &[MemoryHit],
) -> std::io::Result<()> {
	if hits.is_empty() {
		writeln!(
			out, "{}", cli_messages::MEMORY_NO_RESULTS,
		)?;
		return Ok(());
	}
	for (idx, hit) in hits.iter().enumerate() {
		writeln!(
			out,
			"  {}. [{:.3}] {}",
			idx + 1,
			hit.score,
			truncate_text(
				&hit.interaction.input.text,
				TRUNCATE_LEN,
			),
		)?;
	}
	Ok(())
}

/// Print memory statistics
pub fn print_stats(
	out: &mut impl Write,
	stats: &MemoryStats,
) -> std::io::Result<()> {
	writeln!(
		out,
		"{}",
		cli_messages::MEMORY_STATS_HEADER,
	)?;
	writeln!(
		out, "  Interactions: {}", stats.total,
	)?;
	writeln!(
		out, "  Sessions:     {}", stats.sessions,
	)?;
	writeln!(
		out, "  Size:         {} bytes",
		stats.size_bytes,
	)?;
	writeln!(
		out, "  Last updated: {}", stats.last_updated,
	)?;
	Ok(())
}

/// Truncate text to max_len, add "..." if needed
pub fn truncate_text(
	text: &str,
	max_len: usize,
) -> String {
	let clean = text.replace('\n', " ");
	if clean.len() <= max_len {
		return clean;
	}
	let truncated: String =
		clean.chars().take(max_len).collect();
	format!("{truncated}...")
}
