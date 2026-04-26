use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::domain::errors::command::CommandResult;
use crate::service::atlassian::credentials;
use crate::service::config;

use super::mcp_handlers;
use super::mcp_types;

pub fn mcp_server_command() -> CommandResult {
	let root = Path::new(".");
	let atl_creds =
		credentials::load_credentials(root);
	let aikido_creds =
		config::load_config(root).aikido;
	let ctx = mcp_types::McpContext::build(
		atl_creds,
		aikido_creds,
	);
	run_stdio_loop(&ctx)
}

fn run_stdio_loop(
	ctx: &mcp_types::McpContext,
) -> CommandResult {
	let stdin = io::stdin().lock();
	let mut stdout = io::stdout().lock();
	for line in stdin.lines() {
		let line = line?;
		if line.trim().is_empty() {
			continue;
		}
		if let Some(resp) =
			mcp_handlers::handle_message(
				&line, ctx,
			)
		{
			writeln!(stdout, "{resp}")?;
			stdout.flush()?;
		}
	}
	Ok(())
}
