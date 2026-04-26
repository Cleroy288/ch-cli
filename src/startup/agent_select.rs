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

use crate::domain::agent::{
	AgentKind, AgentPreference,
};
use crate::service::agents::detect::detect_agents;
use crate::service::agents::preference::{
	load_agent, save_agent,
};
use crate::service::config;

use super::key_reader::{KeyAction, read_ynq_key};
use super::prompts_shared::{
	clear_and_show_header, display_ynq_buttons,
};

pub fn check_and_prompt_agent(
) -> io::Result<Option<AgentKind>> {
	let root = Path::new(".");
	if let Some(pref) = load_agent(root) {
		return Ok(Some(pref.kind));
	}
	let cfg = config::load_config(root);
	if cfg.setup.agent {
		return Ok(None);
	}
	let agents = detect_agents();
	if agents.is_empty() {
		return Ok(None);
	}
	prompt_agent_setup(root, &agents)
}

/// Display prompt and handle user response
fn prompt_agent_setup(
	root: &Path,
	agents: &[AgentKind],
) -> io::Result<Option<AgentKind>> {
	let mut stdout = io::stdout();
	display_prompt(&mut stdout, agents)?;
	terminal::enable_raw_mode()?;
	let result = read_ynq_key();
	terminal::disable_raw_mode()?;
	let action = result?;
	writeln!(stdout)?;
	handle_action(root, action, agents[0])
}

fn display_prompt(
	stdout: &mut io::Stdout,
	agents: &[AgentKind],
) -> io::Result<()> {
	clear_and_show_header(stdout)?;
	let label = agents[0].label();
	execute!(
		stdout,
		SetForegroundColor(Color::Cyan),
		Print(format!(
			"  {label} detected.\n\n"
		)),
		ResetColor,
		SetForegroundColor(Color::DarkGrey),
		Print(
			"  Discover external MCP tools?\n",
		),
		Print(
			"  (shows tools in # picker)\n\n",
		),
		ResetColor,
	)?;
	display_ynq_buttons(
		stdout,
		"  Enable discovery?\n\n",
	)
}

fn handle_action(
	root: &Path,
	action: KeyAction,
	kind: AgentKind,
) -> io::Result<Option<AgentKind>> {
	let result = match action {
		KeyAction::Yes => {
			let pref = AgentPreference { kind };
			save_agent(root, &pref)?;
			Some(kind)
		}
		KeyAction::Quit => {
			return Err(io::Error::new(
				io::ErrorKind::Interrupted,
				"User quit",
			));
		}
		KeyAction::No => None,
	};
	super::mark_setup_done(root, |s| {
		s.agent = true;
	})?;
	Ok(result)
}
