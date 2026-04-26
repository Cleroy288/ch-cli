use std::io::Write;
use std::path::Path;

use crate::service::search::types_navigation::{
	SymbolEntry, SymbolListOptions,
};
use crate::service::{
	DefaultSearchService, SearchService,
};

use super::super::error::{
	CommandError, CommandResult,
};
use super::super::search::parse_symbol_kind;

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

pub fn symbols_command(
	file: Option<&str>,
	kind: Option<&str>,
) -> CommandResult {
	let service = DefaultSearchService::new();
	let kind_filter = kind
		.map(parse_symbol_kind)
		.transpose()?;
	let opts = SymbolListOptions {
		file: file.map(str::to_string),
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
