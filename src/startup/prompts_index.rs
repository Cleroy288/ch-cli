use std::io::{self, Write};

use crossterm::{
    execute,
    style::{
        Color, Print, ResetColor, SetForegroundColor,
    },
    terminal,
};

use super::key_reader::{read_ynq_key, KeyAction};
use super::prompts_shared::{
    clear_and_show_header, display_loading_hint,
    display_ynq_buttons,
};
use super::StartupAction;

pub fn prompt_for_indexing(
) -> io::Result<StartupAction> {
    let mut stdout = io::stdout();
    display_index_prompt(&mut stdout)?;
    read_index_choice(&mut stdout)
}

fn display_index_prompt(
    stdout: &mut io::Stdout,
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

    display_index_benefits(stdout)?;
    display_ynq_buttons(
        stdout,
        "  Would you like to index your \
        codebase now?\n\n",
    )?;
    display_loading_hint(stdout)
}

fn display_index_benefits(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    writeln!(
        stdout, "  Indexing your codebase enables:"
    )?;
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
            "    - Semantic code understanding\
            \n\n",
        ),
        ResetColor
    )
}

fn read_index_choice(
    stdout: &mut io::Stdout,
) -> io::Result<StartupAction> {
    terminal::enable_raw_mode()?;

    let result = read_ynq_key();
    terminal::disable_raw_mode()?;
    let action = match result? {
        KeyAction::Yes => StartupAction::Index,
        KeyAction::No => StartupAction::Skip,
        KeyAction::Quit => StartupAction::Quit,
    };
    writeln!(stdout)?;
    Ok(action)
}
