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

use crate::indexer::IndexResult;

use super::progress_draw::{
    draw_file_stats, draw_progress_bar,
};
use super::progress_shared::{
    ProgressState, collect_index_result,
    draw_elapsed_time,
};
use super::progress_spawn::spawn_index_thread;

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
    let current = state.current_file_name();

    draw_progress_bar(
        stdout, processed, total, spin_idx,
    )?;
    draw_file_stats(
        stdout, processed, total, &current,
    )?;
    draw_elapsed_time(stdout, start, 7)?;
    stdout.flush()
}
