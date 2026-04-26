use std::io::{self, Write};

use crossterm::{
	execute,
	style::{
		Color, Print, ResetColor,
		SetForegroundColor,
	},
};

use crate::indexer::{DetectedLanguage, Language};

/// Show detected language info and supported languages
pub fn show_language_info(
    stdout: &mut io::Stdout,
    primary_lang: &DetectedLanguage,
) -> io::Result<()> {
    show_detected_language(stdout, primary_lang)?;
    show_supported_list(stdout)
}

/// Show the detected primary language
fn show_detected_language(
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
    )
}

/// Show the list of supported languages
fn show_supported_list(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
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
    writeln!(stdout)
}
