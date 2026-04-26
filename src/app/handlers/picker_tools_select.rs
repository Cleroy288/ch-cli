use std::path::Path;

use crate::app::App;
use crate::domain::tool_ref::ToolKind;
use crate::picker::tool_items;
use crate::service::tools::repo_cache;
use crate::ui::strings::tui_labels;

use super::picker_tools_fetch::{
    spawn_bb_branches, spawn_bb_prs_pub,
};
use super::picker_tools_fetch_ctx::load_bb_context;

impl App {
    /// Select a tool and trigger background fetch
    pub(crate) fn select_tool_item(&mut self) {
        let query = self.picker.query().to_string();
        let items = tool_items::filter_tools(&query);
        let idx = self.picker.selected_index();
        let Some(item) = items.get(idx) else {
            return;
        };
        match item.kind {
            ToolKind::Branch => {
                self.fetch_bb_tool(ToolKind::Branch)
            }
            ToolKind::PullRequest => {
                self.fetch_bb_tool(
                    ToolKind::PullRequest,
                )
            }
            ToolKind::Jira => {
                self.fetch_jira_assignees_bg()
            }
            ToolKind::McpTool => {
                self.picker.activate_mcp_browse()
            }
        }
    }

    /// Fetch a BB tool with repo resolution
    fn fetch_bb_tool(&mut self, tool: ToolKind) {
        let root = Path::new(".");
        if let Some(ctx) = load_bb_context(root) {
            let (creds, workspace, slug) = ctx;
            self.start_tool_fetch(tool, |sender| {
                dispatch_bb_spawn(
                    tool, creds, workspace,
                    slug, sender,
                );
            });
            return;
        }
        self.try_cached_repos(tool);
    }

    /// Try cached repos for repo selection
    fn try_cached_repos(&mut self, tool: ToolKind) {
        let root = Path::new(".");
        let Some(cache) =
            repo_cache::load_repos(root)
        else {
            self.picker.set_tool_error(
                tool,
                tui_labels::ERR_NO_BB_REPO.into(),
            );
            return;
        };
        let repos = cache.repos;
        if repos.len() == 1 {
            self.fetch_with_repo(tool, &repos[0]);
        } else {
            self.picker.activate_repo_select(
                tool, repos,
            );
        }
    }
}

fn dispatch_bb_spawn(
    tool: ToolKind,
    creds: crate::service::atlassian::types
        ::BbCredentials,
    workspace: String,
    slug: String,
    sender: super::picker_tools_fetch::ToolTx,
) {
    match tool {
        ToolKind::Branch => spawn_bb_branches(
            creds, workspace, slug, sender,
        ),
        ToolKind::PullRequest => spawn_bb_prs_pub(
            creds, workspace, slug, sender,
        ),
        ToolKind::Jira | ToolKind::McpTool => {}
    }
}
