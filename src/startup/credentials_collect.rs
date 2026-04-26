use std::io::{self, Write};

use crossterm::{
	execute,
	style::{
		Color, Print, ResetColor,
		SetForegroundColor,
	},
};

use crate::service::atlassian::credentials
	::save_credentials;
use crate::service::atlassian::types::{
	AtlassianCredentials, BbCredentials,
	JiraCredentials,
};

use super::credentials::{
	prompt_input, prompt_optional,
};

pub fn collect_and_save(
	root: &std::path::Path,
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	writeln!(stdout)?;
	let bitbucket = collect_bb_creds(stdout)?;
	let jira = collect_jira_creds(stdout)?;
	let creds = AtlassianCredentials {
		bitbucket,
		jira,
	};
	save_credentials(root, &creds)?;
	execute!(
		stdout,
		SetForegroundColor(Color::Green),
		Print("\n  Credentials saved.\n\n"),
		ResetColor,
	)
}

/// Prompt for Bitbucket credentials
fn collect_bb_creds(
	stdout: &mut io::Stdout,
) -> io::Result<Option<BbCredentials>> {
	let email = prompt_input(
		stdout, "  BITBUCKET_EMAIL: ",
	)?;
	if email.is_empty() {
		return Ok(None);
	}
	let token = prompt_input(
		stdout, "  BITBUCKET_TOKEN: ",
	)?;
	let workspace = prompt_input(
		stdout, "  BITBUCKET_WORKSPACE: ",
	)?;
	let repo_slug = prompt_optional(
		stdout,
		"  BITBUCKET_REPO_SLUG (optional): ",
	)?;
	Ok(Some(BbCredentials {
		email,
		token,
		workspace,
		repo_slug,
	}))
}

/// Prompt for Jira credentials
fn collect_jira_creds(
	stdout: &mut io::Stdout,
) -> io::Result<Option<JiraCredentials>> {
	let email = prompt_input(
		stdout, "  JIRA_EMAIL: ",
	)?;
	if email.is_empty() {
		return Ok(None);
	}
	let token = prompt_input(
		stdout, "  JIRA_TOKEN: ",
	)?;
	let base_url = prompt_input(
		stdout, "  JIRA_BASE_URL: ",
	)?;
	let cloud_id = prompt_input(
		stdout, "  JIRA_CLOUD_ID: ",
	)?;
	let project_key = prompt_optional(
		stdout,
		"  JIRA_PROJECT_KEY (optional): ",
	)?;
	Ok(Some(JiraCredentials {
		email,
		token,
		base_url,
		cloud_id,
		project_key,
	}))
}
