use std::io::{self, Write};
use std::path::Path;

use crossterm::{
	execute,
	style::{
		Color, Print, ResetColor,
		SetForegroundColor,
	},
	terminal,
};

use crate::service::atlassian::credentials
	::has_credentials;
use crate::service::config;

use super::key_reader::{KeyAction, read_ynq_key};
use super::prompts_shared::{
	clear_and_show_header, display_ynq_buttons,
};

pub fn check_and_prompt_credentials(
) -> io::Result<()> {
	let root = Path::new(".");
	if has_credentials(root) {
		return Ok(());
	}
	let cfg = config::load_config(root);
	if cfg.setup.credentials {
		return Ok(());
	}
	prompt_credential_setup(root)
}

/// Display prompt and handle user response
fn prompt_credential_setup(
	root: &Path,
) -> io::Result<()> {
	let mut stdout = io::stdout();
	display_prompt(&mut stdout)?;
	terminal::enable_raw_mode()?;
	let result = read_ynq_key();
	terminal::disable_raw_mode()?;
	let action = result?;
	writeln!(stdout)?;
	if let KeyAction::Yes = action {
		super::credentials_collect
			::collect_and_save(
				root, &mut stdout,
			)?;
	}
	super::mark_setup_done(root, |s| {
		s.credentials = true;
	})
}

fn display_prompt(
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	clear_and_show_header(stdout)?;
	execute!(
		stdout,
		SetForegroundColor(Color::Cyan),
		Print(
			"  Configure Atlassian tools?\n\n",
		),
		ResetColor,
		SetForegroundColor(Color::DarkGrey),
		Print(
			"  Enables Bitbucket + Jira in MCP\n",
		),
		Print(
			"  (press Enter to skip a field)\n\n",
		),
		ResetColor,
	)?;
	display_ynq_buttons(
		stdout,
		"  Set up now?\n\n",
	)
}

pub fn prompt_optional(
	stdout: &mut io::Stdout,
	label: &str,
) -> io::Result<Option<String>> {
	let val = prompt_input(stdout, label)?;
	Ok(if val.is_empty() {
		None
	} else {
		Some(val)
	})
}

pub fn prompt_input(
	stdout: &mut io::Stdout,
	label: &str,
) -> io::Result<String> {
	write!(stdout, "{label}")?;
	stdout.flush()?;
	let mut buf = String::new();
	io::stdin().read_line(&mut buf)?;
	Ok(buf.trim().to_string())
}
