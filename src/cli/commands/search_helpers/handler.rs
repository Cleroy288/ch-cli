use std::io::Write;
use std::path::Path;

use crate::service::search::types_navigation::{
	ModuleStructure, StructureResult,
};
use crate::service::{
	DefaultSearchService, SearchService,
};

use super::super::error::CommandResult;

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
