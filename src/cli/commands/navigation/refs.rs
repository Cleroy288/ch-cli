use std::io::Write;
use std::path::Path;

use crate::service::search::types_navigation::{
	ReferenceResult, UsageLocation,
};
use crate::service::{
	DefaultSearchService, SearchService,
};

use crate::cli::commands::error::{
	CommandError, CommandResult,
};

pub fn refs_command(
	symbol: &str,
	include_definition: bool,
) -> CommandResult {
	let service = DefaultSearchService::new();
	let result = service.find_references(
		symbol,
		Path::new("."),
		include_definition,
	)?;
	let total = result.definitions.len()
		+ result.references.len();
	if total == 0 {
		return Err(CommandError::SymbolNotFound(
			symbol.to_string(),
		));
	}
	format_refs_result(symbol, &result);
	Ok(())
}

fn format_refs_result(
	symbol: &str,
	result: &ReferenceResult,
) {
	let mut out = std::io::stdout().lock();
	print_ref_definitions(
		&mut out, symbol, &result.definitions,
	);
	print_ref_locations(
		&mut out, symbol, &result.references,
	);
}

/// Print definition locations for refs command
fn print_ref_definitions(
	out: &mut impl Write,
	symbol: &str,
	defs: &[UsageLocation],
) {
	if defs.is_empty() {
		return;
	}
	writeln!(out, "Definition(s):\n").ok();
	for def in defs {
		writeln!(
			out,
			"  {} - {}:{}",
			symbol,
			def.file.display(),
			def.line
		)
		.ok();
	}
	writeln!(out).ok();
}

/// Print reference locations for refs command
fn print_ref_locations(
	out: &mut impl Write,
	symbol: &str,
	refs: &[UsageLocation],
) {
	if refs.is_empty() {
		writeln!(
			out,
			"No references found for '{}'",
			symbol
		)
		.ok();
		return;
	}
	writeln!(
		out,
		"References ({}):\n",
		refs.len()
	)
	.ok();
	for ref_loc in refs {
		writeln!(
			out,
			"  {}:{}",
			ref_loc.file.display(),
			ref_loc.line
		)
		.ok();
	}
}
