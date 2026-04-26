use std::io;

use crossterm::{
	execute,
	style::{
		Color, Print, ResetColor,
		SetForegroundColor,
	},
};

pub fn render_menu(
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	render_header(stdout)?;
	render_cli_option(stdout)?;
	render_api_option(stdout)?;
	render_buttons(stdout)
}

fn render_header(
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	execute!(
		stdout,
		SetForegroundColor(Color::Cyan),
		Print("  How should rustean connect"),
		Print(" to Claude?\n\n"),
		ResetColor,
	)
}

fn render_cli_option(
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	execute!(
		stdout,
		SetForegroundColor(Color::Green),
		Print("  [1] CLI"),
		ResetColor,
		Print(" — spawn `claude` subprocess\n"),
		SetForegroundColor(Color::DarkGrey),
		Print("      Streaming, session resume\n"),
		Print("      MCP tools via Claude Code\n"),
		Print("      Included in subscription\n\n"),
		ResetColor,
	)
}

fn render_api_option(
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	execute!(
		stdout,
		SetForegroundColor(Color::Green),
		Print("  [2] API"),
		ResetColor,
		Print(
			" — call Anthropic API directly\n",
		),
		SetForegroundColor(Color::DarkGrey),
		Print("      Custom tools, model mixing\n"),
		Print("      Deterministic pipelines\n"),
		Print(
			"      Requires API key (pay/token)\n\n",
		),
		ResetColor,
	)
}

fn render_buttons(
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	execute!(
		stdout,
		SetForegroundColor(Color::White),
		Print("    ["),
		SetForegroundColor(Color::Green),
		Print("1"),
		SetForegroundColor(Color::White),
		Print("]  ["),
		SetForegroundColor(Color::Green),
		Print("2"),
		SetForegroundColor(Color::White),
		Print("]  ["),
		SetForegroundColor(Color::Yellow),
		Print("Q"),
		SetForegroundColor(Color::White),
		Print("]uit\n\n"),
		ResetColor,
	)
}
