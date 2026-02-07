//! Language information display

use std::io;

use crossterm::{execute, style::*};

use crate::indexer::{DetectedLanguage, Language};

/// Show detected language info and supported languages
pub fn show_language_info(
    stdout: &mut io::Stdout,
    primary_lang: &DetectedLanguage,
) -> io::Result<()> {
    execute!(
        stdout,
        SetForegroundColor(Color::White),
        Print("  Detected primary language: "),
        SetForegroundColor(Color::Cyan),
        Print(format!(
            "{}\n\n",
            primary_lang.display_name()
        )),
        ResetColor
    )?;

    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("  Currently supported languages:\n"),
        ResetColor
    )?;

    for lang in Language::all_supported() {
        execute!(
            stdout,
            SetForegroundColor(Color::Green),
            Print(format!(
                "    - {}\n",
                lang.display_name()
            )),
            ResetColor
        )?;
    }
    println!();

    Ok(())
}
