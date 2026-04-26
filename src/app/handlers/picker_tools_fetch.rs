use std::sync::mpsc;

use crate::app::App;
use crate::domain::errors::AtlassianError;
use crate::domain::repo_info::RepoEntry;
use crate::domain::tool_ref::{
    ToolFetchResult, ToolKind,
};
use crate::service::atlassian::client_bb::BbClient;
use crate::service::atlassian::types::BbCredentials;
use crate::service::tools::fetch_bb;
use crate::ui::strings::tui_labels;

/// Channel sender type for tool results
pub(crate) type ToolTx = mpsc::Sender<
    Result<ToolFetchResult, AtlassianError>,
>;

/// Spawn branch fetch thread
pub(crate) fn spawn_bb_branches(
    creds: BbCredentials,
    workspace: String,
    slug: String,
    sender: ToolTx,
) {
    std::thread::spawn(move || {
        let client = BbClient::new(creds);
        let result = fetch_bb::fetch_branches(
            &client, &workspace, &slug,
        );
        let _ = sender.send(
            result.map(ToolFetchResult::Items),
        );
    });
}

/// Spawn PR fetch thread (public for reuse)
pub(crate) fn spawn_bb_prs_pub(
    creds: BbCredentials,
    workspace: String,
    slug: String,
    sender: ToolTx,
) {
    std::thread::spawn(move || {
        let client = BbClient::new(creds);
        let result = fetch_bb::fetch_pull_requests(
            &client, &workspace, &slug,
        );
        let _ = sender.send(
            result.map(ToolFetchResult::Items),
        );
    });
}

impl App {
    pub(crate) fn start_tool_fetch(
        &mut self,
        tool: ToolKind,
        spawn: impl FnOnce(ToolTx),
    ) {
        self.picker.activate_tool_loading(tool);
        let (sender, recv) = mpsc::channel();
        self.tool_rx = Some(recv);
        spawn(sender);
    }

    /// Fetch tool data using a specific RepoEntry
    pub(crate) fn fetch_with_repo(
        &mut self,
        tool: ToolKind,
        repo: &RepoEntry,
    ) {
        let Some(bb_creds) = load_bb_creds() else {
            self.picker.set_tool_error(
                tool,
                tui_labels::ERR_NO_BB_CREDS.into(),
            );
            return;
        };
        let workspace = repo.workspace.clone();
        let slug = repo.repo_slug.clone();
        self.start_tool_fetch(tool, |sender| {
            match tool {
                ToolKind::Branch => spawn_bb_branches(
                    bb_creds, workspace, slug, sender,
                ),
                ToolKind::PullRequest => {
                    spawn_bb_prs_pub(
                        bb_creds, workspace, slug,
                        sender,
                    );
                }
                ToolKind::Jira | ToolKind::McpTool => {}
            }
        });
    }
}

fn load_bb_creds() -> Option<BbCredentials> {
    let root = std::path::Path::new(".");
    let creds =
        crate::service::atlassian::credentials
            ::load_credentials(root)?;
    creds.bitbucket
}
