use std::io::{self, Write};
use std::path::Path;

use crossterm::terminal;

use crate::domain::config::IntegrationMode;
use crate::service::config;

use super::keys::Choice;

pub fn check_and_prompt_integration_mode(
) -> io::Result<()> {
	let root = Path::new(".");
	let cfg = config::load_config(root);
	if cfg.claude.integration
		!= IntegrationMode::Cli
	{
		return Ok(());
	}
	if cfg.setup.integration {
		return Ok(());
	}
	prompt_integration_mode(root)
}

/// Display prompt and handle user choice
fn prompt_integration_mode(
	root: &Path,
) -> io::Result<()> {
	let mut stdout = io::stdout();
	super::display::render_menu(&mut stdout)?;
	terminal::enable_raw_mode()?;
	let result = super::keys::read_choice_key();
	terminal::disable_raw_mode()?;
	let choice = result?;
	writeln!(stdout)?;
	handle_choice(root, &mut stdout, choice)
}

fn handle_choice(
	root: &Path,
	stdout: &mut io::Stdout,
	choice: Choice,
) -> io::Result<()> {
	match choice {
		Choice::Cli => {
			save_mode(root, IntegrationMode::Cli)?;
		}
		Choice::Api => {
			prompt_api_key(root, stdout)?;
		}
		Choice::Quit => {
			return Err(io::Error::new(
				io::ErrorKind::Interrupted,
				"User quit",
			));
		}
	}
	crate::startup::mark_setup_done(root, |s| {
		s.integration = true;
	})
}

/// Prompt for API key and save
fn prompt_api_key(
	root: &Path,
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	write!(stdout, "  API key: ")?;
	stdout.flush()?;
	let mut key = String::new();
	io::stdin().read_line(&mut key)?;
	let key = key.trim().to_string();
	if key.is_empty() {
		return save_mode(
			root, IntegrationMode::Cli,
		);
	}
	save_mode(
		root,
		IntegrationMode::Api { api_key: key },
	)
}

fn save_mode(
	root: &Path,
	mode: IntegrationMode,
) -> io::Result<()> {
	config::update_config(root, move |cfg| {
		cfg.claude.integration = mode;
	})
}
