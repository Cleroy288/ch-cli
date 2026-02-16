//! Progress display for full indexing.

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

use crate::indexer::{IndexManager, IndexResult};

use super::progress_shared::{
    ProgressState, SPINNER, build_progress_callback,
    collect_index_result, draw_elapsed_time,
    format_progress_bar, truncate_name,
};

/// Display a progress bar during indexing
pub fn index_with_progress(
) -> io::Result<Option<IndexResult>> {
    let mut stdout = io::stdout();
    display_index_header(&mut stdout)?;

    let state = ProgressState::new();
    let handle = spawn_index_thread(&state);

    run_progress_loop(&mut stdout, &state)?;
    collect_index_result(&mut stdout, handle)
}

/// Show the indexing header
fn display_index_header(
    stdout: &mut io::Stdout,
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
        Print("  Indexing codebase...\n\n"),
        ResetColor
    )
}

/// Spawn indexing thread with progress callbacks
fn spawn_index_thread(
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

/// Run the progress display loop until done
fn run_progress_loop(
    stdout: &mut io::Stdout,
    state: &ProgressState,
) -> io::Result<()> {
    let mut spin_idx = 0;
    let start = Instant::now();

    while !state.done.load(Ordering::SeqCst) {
        draw_progress_frame(
            stdout, state, spin_idx, &start,
        )?;
        spin_idx += 1;
        std::thread::sleep(
            Duration::from_millis(80),
        );
    }
    Ok(())
}

/// Draw one frame of the full-index progress
fn draw_progress_frame(
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

    draw_progress_bar(
        stdout, processed, total, spin_idx,
    )?;
    draw_file_stats(
        stdout, processed, total, &current,
    )?;
    draw_elapsed_time(stdout, start, 7)?;
    stdout.flush()
}

/// Draw the progress bar line
fn draw_progress_bar(
    stdout: &mut io::Stdout,
    processed: usize,
    total: usize,
    spin_idx: usize,
) -> io::Result<()> {
    let (progress_bar, progress) =
        format_progress_bar(processed, total);
    let spinner =
        SPINNER[spin_idx % SPINNER.len()];

    execute!(
        stdout,
        cursor::MoveTo(0, 3),
        terminal::Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::Green),
        Print(format!("  {} ", spinner)),
        SetForegroundColor(Color::Cyan),
        Print("["),
        SetForegroundColor(Color::Green),
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
fn draw_file_stats(
    stdout: &mut io::Stdout,
    processed: usize,
    total: usize,
    current: &str,
) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, 5),
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
        cursor::MoveTo(0, 6),
        terminal::Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  Current: {}", display)),
        ResetColor
    )
}
