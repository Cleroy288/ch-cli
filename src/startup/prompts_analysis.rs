//! User prompts with codebase analysis information.

use std::io::{self, Write};

use crossterm::{
    execute,
    style::{
        Color, Print, ResetColor, SetForegroundColor,
    },
    terminal,
};

use crate::indexer::CodebaseAnalysis;

use super::key_reader::{read_ynq_key, KeyAction};
use super::prompts_analysis_display::{
    display_language_note, display_setup_benefits,
    display_supported_count, display_ynq_row,
};
use super::prompts_shared::clear_and_show_header;
use super::StartupAction;

/// Prompt for indexing with optional language analysis
pub fn prompt_for_indexing_with_analysis(
    show_partial_note: bool,
    analysis: &CodebaseAnalysis,
) -> io::Result<StartupAction> {
    let mut stdout = io::stdout();
    display_analysis_prompt(
        &mut stdout,
        show_partial_note,
        analysis,
    )?;
    read_analysis_choice(&mut stdout)
}

/// Display the analysis prompt screen
fn display_analysis_prompt(
    stdout: &mut io::Stdout,
    show_partial_note: bool,
    analysis: &CodebaseAnalysis,
) -> io::Result<()> {
    clear_and_show_header(stdout)?;

    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print(
            "  No code index found for this \
            project.\n\n",
        ),
        ResetColor
    )?;

    if show_partial_note {
        display_language_note(stdout, analysis)?;
        display_supported_count(stdout, analysis)?;
    }

    display_setup_benefits(stdout)?;
    display_ynq_row(stdout)
}

/// Read the user's Y/N/Q choice
fn read_analysis_choice(
    stdout: &mut io::Stdout,
) -> io::Result<StartupAction> {
    terminal::enable_raw_mode()?;

    let result = match read_ynq_key()? {
        KeyAction::Yes => StartupAction::Index,
        KeyAction::No => StartupAction::Skip,
        KeyAction::Quit => StartupAction::Quit,
    };

    terminal::disable_raw_mode()?;
    writeln!(stdout)?;
    Ok(result)
}
