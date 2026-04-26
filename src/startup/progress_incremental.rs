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

use crate::indexer::{ChangeSet, IndexResult};

use super::progress_incremental_draw::{
    draw_incremental_bar, draw_incremental_stats,
};
use super::progress_shared::{
    ProgressState, collect_index_result,
    draw_elapsed_time,
};
use super::progress_spawn::spawn_index_thread;

/// Display a progress bar during incremental indexing
pub fn index_with_progress_incremental(
    changes: &ChangeSet,
) -> io::Result<Option<IndexResult>> {
    let mut stdout = io::stdout();
    display_update_header(&mut stdout, changes)?;

    let state = ProgressState::new();
    let handle = spawn_index_thread(&state);

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
    let current = state.current_file_name();

    draw_incremental_bar(
        stdout, processed, total, spin_idx,
    )?;
    draw_incremental_stats(
        stdout, processed, total, &current,
    )?;
    draw_elapsed_time(stdout, start, 9)?;
    stdout.flush()
}
