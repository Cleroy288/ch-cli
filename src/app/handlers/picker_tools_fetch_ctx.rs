use std::path::Path;

use crate::service::atlassian::credentials;
use crate::service::atlassian::types::{
    BbCredentials, JiraCredentials,
};
use crate::service::tools::git_remote;

/// BB context: credentials + workspace + repo_slug
pub(crate) type BbContext =
    (BbCredentials, String, String);

///
/// Auto-detects from git remote if Bitbucket,
/// otherwise falls back to credentials.
pub(crate) fn load_bb_context(
    root: &Path,
) -> Option<BbContext> {
    let creds =
        credentials::load_credentials(root)?;
    let bb_creds = creds.bitbucket?;
    let (workspace, slug) =
        resolve_bb_repo(&bb_creds, root)?;
    Some((bb_creds, workspace, slug))
}

fn resolve_bb_repo(
    creds: &BbCredentials,
    root: &Path,
) -> Option<(String, String)> {
    let info = git_remote::detect_repo_info(root);
    let is_bb = info
        .as_ref()
        .is_some_and(|i| {
            i.host.contains("bitbucket.org")
        });
    if is_bb {
        let i = info.unwrap();
        return Some((i.workspace, i.repo_slug));
    }
    let slug = creds.repo_slug.clone()?;
    Some((creds.workspace.clone(), slug))
}

pub(crate) fn load_jira_creds(
    root: &Path,
) -> Option<JiraCredentials> {
    credentials::load_credentials(root)?
        .jira
}
