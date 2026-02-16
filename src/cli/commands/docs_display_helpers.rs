//! Low-level display helpers for documentation output.

use std::io::Write;

use crate::retrieval::docgen::DocEntry;

/// Print signature and comment sections
pub(super) fn print_doc_signature(
	out: &mut impl Write,
	entry: &DocEntry,
) -> std::io::Result<()> {
	if let Some(ref sig) = entry.signature {
		writeln!(
			out, "\n  Signature:\n    {}", sig
		)?;
	}
	print_multiline(
		out, "\n  User Comment:",
		&entry.user_comment,
	)?;
	print_multiline(
		out,
		"\n  Generated Documentation:",
		&entry.llm_doc,
	)?;
	Ok(())
}

/// Print an optional multiline section
fn print_multiline(
	out: &mut impl Write,
	header: &str,
	text: &Option<String>,
) -> std::io::Result<()> {
	let Some(ref content) = text else {
		return Ok(());
	};
	writeln!(out, "{}", header)?;
	for line in content.lines() {
		writeln!(out, "    {}", line)?;
	}
	Ok(())
}

/// Print dependency and link sections
pub(super) fn print_doc_links(
	out: &mut impl Write,
	entry: &DocEntry,
) -> std::io::Result<()> {
	let links = &entry.links;
	print_link_list(
		out, "Depends On", &links.depends_on,
	)?;
	print_link_list(
		out, "Used By", &links.depended_by,
	)?;
	print_link_list(
		out, "External Crates",
		&links.external_deps,
	)?;
	Ok(())
}

/// Print a named list of links if non-empty
fn print_link_list(
	out: &mut impl Write,
	label: &str,
	items: &[String],
) -> std::io::Result<()> {
	if items.is_empty() {
		return Ok(());
	}
	writeln!(out, "\n  {}:", label)?;
	for dep in items {
		writeln!(out, "    - {}", dep)?;
	}
	Ok(())
}
