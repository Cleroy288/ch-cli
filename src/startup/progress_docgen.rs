//! Progress display for documentation generation.
//!
//! Polls daemon DocGenStatus and displays a progress bar
//! until complete. User can press 'B' to send doc gen to
//! background and launch the TUI immediately.

use std::io;

use crate::retrieval::daemon::{
	DaemonClient, ensure_daemon_ready,
};

use super::progress_docgen_draw::{
	display_daemon_wait, display_docgen_header,
	show_daemon_error, show_start_error,
};
use super::progress_docgen_render::poll_docgen_progress;
use super::progress_docgen_stats::display_docgen_complete;

/// Poll interval for checking doc gen status
pub(crate) const POLL_INTERVAL_MS: u64 = 2000;

/// Display progress bar while generating docs.
///
/// Returns true if user pressed 'B' to background.
pub fn generate_docs_with_progress(
) -> io::Result<bool> {
	let mut stdout = io::stdout();
	let client = DaemonClient::new();
	let project_path = std::env::current_dir()
		.unwrap_or_default()
		.to_string_lossy()
		.to_string();

	display_daemon_wait(&mut stdout)?;

	if !ensure_daemon_ready() {
		show_daemon_error(&mut stdout)?;
		return Ok(false);
	}

	display_docgen_header(&mut stdout)?;
	start_doc_gen(
		&client, &mut stdout, &project_path,
	)?;

	let backgrounded = poll_docgen_progress(
		&mut stdout, &client, &project_path,
	)?;

	if !backgrounded {
		display_docgen_complete(&mut stdout)?;
	}
	Ok(backgrounded)
}

/// Fire off doc gen on daemon, show error on failure
fn start_doc_gen(
	client: &DaemonClient,
	stdout: &mut io::Stdout,
	project_path: &str,
) -> io::Result<()> {
	if let Err(err) = client
		.start_doc_gen(project_path.to_string(), false)
	{
		show_start_error(
			stdout,
			&format!("{}", err),
		)?;
	}
	Ok(())
}
