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

use crate::service::config;
use crate::service::tools::{
	git_remote, repo_cache,
};

use super::key_reader::{KeyAction, read_ynq_key};
use super::prompts_shared::display_ynq_buttons;

pub fn check_and_prompt_repo_scan(
) -> io::Result<()> {
	let root = Path::new(".");
	if repo_cache::has_repos_cache(root) {
		return Ok(());
	}
	if has_bb_remote(root) {
		return Ok(());
	}
	let cfg = config::load_config(root);
	if cfg.setup.repo_scan {
		return Ok(());
	}
	prompt_repo_scan(root)
}

fn has_bb_remote(root: &Path) -> bool {
	git_remote::detect_repo_info(root)
		.map(|info| {
			info.host.contains("bitbucket.org")
		})
		.unwrap_or(false)
}

/// Display prompt and handle user response
fn prompt_repo_scan(
	root: &Path,
) -> io::Result<()> {
	let mut stdout = io::stdout();
	display_scan_prompt(&mut stdout)?;
	terminal::enable_raw_mode()?;
	let result = read_ynq_key();
	terminal::disable_raw_mode()?;
	let action = result?;
	writeln!(stdout)?;
	if let KeyAction::Yes = action {
		super::repo_scan_display::run_scan(
			root, &mut stdout,
		)?;
	}
	super::mark_setup_done(root, |s| {
		s.repo_scan = true;
	})
}

fn display_scan_prompt(
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	execute!(
		stdout,
		SetForegroundColor(Color::Cyan),
		Print(
			"  Scan for Bitbucket repos?\n\n",
		),
		ResetColor,
		SetForegroundColor(Color::DarkGrey),
		Print(
			"  Scans subdirectories for git repos\n",
		),
		Print(
			"  with Bitbucket remotes.\n\n",
		),
		ResetColor,
	)?;
	display_ynq_buttons(stdout, "  Scan?\n\n")
}

/// Current Unix epoch seconds as string
pub fn now_unix_secs() -> String {
	std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.unwrap_or_default()
		.as_secs()
		.to_string()
}
