//! Display helpers for search commands.
//!
//! Pure presentation logic for formatting
//! search results in the CLI.

use std::io::Write;
use std::path::Path;

use crate::service::search::types::SearchResultHit;
use crate::service::search::types_navigation::{
	ModuleStructure, StructureResult,
};
use crate::service::{
	DefaultSearchService, SearchService,
};

use super::error::CommandResult;

/// Handle a module structure query via service
pub fn handle_structure_query(
	target: &str,
) -> CommandResult {
	let service = DefaultSearchService::new();
	let result = service.find_structure(
		target,
		Path::new("."),
	)?;
	format_structure_result(&result);
	Ok(())
}

/// Format structure result for CLI output
fn format_structure_result(
	result: &StructureResult,
) {
	let mut out = std::io::stdout().lock();
	if result.modules.is_empty() {
		print_no_structure(
			&mut out, result,
		);
		return;
	}
	writeln!(
		out,
		"Module structure for '{}':\n",
		result.target
	)
	.ok();
	for module in &result.modules {
		format_module(&mut out, module);
	}
}

/// Print when no module structure is found
fn print_no_structure(
	out: &mut impl Write,
	result: &StructureResult,
) {
	writeln!(
		out,
		"No module structure found for '{}'",
		result.target
	)
	.ok();
	writeln!(
		out,
		"\nAvailable directories under src/:"
	)
	.ok();
	for dir in &result.available_dirs {
		writeln!(out, "  - {}", dir).ok();
	}
}

/// Format a single module structure
fn format_module(
	out: &mut impl Write,
	module: &ModuleStructure,
) {
	writeln!(out, "{}", module.path.display())
		.ok();
	for sub in &module.submodules {
		format_submodule(out, sub);
	}
	if module.reexport_count > 0 {
		writeln!(
			out,
			"   // {} re-exports",
			module.reexport_count
		)
		.ok();
	}
	writeln!(out).ok();
}

/// Format a single submodule entry
fn format_submodule(
	out: &mut impl Write,
	sub: &crate::service::search
		::types_navigation::SubmoduleInfo,
) {
	let vis = if sub.is_public {
		"pub mod"
	} else {
		"mod"
	};
	let doc_suffix = sub
		.doc
		.as_ref()
		.map(|doc| format!(" // {}", doc))
		.unwrap_or_default();
	writeln!(
		out, "   {} {};{}", vis, sub.name,
		doc_suffix,
	)
	.ok();
}

/// Print full source content for search hits
pub fn print_full_content(
	hits: &[SearchResultHit],
) {
	let mut out = std::io::stdout().lock();
	writeln!(out, "\n--- Full Content ---\n").ok();
	for hit in hits {
		let path = &hit.symbol.location.file;
		let target = hit.symbol.location.line;
		writeln!(
			out,
			"=== {} (line {}) ===\n",
			path.display(),
			target
		)
		.ok();
		if let Ok(content) =
			std::fs::read_to_string(path)
		{
			print_snippet(
				&mut out, &content, target,
			);
		}
		writeln!(out).ok();
	}
}

/// Print source snippet around a target line
fn print_snippet(
	out: &mut impl Write,
	content: &str,
	target: usize,
) {
	let lines: Vec<&str> =
		content.lines().collect();
	let start = target.saturating_sub(5);
	let end = (target + 195).min(lines.len());

	print_snippet_lines(
		out, &lines[start..end], start, target,
	);
	if end < lines.len() {
		writeln!(
			out,
			"      ... ({} more lines)",
			lines.len() - end
		)
		.ok();
	}
}

/// Print snippet lines with markers
fn print_snippet_lines(
	out: &mut impl Write,
	lines: &[&str],
	start: usize,
	target: usize,
) {
	for (idx, line) in lines.iter().enumerate() {
		let line_num = start + idx + 1;
		let marker = if line_num == target {
			">"
		} else {
			" "
		};
		writeln!(
			out,
			"{}{:4} | {}",
			marker, line_num, line
		)
		.ok();
	}
}
