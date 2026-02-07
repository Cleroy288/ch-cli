//! Startup flow for ch-cli.
//!
//! Handles index checking, user prompting, and progress
//! display during indexing.

mod checks;
mod progress;
mod progress_docgen;
mod progress_incremental;
mod prompts;
mod prompts_analysis;
mod unsupported_display;
pub mod watcher;

use std::io;

use crate::indexer::{ChangeSet, IndexResult};

pub use checks::{
	analyze_codebase, check_index_exists,
	detect_codebase_changes,
};
pub use progress::index_with_progress;
pub use progress_docgen::generate_docs_with_progress;
pub use progress_incremental::index_with_progress_incremental;
pub use prompts::{prompt_for_indexing, prompt_for_update};
pub use prompts_analysis::prompt_for_indexing_with_analysis;
pub use watcher::{spawn_watcher, WatcherHandle};

use unsupported_display::display_unsupported_language_message;

/// Result of the startup check
pub enum StartupAction {
	/// User wants to index the codebase (full index)
	Index,
	/// User wants to update the index (incremental)
	Update(ChangeSet),
	/// User declined indexing
	Skip,
	/// Index already exists and is up to date
	UpToDate,
	/// User wants to quit
	Quit,
	/// User wants to index + generate docs (first-launch setup)
	IndexAndGenerateDocs,
}

/// Run the complete startup flow
pub fn run_startup() -> io::Result<Option<IndexResult>> {
	// Step 1: Analyze codebase language
	let analysis = analyze_codebase();

	// Step 2: Check if primary language is supported
	if !analysis.is_primary_supported
		&& !analysis.has_supported_files()
	{
		if let Some(primary) = analysis.primary_language {
			display_unsupported_language_message(
				primary,
				0,
				analysis.total_source_files,
			)?;
		}
		return Ok(None);
	}

	// Step 3: Check if index already exists
	if check_index_exists() {
		handle_existing_index()
	} else {
		handle_first_launch(&analysis)
	}
}

/// Handle startup when index already exists
fn handle_existing_index() -> io::Result<Option<IndexResult>> {
	if let Some(changes) = detect_codebase_changes() {
		match prompt_for_update(&changes)? {
			StartupAction::Update(_) => {
				// run incremental index (blocking, fast)
				let result = index_with_progress_incremental(&changes)?;

				// spawn background doc update (non-blocking)
				spawn_background_doc_update();

				Ok(result)
			}
			StartupAction::Skip => Ok(None),
			StartupAction::Quit => {
				Err(io::Error::new(io::ErrorKind::Interrupted, "User quit"))
			}
			_ => Ok(None),
		}
	} else {
		// no changes, index is up to date
		Ok(None)
	}
}

/// Handle first-launch setup (no index exists)
fn handle_first_launch(
	analysis: &crate::indexer::CodebaseAnalysis,
) -> io::Result<Option<IndexResult>> {
	let show_partial_note = !analysis.is_primary_supported
		&& analysis.has_supported_files();

	match prompt_for_indexing_with_analysis(
		show_partial_note,
		analysis,
	)? {
		StartupAction::Index | StartupAction::IndexAndGenerateDocs => {
			// Step 1/3: Index codebase
			let result = index_with_progress()?;

			// Step 2/3 + 3/3: Start daemon + generate docs with progress
			if result.is_some() {
				generate_docs_with_progress()?;
			}

			Ok(result)
		}
		StartupAction::Skip => Ok(None),
		StartupAction::Quit => {
			Err(io::Error::new(io::ErrorKind::Interrupted, "User quit"))
		}
		_ => Ok(None),
	}
}

/// Spawn background thread to update docs for changed symbols
fn spawn_background_doc_update() {
	std::thread::spawn(|| {
		let client = crate::retrieval::daemon::DaemonClient::new();

		// check if daemon is alive
		if client.ping().is_err() {
			return; // daemon not running, skip
		}

		// trigger doc gen (non-blocking on daemon side now)
		let project_path = std::env::current_dir()
			.unwrap_or_default()
			.to_string_lossy()
			.to_string();

		match client.start_doc_gen(project_path, false) {
			Ok(_) => eprintln!(
				"[startup] Background doc update started"
			),
			Err(e) => eprintln!(
				"[startup] Background doc update failed: {}",
				e
			),
		}
	});
}
