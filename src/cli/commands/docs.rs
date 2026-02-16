//! Documentation generation commands.
//!
//! Thin handlers that delegate to DocGenService
//! for all doc operations.

use std::io::Write;
use std::path::Path;

use crate::service::{
	DefaultDocGenService, DocGenService,
};

use super::docs_display::{
	print_doc_detail, print_search_result,
};
use super::error::CommandResult;

/// Execute the `docs generate` command
pub fn docs_generate_command(
	force: bool,
) -> CommandResult {
	let svc = DefaultDocGenService::new();
	let path = Path::new(".");
	let mut out = std::io::stdout().lock();

	writeln!(
		out,
		"Starting documentation generation..."
	)?;
	let result = svc.start_generation(path, force);
	print_generation_result(&mut out, result)?;
	Ok(())
}

/// Print generation start result
fn print_generation_result(
	out: &mut impl Write,
	result: Result<(), impl std::fmt::Display>,
) -> std::io::Result<()> {
	match result {
		Ok(_) => {
			writeln!(
				out,
				"Doc generation started \
				in background."
			)?;
			writeln!(
				out,
				"Use 'rustean docs status' \
				to check."
			)?;
		}
		Err(gen_err) => {
			writeln!(out, "Error: {}", gen_err)?;
			writeln!(
				out,
				"Make sure daemon is running: \
				rustean daemon start"
			)?;
		}
	}
	Ok(())
}

/// Execute the `docs status` command
pub fn docs_status_command() -> CommandResult {
	let svc = DefaultDocGenService::new();
	let path = Path::new(".");
	let mut out = std::io::stdout().lock();

	match svc.get_status(path) {
		Ok(status) => {
			print_status_counts(&mut out, &status)?;
			print_status_progress(&status)?;
		}
		Err(status_err) => {
			writeln!(
				out, "No documentation found."
			)?;
			writeln!(
				out, "(Error: {})", status_err
			)?;
		}
	}
	Ok(())
}

/// Print doc status counts
fn print_status_counts(
	out: &mut impl Write,
	status: &crate::service::docgen::types
		::DocStatusInfo,
) -> std::io::Result<()> {
	writeln!(out, "Documentation Status:\n")?;
	writeln!(
		out, "  Total:     {}", status.total
	)?;
	writeln!(
		out, "  Completed: {}", status.completed
	)?;
	writeln!(
		out, "  Pending:   {}", status.pending
	)?;
	Ok(())
}

/// Execute the `docs show` command
pub fn docs_show_command(
	symbol: &str,
) -> CommandResult {
	let svc = DefaultDocGenService::new();
	let path = Path::new(".");
	let mut out = std::io::stdout().lock();

	match svc.get_doc(path, symbol) {
		Ok(Some(entry)) => {
			print_doc_detail(&mut out, &entry)?;
		}
		Ok(None) => {
			writeln!(
				out,
				"No documentation found for '{}'",
				symbol,
			)?;
		}
		Err(doc_err) => {
			writeln!(out, "Error: {}", doc_err)?;
		}
	}
	Ok(())
}

/// Execute the `docs search` command
pub fn docs_search_command(
	query: &str,
	limit: usize,
) -> CommandResult {
	let svc = DefaultDocGenService::new();
	let path = Path::new(".");
	let mut out = std::io::stdout().lock();

	let result =
		svc.search_docs(path, query, limit);
	print_search_results(&mut out, query, result)?;
	Ok(())
}

/// Print docs search results or error
fn print_search_results(
	out: &mut impl Write,
	query: &str,
	result: Result<
		Vec<crate::retrieval::docgen::DocEntry>,
		impl std::fmt::Display,
	>,
) -> std::io::Result<()> {
	match result {
		Ok(ref results) if results.is_empty() => {
			writeln!(
				out,
				"No docs found for '{}'", query,
			)?;
		}
		Ok(results) => {
			print_doc_hits(out, query, &results)?;
		}
		Err(search_err) => {
			writeln!(
				out,
				"Search failed: {}", search_err,
			)?;
		}
	}
	Ok(())
}

/// Print document search hits
fn print_doc_hits(
	out: &mut impl Write,
	query: &str,
	results: &[crate::retrieval::docgen::DocEntry],
) -> std::io::Result<()> {
	writeln!(
		out, "Doc results for '{}':\n", query,
	)?;
	for (idx, doc) in results.iter().enumerate() {
		print_search_result(out, idx, doc)?;
	}
	Ok(())
}

/// Print status progress bar and state
fn print_status_progress(
	status: &crate::service::docgen::types
		::DocStatusInfo,
) -> std::io::Result<()> {
	let mut out = std::io::stdout().lock();
	print_progress_bar(&mut out, status)?;
	print_status_label(&mut out, status)?;
	Ok(())
}

/// Print the progress bar if total > 0
fn print_progress_bar(
	out: &mut impl Write,
	status: &crate::service::docgen::types
		::DocStatusInfo,
) -> std::io::Result<()> {
	if status.total == 0 {
		return Ok(());
	}
	let pct = (status.completed as f64
		/ status.total as f64)
		* 100.0;
	let filled = ((pct / 5.0) as usize).min(20);
	let empty = 20_usize.saturating_sub(filled);
	writeln!(
		out,
		"\n  Progress: [{}{}] {:.1}%",
		"#".repeat(filled),
		".".repeat(empty),
		pct,
	)?;
	Ok(())
}

/// Print the generation state label
fn print_status_label(
	out: &mut impl Write,
	status: &crate::service::docgen::types
		::DocStatusInfo,
) -> std::io::Result<()> {
	if status.is_ready {
		writeln!(
			out, "\n  Documentation is ready!"
		)?;
	} else if status.is_generating {
		writeln!(
			out, "\n  Generation running..."
		)?;
	} else {
		writeln!(
			out,
			"\n  Run 'rustean docs generate'."
		)?;
	}
	Ok(())
}
