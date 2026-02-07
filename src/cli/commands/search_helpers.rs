//! Display helpers for search commands.
//!
//! Pure presentation logic for formatting
//! search results in the CLI.

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
	if result.modules.is_empty() {
		println!(
			"No module structure found for '{}'",
			result.target
		);
		println!(
			"\nAvailable directories under src/:"
		);
		for dir in &result.available_dirs {
			println!("  - {}", dir);
		}
		return;
	}

	println!(
		"Module structure for '{}':\n",
		result.target
	);
	for module in &result.modules {
		format_module(module);
	}
}

/// Format a single module structure
fn format_module(module: &ModuleStructure) {
	println!("{}", module.path.display());
	for sub in &module.submodules {
		let vis = if sub.is_public {
			"pub mod"
		} else {
			"mod"
		};
		if let Some(ref doc) = sub.doc {
			println!(
				"   {} {}; // {}",
				vis, sub.name, doc
			);
		} else {
			println!(
				"   {} {};", vis, sub.name
			);
		}
	}
	if module.reexport_count > 0 {
		println!(
			"   // {} re-exports",
			module.reexport_count
		);
	}
	println!();
}

/// Print full source content for search hits
pub fn print_full_content(
	hits: &[SearchResultHit],
) {
	println!("\n--- Full Content ---\n");
	for hit in hits {
		let path = &hit.symbol.location.file;
		let target = hit.symbol.location.line;
		println!(
			"=== {} (line {}) ===\n",
			path.display(),
			target
		);
		if let Ok(content) =
			std::fs::read_to_string(path)
		{
			print_snippet(&content, target);
		}
		println!();
	}
}

/// Print source snippet around a target line
fn print_snippet(content: &str, target: usize) {
	let lines: Vec<&str> =
		content.lines().collect();
	let start = target.saturating_sub(5);
	let end = (target + 195).min(lines.len());

	for (i, line) in
		lines[start..end].iter().enumerate()
	{
		let n = start + i + 1;
		let m = if n == target { ">" } else { " " };
		println!("{}{:4} | {}", m, n, line);
	}
	if end < lines.len() {
		println!(
			"      ... ({} more lines)",
			lines.len() - end
		);
	}
}
