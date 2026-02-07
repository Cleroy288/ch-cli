//! Navigation commands (goto, refs, symbols).
//!
//! Thin handlers: parse CLI args, call service,
//! format output. No direct IndexManager usage.

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
pub fn goto_command(symbol: &str) -> CommandResult {
	let service = DefaultSearchService::new();
	let defs = service.find_definition(
		symbol, Path::new("."),
	)?;
	if defs.is_empty() {
		return Err(CommandError::SymbolNotFound(
			symbol.to_string(),
		));
	}

	println!("Definition(s) for '{}':\n", symbol);
	for def in &defs {
		println!(
			"  {} {} - {}:{}:{}",
			def.symbol.kind,
			def.symbol.name,
			def.symbol.location.file.display(),
			def.symbol.location.line,
			def.symbol.location.column
		);
		if let Some(ref sig) = def.symbol.signature
		{
			println!("     {}", sig);
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
		symbol, Path::new("."), include_definition,
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
		file: file.map(|s| s.to_string()),
		kind: kind_filter,
	};
	let symbols = service.list_symbols(
		Path::new("."), &opts,
	)?;
	if symbols.is_empty() {
		println!("No symbols found");
		return Ok(());
	}

	let title = match (file, kind) {
		(Some(f), Some(k)) => format!(
			"Symbols in '{}' of kind '{}'", f, k
		),
		(Some(f), None) => {
			format!("Symbols in '{}'", f)
		}
		(None, Some(k)) => {
			format!("All '{}' symbols", k)
		}
		(None, None) => "All symbols".to_string(),
	};
	println!("{}\n", title);
	for entry in &symbols {
		format_symbol_entry(entry);
	}
	Ok(())
}

/// Format and print reference result
fn format_refs_result(
	symbol: &str,
	result: &ReferenceResult,
) {
	if !result.definitions.is_empty() {
		println!("Definition(s):\n");
		for d in &result.definitions {
			println!(
				"  {} - {}:{}",
				symbol, d.file.display(), d.line
			);
		}
		println!();
	}
	if result.references.is_empty() {
		println!(
			"No references found for '{}'",
			symbol
		);
	} else {
		println!(
			"References ({}):\n",
			result.references.len()
		);
		for r in &result.references {
			println!(
				"  {}:{}",
				r.file.display(), r.line
			);
		}
	}
}

/// Format a symbol entry for listing
fn format_symbol_entry(entry: &SymbolEntry) {
	let file = entry
		.symbol
		.location
		.file
		.file_name()
		.and_then(|f| f.to_str())
		.unwrap_or("?");
	println!(
		"  {} {} ({}:{})",
		entry.symbol.kind,
		entry.symbol.name,
		file,
		entry.symbol.location.line
	);
}
