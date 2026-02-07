//! Documentation generation commands.
//!
//! Thin handlers that delegate to DocGenService
//! for all doc operations.

use std::path::Path;

use crate::retrieval::docgen::DocEntry;
use crate::service::{
	DefaultDocGenService, DocGenService,
};

use super::error::CommandResult;

/// Execute the `docs generate` command
pub fn docs_generate_command(
	force: bool,
) -> CommandResult {
	let svc = DefaultDocGenService::new();
	let path = Path::new(".");

	println!("Starting documentation generation...");
	match svc.start_generation(path, force) {
		Ok(_) => {
			println!(
				"Doc generation started in background."
			);
			println!(
				"Use 'ch-cli docs status' to check."
			);
		}
		Err(e) => {
			println!("Error: {}", e);
			println!(
				"Make sure daemon is running: \
				ch-cli daemon start"
			);
		}
	}
	Ok(())
}

/// Execute the `docs status` command
pub fn docs_status_command() -> CommandResult {
	let svc = DefaultDocGenService::new();
	let path = Path::new(".");

	match svc.get_status(path) {
		Ok(s) => {
			println!("Documentation Status:\n");
			println!("  Total:     {}", s.total);
			println!("  Completed: {}", s.completed);
			println!("  Pending:   {}", s.pending);
			if s.total > 0 {
				let pct = (s.completed as f64
					/ s.total as f64) * 100.0;
				let filled =
					((pct / 5.0) as usize).min(20);
				let empty =
					20_usize.saturating_sub(filled);
				println!(
					"\n  Progress: [{}{}] {:.1}%",
					"#".repeat(filled),
					".".repeat(empty),
					pct,
				);
			}
			if s.is_ready {
				println!("\n  Documentation is ready!");
			} else if s.is_generating {
				println!("\n  Generation running...");
			} else {
				println!(
					"\n  Run 'ch-cli docs generate'."
				);
			}
		}
		Err(e) => {
			println!("No documentation found.");
			println!("(Error: {})", e);
		}
	}
	Ok(())
}

/// Execute the `docs show` command
pub fn docs_show_command(
	symbol: &str,
) -> CommandResult {
	let svc = DefaultDocGenService::new();
	let path = Path::new(".");

	match svc.get_doc(path, symbol) {
		Ok(Some(entry)) => {
			print_doc_detail(&entry);
		}
		Ok(None) => {
			println!(
				"No documentation found for '{}'",
				symbol,
			);
		}
		Err(e) => {
			println!("Error: {}", e);
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

	match svc.search_docs(path, query, limit) {
		Ok(results) if results.is_empty() => {
			println!(
				"No docs found for '{}'", query,
			);
		}
		Ok(results) => {
			println!(
				"Doc results for '{}':\n", query,
			);
			for (i, d) in results.iter().enumerate() {
				println!(
					"  {}. {} {} ({}:{})",
					i + 1, d.kind, d.name,
					d.file_path.display(), d.line,
				);
				if let Some(ref doc) = d.llm_doc {
					let short: String =
						doc.chars().take(100).collect();
					let tail =
						if doc.len() > 100 { "..." }
						else { "" };
					println!(
						"     {}{}",
						short.replace('\n', " "),
						tail,
					);
				}
				println!();
			}
		}
		Err(e) => {
			println!("Search failed: {}", e);
		}
	}
	Ok(())
}

/// Display full documentation for a single entry
fn print_doc_detail(e: &DocEntry) {
	println!("Doc for '{}'\n", e.name);
	println!("  Kind: {}", e.kind);
	println!(
		"  File: {}:{}",
		e.file_path.display(), e.line,
	);
	if let Some(ref s) = e.signature {
		println!("\n  Signature:\n    {}", s);
	}
	if let Some(ref c) = e.user_comment {
		println!("\n  User Comment:");
		for line in c.lines() {
			println!("    {}", line);
		}
	}
	if let Some(ref d) = e.llm_doc {
		println!("\n  Generated Documentation:");
		for line in d.lines() {
			println!("    {}", line);
		}
	}
	if !e.links.depends_on.is_empty() {
		println!("\n  Depends On:");
		for dep in &e.links.depends_on {
			println!("    - {}", dep);
		}
	}
	if !e.links.depended_by.is_empty() {
		println!("\n  Used By:");
		for dep in &e.links.depended_by {
			println!("    - {}", dep);
		}
	}
	if !e.links.external_deps.is_empty() {
		println!("\n  External Crates:");
		for dep in &e.links.external_deps {
			println!("    - {}", dep);
		}
	}
	println!("\n  Status: {}", e.status);
}
