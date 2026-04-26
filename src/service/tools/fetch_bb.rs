use crate::domain::errors::atlassian::AtlassianResult;
use crate::domain::tool_ref::ToolItem;
use crate::service::atlassian::client_bb::BbClient;

use super::bb_types::{
    BbBranch, BbPage, BbPullRequest,
};

pub fn fetch_branches(
    client: &BbClient,
    workspace: &str,
    repo_slug: &str,
) -> AtlassianResult<Vec<ToolItem>> {
    let path = format!(
        "/repositories/{workspace}/{repo_slug}\
        /refs/branches?pagelen=50"
    );
    let json = client.get(&path)?;
    let page: BbPage<BbBranch> =
        serde_json::from_str(&json)?;
    Ok(map_branches(page.values))
}

fn map_branches(
    branches: Vec<BbBranch>,
) -> Vec<ToolItem> {
    branches
        .into_iter()
        .map(|branch| ToolItem {
            key: branch.name.clone(),
            display: branch.name,
            description: String::new(),
        })
        .collect()
}

pub fn fetch_pull_requests(
    client: &BbClient,
    workspace: &str,
    repo_slug: &str,
) -> AtlassianResult<Vec<ToolItem>> {
    let path = format!(
        "/repositories/{workspace}/{repo_slug}\
        /pullrequests?state=OPEN&pagelen=50"
    );
    let json = client.get(&path)?;
    let page: BbPage<BbPullRequest> =
        serde_json::from_str(&json)?;
    Ok(map_pull_requests(page.values))
}

fn map_pull_requests(
    pull_requests: Vec<BbPullRequest>,
) -> Vec<ToolItem> {
    pull_requests
        .into_iter()
        .map(|pull| ToolItem {
            key: pull.id.to_string(),
            display: format!(
                "{} - {}", pull.id, pull.title
            ),
            description: pull.state,
        })
        .collect()
}
