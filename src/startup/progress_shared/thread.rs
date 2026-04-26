use std::io;
use std::sync::atomic::Ordering;

use crossterm::{
	cursor, execute,
	terminal::{self, ClearType},
};

use crate::indexer::{IndexManagerResult, IndexResult};

use super::state::ProgressState;

/// Build a thread-safe callback that updates the shared
/// ProgressState from the indexing thread.
pub(crate) fn build_progress_callback(
	state: &ProgressState,
) -> impl Fn(usize, usize, &std::path::Path)
	+ Send
	+ Sync
	+ 'static {
	let files_ref = state.files_done.clone();
	let total_ref = state.total.clone();
	let file_ref = state.current_file.clone();

	move |current, total, path| {
		files_ref.store(current, Ordering::SeqCst);
		total_ref.store(total, Ordering::SeqCst);
		if let Ok(mut name) = file_ref.lock() {
			*name = path
				.file_name()
				.and_then(|fname| fname.to_str())
				.unwrap_or("")
				.to_string();
		}
	}
}

/// Join the indexing thread, clear the screen, and
/// return the IndexResult if the thread succeeded.
pub(crate) fn collect_index_result(
	stdout: &mut io::Stdout,
	handle: std::thread::JoinHandle<
		IndexManagerResult<IndexResult>,
	>,
) -> io::Result<Option<IndexResult>> {
	let result = handle.join().map_err(|_| {
		io::Error::other("Indexing thread panicked")
	})?;

	execute!(
		stdout,
		cursor::MoveTo(0, 0),
		terminal::Clear(ClearType::All)
	)?;

	match result {
		Ok(index_result) => Ok(Some(index_result)),
		Err(_) => Ok(None),
	}
}
