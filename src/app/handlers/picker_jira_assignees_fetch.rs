use std::path::Path;

use crate::app::App;
use crate::domain::tool_ref::{
    ToolFetchResult, ToolKind,
};
use crate::service::atlassian::client_jira::JiraClient;
use crate::service::atlassian::types::JiraCredentials;
use crate::service::tools::fetch_jira_assignees;
use crate::ui::strings::tui_labels;

use super::picker_tools_fetch::ToolTx;
use super::picker_tools_fetch_ctx::load_jira_creds;

impl App {
    /// Spawn Jira assignee discovery in background.
    ///
    /// Shows loading state, then transitions to
    /// JiraAssigneeFilter when names arrive.
    pub(crate) fn fetch_jira_assignees_bg(
        &mut self,
    ) {
        let root = Path::new(&self.project_root);
        let Some(jira) = load_jira_creds(root)
        else {
            self.picker.set_tool_error(
                ToolKind::Jira,
                tui_labels::ERR_NO_JIRA_CREDS.into(),
            );
            return;
        };
        let root = self.project_root.clone();
        self.start_tool_fetch(
            ToolKind::Jira,
            |sender| {
                spawn_assignees(jira, root, sender)
            },
        );
    }
}

/// Spawn assignee discovery thread
fn spawn_assignees(
    jira: JiraCredentials,
    root: String,
    sender: ToolTx,
) {
    std::thread::spawn(move || {
        let key = jira.project_key.clone();
        let client = JiraClient::new(jira);
        let root_path = Path::new(&root);
        let result =
            fetch_jira_assignees::fetch_jira_assignees(
                &client, key.as_deref(), root_path,
            );
        let _ = sender.send(
            result
                .map(ToolFetchResult::JiraAssignees),
        );
    });
}
