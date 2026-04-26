use crossterm::event::KeyCode;

use crate::app::App;
use crate::picker::PickerMode;

impl App {
    /// Route picker key events by current mode.
    ///
    /// Returns true if the app should quit.
    pub(crate) fn handle_picker_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        match self.picker.mode() {
            PickerMode::Inactive => false,
            PickerMode::Browse { .. } => {
                self.handle_browse_key(key)
            }
            PickerMode::Symbols { .. } => {
                self.handle_symbols_key(key)
            }
            PickerMode::Tools => {
                self.handle_tools_key(key)
            }
            _ => self.handle_picker_key_extra(key),
        }
    }

    /// Route tool and selection modes
    fn handle_picker_key_extra(
        &mut self,
        key: KeyCode,
    ) -> bool {
        match self.picker.mode() {
            PickerMode::ToolLoading { .. }
            | PickerMode::ToolResults { .. } => {
                self.handle_tool_results_key(key)
            }
            PickerMode::RepoSelect { .. } => {
                self.handle_repo_select_key(key)
            }
            PickerMode::McpToolBrowse => {
                self.handle_mcp_browse_key(key)
            }
            PickerMode::SlashCommand
            | PickerMode::SlashArg { .. } => {
                self.handle_slash_key(key)
            }
            _ => self.handle_picker_key_jira(key),
        }
    }

    /// Route Jira-specific picker modes
    fn handle_picker_key_jira(
        &mut self,
        key: KeyCode,
    ) -> bool {
        match self.picker.mode() {
            PickerMode::JiraAssigneeFilter => {
                self.handle_jira_filter_key(key)
            }
            PickerMode::JiraBoardSelect => {
                self.handle_board_select_key(key)
            }
            PickerMode::JiraTicketDetail => {
                self.handle_jira_detail_key(key)
            }
            PickerMode::GitRepoSelect => {
                self.handle_git_select_key(key)
            }
            PickerMode::GitLoading => {
                self.handle_git_loading_key(key)
            }
            PickerMode::GitHistory => {
                self.handle_git_history_key(key)
            }
            PickerMode::GitDetail => {
                self.handle_git_detail_key(key)
            }
            _ => false,
        }
    }
}
