use std::path::Path;

use crate::app::App;
use crate::domain::tool_ref::{
    ToolFetchResult, ToolKind,
};
use crate::service::atlassian::client_jira::JiraClient;
use crate::service::atlassian::types::JiraCredentials;
use crate::service::tools::fetch_jira;
use crate::ui::strings::tui_labels;

use super::picker_tools_fetch::ToolTx;
use super::picker_tools_fetch_ctx::load_jira_creds;

impl App {
    /// Spawn Jira ticket fetch in background.
    ///
    /// Uses JQL with auto-discovered project keys
    /// and the current assignee filter.
    pub(crate) fn fetch_jira_bg(&mut self) {
        let root = Path::new(&self.project_root);
        let Some(jira) = load_jira_creds(root)
        else {
            self.picker.set_tool_error(
                ToolKind::Jira,
                tui_labels::ERR_NO_JIRA_CREDS.into(),
            );
            return;
        };
        let assignee = self
            .picker
            .jira_assignee_filter()
            .map(|val| val.to_string());
        let root = self.project_root.clone();
        self.start_tool_fetch(
            ToolKind::Jira,
            |sender| {
                spawn_jira(
                    jira, root, assignee, sender,
                )
            },
        );
    }
}

/// Spawn JQL fetch thread
fn spawn_jira(
    jira: JiraCredentials,
    root: String,
    assignee: Option<String>,
    sender: ToolTx,
) {
    std::thread::spawn(move || {
        let key = jira.project_key.clone();
        let client = JiraClient::new(jira);
        let root_path = Path::new(&root);
        let result = fetch_jira::fetch_jira_board(
            &client,
            key.as_deref(),
            root_path,
            assignee.as_deref(),
        );
        let _ = sender.send(
            result.map(ToolFetchResult::JiraBoard),
        );
    });
}
