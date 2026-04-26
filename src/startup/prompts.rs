use std::io::{self, Write};

use crossterm::{
    execute,
    style::{
        Color, Print, ResetColor, SetForegroundColor,
    },
    terminal,
};

use crate::indexer::ChangeSet;

use super::key_reader::{read_ynq_key, KeyAction};
use super::prompts_shared::{
    clear_and_show_header, display_loading_hint,
    display_ynq_buttons,
};
use super::StartupAction;

/// Prompt user to decide whether to update the index
pub fn prompt_for_update(
    changes: &ChangeSet,
) -> io::Result<StartupAction> {
    let mut stdout = io::stdout();
    display_update_prompt(&mut stdout, changes)?;
    read_update_choice(&mut stdout, changes)
}

fn display_update_prompt(
    stdout: &mut io::Stdout,
    changes: &ChangeSet,
) -> io::Result<()> {
    clear_and_show_header(stdout)?;

    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print(
            "  Changes detected in your codebase!\n\n",
        ),
        ResetColor
    )?;

    display_change_summary(stdout, changes)?;
    display_ynq_buttons(
        stdout,
        "  Would you like to update your index?\n\n",
    )?;
    display_loading_hint(stdout)
}

/// Show change summary (added/modified/deleted)
fn display_change_summary(
    stdout: &mut io::Stdout,
    changes: &ChangeSet,
) -> io::Result<()> {
    print_change_if_nonempty(
        stdout, &changes.added,
        Color::Green, "+ {} new file(s)",
    )?;
    print_change_if_nonempty(
        stdout, &changes.modified,
        Color::Yellow, "~ {} modified file(s)",
    )?;
    print_change_if_nonempty(
        stdout, &changes.deleted,
        Color::Red, "- {} deleted file(s)",
    )?;
    execute!(stdout, ResetColor, Print("\n"))
}

/// Print a change line if the list is non-empty
fn print_change_if_nonempty<T>(
    stdout: &mut io::Stdout,
    items: &[T],
    color: Color,
    fmt: &str,
) -> io::Result<()> {
    if items.is_empty() {
        return Ok(());
    }
    let msg = fmt.replacen(
        "{}", &items.len().to_string(), 1,
    );
    execute!(
        stdout,
        SetForegroundColor(color),
        Print(format!("    {}\n", msg)),
    )
}

fn read_update_choice(
    stdout: &mut io::Stdout,
    changes: &ChangeSet,
) -> io::Result<StartupAction> {
    terminal::enable_raw_mode()?;

    let result = read_ynq_key();
    terminal::disable_raw_mode()?;
    let action = match result? {
        KeyAction::Yes => {
            StartupAction::Update(changes.clone())
        }
        KeyAction::No => StartupAction::Skip,
        KeyAction::Quit => StartupAction::Quit,
    };
    writeln!(stdout)?;
    Ok(action)
}
