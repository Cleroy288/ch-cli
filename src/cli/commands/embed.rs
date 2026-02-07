//! Embed command implementation.
//!
//! Thin handler that uses IndexService for indexing
//! and DaemonClient for embedding (no embed service
//! yet).

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

	println!("Indexing project at: {}", path);

	// index via service
	let svc = DefaultIndexService::new();
	let opts = IndexOptions {
		semantic: true,
		persistence: false,
		verbose: false,
	};
	let result =
		svc.index_project(Path::new(path), &opts)?;
	println!("Found {} symbols", result.symbols.len());

	// connect to daemon for embedding
	let client = DaemonClient::new();
	println!("Connecting to daemon...");
	ensure_daemon_ready(&client)?;

	// generate embeddings in batches
	let texts = build_embed_texts(&result.symbols);
	let total = run_embed_batches(&client, &texts);

	println!(
		"\nEmbedding complete: {} symbols in {} ms",
		total, start.elapsed().as_millis(),
	);
	Ok(())
}

/// Remove existing index directory for force mode
fn remove_existing_index(
	path: &str,
) -> Result<(), std::io::Error> {
	let index_dir =
		std::path::Path::new(path).join(".ch-index");
	if index_dir.exists() {
		println!(
			"Force mode: removing index at {:?}",
			index_dir,
		);
		std::fs::remove_dir_all(&index_dir)?;
	}
	Ok(())
}

/// Ping daemon and wait if models are loading
fn ensure_daemon_ready(
	client: &DaemonClient,
) -> Result<(), std::io::Error> {
	match client.ping() {
		Ok(true) => {
			println!("Daemon is ready");
			Ok(())
		}
		Ok(false) => {
			println!(
				"Daemon started, waiting for models..."
			);
			std::thread::sleep(
				std::time::Duration::from_secs(5),
			);
			Ok(())
		}
		Err(e) => {
			eprintln!(
				"Error: Could not connect: {}", e,
			);
			eprintln!(
				"Run 'ch-cli daemon start' first"
			);
			Err(std::io::Error::new(
				std::io::ErrorKind::Other,
				e.to_string(),
			))
		}
	}
}

/// Build text representations for embedding
fn build_embed_texts(
	symbols: &[crate::indexer::Symbol],
) -> Vec<String> {
	symbols
		.iter()
		.map(|s| {
			format!(
				"{} {} {}",
				s.kind,
				s.name,
				s.signature.as_deref().unwrap_or(""),
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

	for (i, batch) in
		texts.chunks(batch_size).enumerate()
	{
		match client.embed(batch.to_vec()) {
			Ok(embeddings) => {
				total += embeddings.len();
				println!(
					"  Batch {}: {} symbols ({}/{})",
					i + 1, embeddings.len(),
					total, texts.len(),
				);
			}
			Err(e) => {
				println!(
					"  Batch {}: error - {}",
					i + 1, e,
				);
			}
		}
	}
	total
}
