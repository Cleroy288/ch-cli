//! Display helpers for documentation commands.

use std::io::Write;

use crate::retrieval::docgen::DocEntry;

use super::docs_display_helpers::{
	print_doc_links, print_doc_signature,
};

/// Print a single search result entry
pub(super) fn print_search_result(
	out: &mut impl Write,
	idx: usize,
	doc: &DocEntry,
) -> std::io::Result<()> {
	writeln!(
		out,
		"  {}. {} {} ({}:{})",
		idx + 1,
		doc.kind,
		doc.name,
		doc.file_path.display(),
		doc.line,
	)?;
	print_doc_preview(out, &doc.llm_doc)?;
	writeln!(out)?;
	Ok(())
}

/// Print a short preview of a doc string
fn print_doc_preview(
	out: &mut impl Write,
	doc: &Option<String>,
) -> std::io::Result<()> {
	let Some(ref text) = doc else {
		return Ok(());
	};
	let short: String =
		text.chars().take(100).collect();
	let tail =
		if text.len() > 100 { "..." } else { "" };
	writeln!(
		out,
		"     {}{}",
		short.replace('\n', " "),
		tail,
	)?;
	Ok(())
}

/// Display full documentation for a single entry
pub(super) fn print_doc_detail(
	out: &mut impl Write,
	entry: &DocEntry,
) -> std::io::Result<()> {
	writeln!(out, "Doc for '{}'\n", entry.name)?;
	writeln!(out, "  Kind: {}", entry.kind)?;
	writeln!(
		out,
		"  File: {}:{}",
		entry.file_path.display(),
		entry.line,
	)?;
	print_doc_signature(out, entry)?;
	print_doc_links(out, entry)?;
	writeln!(
		out, "\n  Status: {}", entry.status
	)?;
	Ok(())
}
