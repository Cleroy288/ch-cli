//! Shared utilities for progress display screens.
//!
//! Constants, state, and formatting helpers reused by
//! full indexing, incremental indexing, and doc generation
//! progress displays.

use std::io;
use std::sync::atomic::{
    AtomicBool, AtomicUsize, Ordering,
};
use std::sync::Arc;
use std::time::Instant;

use crossterm::{
    cursor, execute,
    style::{
        Color, Print, ResetColor, SetForegroundColor,
    },
    terminal::{self, ClearType},
};

use crate::indexer::{IndexManagerResult, IndexResult};

/// Spinner animation frames
pub(crate) const SPINNER: [&str; 10] = [
    "⠋", "⠙", "⠹", "⠸", "⠼",
    "⠴", "⠦", "⠧", "⠇", "⠏",
];

/// Progress bar width in characters
pub(crate) const BAR_WIDTH: usize = 40;

/// Shared progress state between indexing thread
/// and display loop
pub(crate) struct ProgressState {
    /// number of processed files
    pub files_done: Arc<AtomicUsize>,
    /// total number of files
    pub total: Arc<AtomicUsize>,
    /// current file name being processed
    pub current_file: Arc<std::sync::Mutex<String>>,
    /// flag indicating indexing is done
    pub done: Arc<AtomicBool>,
}

impl ProgressState {
    /// Create a new shared progress state
    pub fn new() -> Self {
        Self {
            files_done: Arc::new(
                AtomicUsize::new(0),
            ),
            total: Arc::new(AtomicUsize::new(0)),
            current_file: Arc::new(
                std::sync::Mutex::new(String::new()),
            ),
            done: Arc::new(AtomicBool::new(false)),
        }
    }
}

/// Build a formatted progress bar string
pub(crate) fn format_progress_bar(
    processed: usize,
    total: usize,
) -> (String, f64) {
    let progress =
        (processed as f64 / total as f64).min(1.0);
    let filled =
        (progress * BAR_WIDTH as f64) as usize;
    let empty = BAR_WIDTH - filled;
    let filled_str = format!(
        "{}{}",
        "█".repeat(filled),
        "░".repeat(empty),
    );
    (filled_str, progress)
}

/// Draw elapsed time at the given row
pub(crate) fn draw_elapsed_time(
    stdout: &mut io::Stdout,
    start: &Instant,
    row: u16,
) -> io::Result<()> {
    let elapsed = start.elapsed().as_secs();
    execute!(
        stdout,
        cursor::MoveTo(0, row),
        terminal::Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  Elapsed: {}s", elapsed)),
        ResetColor
    )
}

/// Truncate a filename for display
pub(crate) fn truncate_name(
    name: &str,
    max_len: usize,
) -> String {
    if name.len() > max_len {
        format!(
            "...{}",
            &name[name.len() - max_len + 3..]
        )
    } else {
        name.to_string()
    }
}

/// Build the progress callback closure
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

/// Join the indexing thread and return its result
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
