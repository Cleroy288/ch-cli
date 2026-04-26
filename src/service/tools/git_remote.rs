use std::fs;
use std::path::Path;

/// Remote host, workspace, and repo slug
pub struct RepoInfo {
    /// e.g. bitbucket.org, github.com
    pub host: String,
    pub workspace: String,
    pub repo_slug: String,
}

pub fn detect_repo_info(
    root: &Path,
) -> Option<RepoInfo> {
    let config = root.join(".git/config");
    let content = fs::read_to_string(config).ok()?;
    parse_origin_url(&content)
}

/// Find `url = ...` under `[remote "origin"]`
fn parse_origin_url(
    content: &str,
) -> Option<RepoInfo> {
    let mut in_origin = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[remote \"origin\"]" {
            in_origin = true;
            continue;
        }
        if trimmed.starts_with('[') {
            in_origin = false;
            continue;
        }
        if let Some(url) =
            trimmed.strip_prefix("url = ")
                .filter(|_| in_origin)
        {
            return parse_ssh_url(url)
                .or_else(|| parse_https_url(url));
        }
    }
    None
}

/// Extract host/workspace/repo from SSH URL
/// `git@bitbucket.org:workspace/repo.git`
fn parse_ssh_url(url: &str) -> Option<RepoInfo> {
    if url.contains("://") {
        return None;
    }
    let (user_host, path) =
        url.split_once(':')?;
    let host =
        user_host.split('@').next_back()?;
    build_repo_info(host, path)
}

/// Extract host/workspace/repo from HTTPS URL
/// `https://bitbucket.org/workspace/repo.git`
fn parse_https_url(url: &str) -> Option<RepoInfo> {
    let after_scheme = url.split("//").nth(1)?;
    let (host, path) =
        after_scheme.split_once('/')?;
    build_repo_info(host, path)
}

fn build_repo_info(
    host: &str,
    path: &str,
) -> Option<RepoInfo> {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() < 2 {
        return None;
    }
    let workspace = parts[0].to_string();
    let slug = parts[1].trim_end_matches(".git");
    Some(RepoInfo {
        host: host.to_string(),
        workspace,
        repo_slug: slug.to_string(),
    })
}
