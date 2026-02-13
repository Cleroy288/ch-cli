//! Embed command implementation.
//!
//! Thin handler that uses IndexService for indexing
//! and DaemonClient for embedding (no embed service
//! yet).

use std::io::Write;
use std::path::Path;
use std::time::Instant;

use crate::retrieval::daemon::DaemonClient;
use crate::service::index::types::IndexOptions;
use crate::service::{
	DefaultIndexService, IndexService,
};

use super::error::CommandResult;

/// Execute the `embed` command
pub fn embed_command(
	path: &str,
	force: bool,
) -> CommandResult {
	let start = Instant::now();
	if force {
		remove_existing_index(path)?;
	}
	let result = index_for_embed(path)?;
	let total = embed_symbols(&result.symbols)?;
	print_embed_summary(total, start)?;
	Ok(())
}

/// Run indexing step for embed command
fn index_for_embed(
	path: &str,
) -> Result<
	crate::indexer::IndexResult,
	super::error::CommandError,
> {
	let mut out = std::io::stdout().lock();
	writeln!(
		out, "Indexing project at: {}", path
	)?;
	let svc = DefaultIndexService::new();
	let opts = IndexOptions {
		flags: crate::service::index::types
			::IndexFlags {
			semantic: true,
			persistence: false,
			verbose: false,
		},
	};
	let result =
		svc.index_project(Path::new(path), &opts)?;
	writeln!(
		out,
		"Found {} symbols",
		result.symbols.len(),
	)?;
	Ok(result)
}

/// Connect to daemon and embed symbols
fn embed_symbols(
	symbols: &[crate::indexer::Symbol],
) -> Result<usize, std::io::Error> {
	let mut out = std::io::stdout().lock();
	let client = DaemonClient::new();
	writeln!(out, "Connecting to daemon...")?;
	ensure_daemon_ready(&client)?;
	let texts = build_embed_texts(symbols);
	Ok(run_embed_batches(&client, &texts))
}

/// Print embed completion summary
fn print_embed_summary(
	total: usize,
	start: Instant,
) -> std::io::Result<()> {
	let mut out = std::io::stdout().lock();
	writeln!(
		out,
		"\nEmbedding complete: {} symbols \
		in {} ms",
		total,
		start.elapsed().as_millis(),
	)?;
	Ok(())
}

/// Remove existing index directory for force mode
fn remove_existing_index(
	path: &str,
) -> Result<(), std::io::Error> {
	let index_dir =
		std::path::Path::new(path).join(".rustean-index");
	if index_dir.exists() {
		writeln!(
			std::io::stdout(),
			"Force mode: removing index at {:?}",
			index_dir,
		)?;
		std::fs::remove_dir_all(&index_dir)?;
	}
	Ok(())
}

/// Ping daemon and wait if models are loading
fn ensure_daemon_ready(
	client: &DaemonClient,
) -> Result<(), std::io::Error> {
	let mut out = std::io::stdout().lock();
	match client.ping() {
		Ok(true) => {
			writeln!(out, "Daemon is ready")?;
			Ok(())
		}
		Ok(false) => {
			writeln!(
				out,
				"Daemon started, \
				waiting for models..."
			)?;
			std::thread::sleep(
				std::time::Duration::from_secs(5),
			);
			Ok(())
		}
		Err(ping_err) => {
			print_daemon_error(&ping_err)
		}
	}
}

/// Print daemon connection error and return Err
fn print_daemon_error(
	ping_err: &impl std::fmt::Display,
) -> Result<(), std::io::Error> {
	let mut err_out = std::io::stderr().lock();
	writeln!(
		err_out,
		"Error: Could not connect: {}",
		ping_err,
	)?;
	writeln!(
		err_out,
		"Run 'rustean daemon start' first"
	)?;
	Err(std::io::Error::other(
		ping_err.to_string(),
	))
}

/// Build text representations for embedding
fn build_embed_texts(
	symbols: &[crate::indexer::Symbol],
) -> Vec<String> {
	symbols
		.iter()
		.map(|sym| {
			format!(
				"{} {} {}",
				sym.kind,
				sym.name,
				sym.signature
					.as_deref()
					.unwrap_or(""),
			)
		})
		.collect()
}

/// Send embedding requests in batches of 32
fn run_embed_batches(
	client: &DaemonClient,
	texts: &[String],
) -> usize {
	let batch_size = 32; // max per daemon request
	let mut total = 0;
	let mut out = std::io::stdout().lock();

	for (idx, batch) in
		texts.chunks(batch_size).enumerate()
	{
		total += run_single_batch(
			&mut out, client, batch,
		);
		print_batch_progress(
			&mut out, idx, total, texts.len(),
		);
	}
	total
}

/// Run a single batch, return count embedded
fn run_single_batch(
	out: &mut impl Write,
	client: &DaemonClient,
	batch: &[String],
) -> usize {
	match client.embed(batch.to_vec()) {
		Ok(embeddings) => embeddings.len(),
		Err(batch_err) => {
			writeln!(
				out,
				"  Batch error: {}", batch_err,
			)
			.ok();
			0
		}
	}
}

/// Print batch progress line
fn print_batch_progress(
	out: &mut impl Write,
	idx: usize,
	total: usize,
	count: usize,
) {
	writeln!(
		out,
		"  Batch {}: {}/{}",
		idx + 1, total, count,
	)
	.ok();
}
