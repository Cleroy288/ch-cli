//! Display helpers for the analysis-based prompt.
//!
//! Language notes, benefit items, and the Y/N/Q
//! row specific to the first-launch analysis prompt.

use std::io::{self, Write};

use crossterm::{
    execute,
    style::{
        Color, Print, ResetColor, SetForegroundColor,
    },
};

use crate::indexer::CodebaseAnalysis;

use super::prompts_shared::display_ynq_buttons;

/// Show primary language not-supported note
pub(super) fn display_language_note(
    stdout: &mut io::Stdout,
    analysis: &CodebaseAnalysis,
) -> io::Result<()> {
    let Some(primary) = analysis.primary_language
    else {
        return Ok(());
    };
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(format!(
            "  Note: Primary language ({}) \
            is not yet supported.\n",
            primary.display_name()
        )),
        ResetColor
    )
}

/// Show supported file count note
pub(super) fn display_supported_count(
    stdout: &mut io::Stdout,
    analysis: &CodebaseAnalysis,
) -> io::Result<()> {
    let supported = analysis.supported_file_count();
    if supported == 0 {
        return Ok(());
    }
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(format!(
            "  Will index {} supported \
            file(s).\n\n",
            supported
        )),
        ResetColor
    )
}

/// Show what first-time setup enables
pub(super) fn display_setup_benefits(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    writeln!(stdout, "  First-time setup enables:")?;
    display_benefit_items(stdout)?;
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(
            "  (Index + doc generation, \
            ~2-5 min one-time setup)\n\n",
        ),
        ResetColor
    )
}

/// Show the list of setup benefit items
fn display_benefit_items(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(
            "    - Fast symbol search across \
            all files\n",
        ),
        Print(
            "    - Go-to-definition \
            functionality\n",
        ),
        Print(
            "    - Find all references to a \
            symbol\n",
        ),
        Print(
            "    - Semantic code understanding\n",
        ),
        Print(
            "    - Auto-generated documentation \
            for all symbols\n\n",
        ),
        ResetColor
    )
}

/// Display the Y/N/Q button row for analysis prompt
pub(super) fn display_ynq_row(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    display_ynq_buttons(
        stdout,
        "  Would you like to set up your \
        codebase now?\n\n",
    )?;
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("  Press Y, N, or Q: "),
        ResetColor
    )?;
    stdout.flush()
}
