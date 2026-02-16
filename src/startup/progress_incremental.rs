//! Progress display for incremental indexing.

use std::io::{self, Write};
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use crossterm::{
    cursor, execute,
    style::{
        Color, Print, ResetColor, SetForegroundColor,
    },
    terminal::{self, ClearType},
};

use crate::indexer::{
    ChangeSet, IndexManager, IndexResult,
};

use super::progress_shared::{
    ProgressState, SPINNER, build_progress_callback,
    collect_index_result, draw_elapsed_time,
    format_progress_bar, truncate_name,
};

/// Display a progress bar during incremental indexing
pub fn index_with_progress_incremental(
    changes: &ChangeSet,
) -> io::Result<Option<IndexResult>> {
    let mut stdout = io::stdout();
    display_update_header(&mut stdout, changes)?;

    let state = ProgressState::new();
    let handle = spawn_incremental_thread(&state);

    run_incremental_loop(&mut stdout, &state)?;
    collect_index_result(&mut stdout, handle)
}

/// Show the incremental update header
fn display_update_header(
    stdout: &mut io::Stdout,
    changes: &ChangeSet,
) -> io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    writeln!(stdout)?;
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("  Updating index...\n\n"),
        ResetColor
    )?;

    let file_count =
        changes.added.len() + changes.modified.len();
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(format!(
            "  {} file(s) to re-index\n\n",
            file_count
        )),
        ResetColor
    )
}

/// Spawn the incremental indexing thread
fn spawn_incremental_thread(
    state: &ProgressState,
) -> std::thread::JoinHandle<
    crate::indexer::IndexManagerResult<IndexResult>,
> {
    let callback = build_progress_callback(state);
    let done_clone = state.done.clone();

    std::thread::spawn(move || {
        let manager = IndexManager::new()
            .with_persistence()
            .with_semantic_analysis()
            .with_reference_extraction()
            .on_progress(callback);

        let result = manager.index_project(".");
        done_clone.store(true, Ordering::SeqCst);
        result
    })
}

/// Run the progress display loop
fn run_incremental_loop(
    stdout: &mut io::Stdout,
    state: &ProgressState,
) -> io::Result<()> {
    let mut spin_idx = 0;
    let start = Instant::now();

    while !state.done.load(Ordering::SeqCst) {
        draw_incremental_frame(
            stdout, state, spin_idx, &start,
        )?;
        spin_idx += 1;
        std::thread::sleep(
            Duration::from_millis(80),
        );
    }
    Ok(())
}

/// Draw one frame of incremental progress
fn draw_incremental_frame(
    stdout: &mut io::Stdout,
    state: &ProgressState,
    spin_idx: usize,
    start: &Instant,
) -> io::Result<()> {
    let processed =
        state.files_done.load(Ordering::SeqCst);
    let total =
        state.total.load(Ordering::SeqCst).max(1);
    let current = state
        .current_file
        .lock()
        .map(|val| val.clone())
        .unwrap_or_default();

    draw_incremental_bar(
        stdout, processed, total, spin_idx,
    )?;
    draw_incremental_stats(
        stdout, processed, total, &current,
    )?;
    draw_elapsed_time(stdout, start, 9)?;
    stdout.flush()
}

/// Draw one frame of the incremental progress bar
fn draw_incremental_bar(
    stdout: &mut io::Stdout,
    processed: usize,
    total: usize,
    spin_idx: usize,
) -> io::Result<()> {
    let (progress_bar, progress) =
        format_progress_bar(processed, total);
    let spinner = SPINNER[spin_idx % SPINNER.len()];

    execute!(
        stdout,
        cursor::MoveTo(0, 5),
        terminal::Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::Yellow),
        Print(format!("  {} ", spinner)),
        SetForegroundColor(Color::Cyan),
        Print("["),
        SetForegroundColor(Color::Yellow),
        Print(&progress_bar),
        SetForegroundColor(Color::Cyan),
        Print("]"),
        SetForegroundColor(Color::White),
        Print(format!(
            " {:.0}%",
            progress * 100.0
        )),
        ResetColor
    )
}

/// Draw file count and current file name
fn draw_incremental_stats(
    stdout: &mut io::Stdout,
    processed: usize,
    total: usize,
    current: &str,
) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, 7),
        terminal::Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey),
        Print(format!(
            "  Files: {}/{}",
            processed, total
        )),
        ResetColor
    )?;

    let display = truncate_name(current, 50);
    execute!(
        stdout,
        cursor::MoveTo(0, 8),
        terminal::Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  Current: {}", display)),
        ResetColor
    )
}
