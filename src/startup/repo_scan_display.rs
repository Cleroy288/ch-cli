use std::io::{self, Write};

use crossterm::{
	execute,
	style::{
		Color, Print, ResetColor,
		SetForegroundColor,
	},
};

use crate::domain::repo_info::{
	RepoCache, RepoEntry,
};
use crate::service::tools::{
	repo_cache, repo_discovery,
};

pub fn run_scan(
	root: &std::path::Path,
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	let depth: u32 = 2;
	let repos =
		repo_discovery::discover_repos(root, depth);
	display_results(stdout, &repos)?;
	let cache = RepoCache {
		scan_depth: depth,
		discovered_at: super::repo_scan::now_unix_secs(),
		repos,
	};
	repo_cache::save_repos(root, &cache)
}

/// Display scan results summary
fn display_results(
	stdout: &mut io::Stdout,
	repos: &[RepoEntry],
) -> io::Result<()> {
	if repos.is_empty() {
		return execute!(
			stdout,
			SetForegroundColor(Color::Yellow),
			Print("  No repos found.\n\n"),
			ResetColor,
		);
	}
	display_found_repos(stdout, repos)
}

/// Display list of found repos
fn display_found_repos(
	stdout: &mut io::Stdout,
	repos: &[RepoEntry],
) -> io::Result<()> {
	execute!(
		stdout,
		SetForegroundColor(Color::Green),
		Print(format!(
			"  Found {} repos:\n",
			repos.len(),
		)),
		ResetColor,
	)?;
	for repo in repos {
		writeln!(
			stdout,
			"    - {} ({}/{})",
			repo.folder,
			repo.workspace,
			repo.repo_slug,
		)?;
	}
	writeln!(stdout)
}
