use std::path::Path;
use std::process::Command;

use crate::domain::errors::git::{
	GitError, GitResult,
};
use crate::domain::git_commit::GitCommit;

use super::log_parse_fields::{
	parse_parents, parse_refs,
};

const FIELD_SEP: &str = "\x1f";
const MAX_COMMITS: u32 = 500;

/// Shell out to `git log` and parse output.
pub fn fetch_git_log(
	repo: &Path,
) -> GitResult<Vec<GitCommit>> {
	let format_str = format!(
		"%H{s}%h{s}%an{s}%ar{s}%s{s}%P{s}%D",
		s = FIELD_SEP,
	);
	let output = run_git_log(repo, &format_str)?;
	parse_log_output(&output)
}

fn run_git_log(
	repo: &Path,
	format_str: &str,
) -> GitResult<String> {
	let output = Command::new("git")
		.arg("log")
		.arg("--all")
		.arg("--topo-order")
		.arg(format!("--format={format_str}"))
		.arg(format!("--max-count={MAX_COMMITS}"))
		.current_dir(repo)
		.output()?;
	if !output.status.success() {
		let stderr =
			String::from_utf8_lossy(&output.stderr);
		return Err(GitError::Command(
			stderr.trim().to_string(),
		));
	}
	Ok(String::from_utf8_lossy(&output.stdout)
		.to_string())
}

fn parse_log_output(
	raw: &str,
) -> GitResult<Vec<GitCommit>> {
	raw.lines()
		.filter(|line| !line.is_empty())
		.map(parse_commit_line)
		.collect()
}

fn parse_commit_line(
	line: &str,
) -> GitResult<GitCommit> {
	let parts: Vec<&str> =
		line.split(FIELD_SEP).collect();
	if parts.len() < 5 {
		return Err(GitError::Parse(
			format!("bad line: {line}"),
		));
	}
	let parent_hashes = parse_parents(
		parts.get(5).unwrap_or(&""),
	);
	let refs = parse_refs(
		parts.get(6).unwrap_or(&""),
	);
	Ok(GitCommit {
		hash: parts[0].to_string(),
		short_hash: parts[1].to_string(),
		author: parts[2].to_string(),
		date: parts[3].to_string(),
		message: parts[4].to_string(),
		parent_hashes,
		refs,
	})
}

