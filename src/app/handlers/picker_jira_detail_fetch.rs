use std::path::Path;

use crate::app::App;
use crate::domain::tool_ref::{
    ToolFetchResult, ToolKind,
};
use crate::service::atlassian::client_jira::JiraClient;
use crate::service::atlassian::types::JiraCredentials;
use crate::service::tools::fetch_jira_detail;

use super::picker_tools_fetch::ToolTx;
use super::picker_tools_fetch_ctx::load_jira_creds;

impl App {
    /// Spawn Jira detail fetch in background
    pub(crate) fn fetch_jira_detail_bg(
        &mut self,
        key: String,
    ) {
        let root = Path::new(&self.project_root);
        let Some(jira) = load_jira_creds(root)
        else {
            return;
        };
        self.start_tool_fetch(
            ToolKind::Jira,
            |sender| {
                spawn_jira_detail(
                    jira, key, sender,
                )
            },
        );
    }
}

/// Spawn detail fetch thread
fn spawn_jira_detail(
    jira: JiraCredentials,
    key: String,
    sender: ToolTx,
) {
    std::thread::spawn(move || {
        let client = JiraClient::new(jira);
        let result =
            fetch_jira_detail::fetch_issue_detail(
                &client, &key,
            );
        let _ = sender.send(
            result.map(ToolFetchResult::JiraDetail),
        );
    });
}
