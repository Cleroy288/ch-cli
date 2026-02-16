//! Navigation commands (goto, refs, symbols).
//!
//! Thin handlers: parse CLI args, call service,
//! format output. No direct IndexManager usage.

use std::io::Write;
use std::path::Path;

use crate::service::search::types_navigation::{
	ReferenceResult, SymbolEntry,
	SymbolListOptions,
};
use crate::service::{
	DefaultSearchService, SearchService,
};

use super::error::{CommandError, CommandResult};
use super::search::parse_symbol_kind;

/// Execute the `goto` command - find definition
pub fn goto_command(
	symbol: &str,
) -> CommandResult {
	let service = DefaultSearchService::new();
	let defs = service.find_definition(
		symbol, Path::new("."),
	)?;
	if defs.is_empty() {
		return Err(CommandError::SymbolNotFound(
			symbol.to_string(),
		));
	}
	print_definitions(symbol, &defs)?;
	Ok(())
}

/// Print definition locations
fn print_definitions(
	symbol: &str,
	defs: &[crate::service::search
		::types_navigation::DefinitionHit],
) -> std::io::Result<()> {
	let mut out = std::io::stdout().lock();
	writeln!(
		out,
		"Definition(s) for '{}':\n",
		symbol,
	)?;
	for def in defs {
		writeln!(
			out,
			"  {} {} - {}:{}:{}",
			def.symbol.kind,
			def.symbol.name,
			def.symbol.location.file.display(),
			def.symbol.location.line,
			def.symbol.location.column
		)?;
		if let Some(ref sig) = def.symbol.signature
		{
			writeln!(out, "     {}", sig)?;
		}
	}
	Ok(())
}

/// Execute the `refs` command - find references
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

/// Execute the `symbols` command - list symbols
pub fn symbols_command(
	file: Option<&str>,
	kind: Option<&str>,
) -> CommandResult {
	let service = DefaultSearchService::new();
	let kind_filter = kind
		.map(parse_symbol_kind)
		.transpose()?;
	let opts = SymbolListOptions {
		file: file.map(|str_val| {
			str_val.to_string()
		}),
		kind: kind_filter,
	};
	let symbols = service.list_symbols(
		Path::new("."), &opts,
	)?;
	let mut out = std::io::stdout().lock();
	if symbols.is_empty() {
		writeln!(out, "No symbols found")?;
		return Ok(());
	}

	let title = format_title(file, kind);
	writeln!(out, "{}\n", title)?;
	for entry in &symbols {
		format_symbol_entry(&mut out, entry);
	}
	Ok(())
}

/// Build a title string from filter options
fn format_title(
	file: Option<&str>,
	kind: Option<&str>,
) -> String {
	match (file, kind) {
		(Some(fpath), Some(knd)) => format!(
			"Symbols in '{}' of kind '{}'",
			fpath, knd,
		),
		(Some(fpath), None) => {
			format!("Symbols in '{}'", fpath)
		}
		(None, Some(knd)) => {
			format!("All '{}' symbols", knd)
		}
		(None, None) => "All symbols".to_string(),
	}
}

/// Format and print reference result
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
	defs: &[crate::service::search
		::types_navigation::UsageLocation],
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
	refs: &[crate::service::search
		::types_navigation::UsageLocation],
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

/// Format a symbol entry for listing
fn format_symbol_entry(
	out: &mut impl Write,
	entry: &SymbolEntry,
) {
	let file = entry
		.symbol
		.location
		.file
		.file_name()
		.and_then(|fname| fname.to_str())
		.unwrap_or("?");
	writeln!(
		out,
		"  {} {} ({}:{})",
		entry.symbol.kind,
		entry.symbol.name,
		file,
		entry.symbol.location.line
	)
	.ok();
}
