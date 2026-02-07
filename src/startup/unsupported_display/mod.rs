//! Display handler for unsupported language messages.

mod file_stats;
mod input;
mod language_info;
mod screen;

use std::io;

use crate::indexer::DetectedLanguage;

use file_stats::show_file_stats;
use input::wait_for_keypress;
use language_info::show_language_info;
use screen::{clear_screen, clear_screen_and_show_header};

/// Display message when primary language is not supported
pub fn display_unsupported_language_message(
    primary_lang: DetectedLanguage,
    supported_count: usize,
    total_count: usize,
) -> io::Result<()> {
    let mut stdout = io::stdout();

    clear_screen_and_show_header(&mut stdout)?;
    show_language_info(&mut stdout, &primary_lang)?;
    show_file_stats(&mut stdout, supported_count, total_count)?;
    wait_for_keypress()?;
    clear_screen(&mut stdout)?;

    Ok(())
}
