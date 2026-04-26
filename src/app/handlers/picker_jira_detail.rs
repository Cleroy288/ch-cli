use crossterm::event::KeyCode;

use crate::app::App;
use crate::domain::tool_ref::{
    ToolItem, ToolKind,
};

impl App {
    /// Handle keys in JiraTicketDetail mode
    pub(crate) fn handle_jira_detail_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        match key {
            KeyCode::Up => {
                self.picker.scroll_detail_up();
            }
            KeyCode::Down => {
                self.picker.scroll_detail_down();
            }
            KeyCode::Enter => {
                self.insert_jira_from_detail();
            }
            KeyCode::Esc | KeyCode::Backspace => {
                self.back_from_jira_detail();
            }
            _ => {}
        }
        false
    }

    /// Insert #JIRA reference from detail view
    fn insert_jira_from_detail(&mut self) {
        let Some(detail) =
            self.picker.jira_detail()
        else {
            return;
        };
        let item = ToolItem {
            key: detail.key.clone(),
            display: format!(
                "{} - {}",
                detail.key, detail.summary,
            ),
            description: detail.status.clone(),
        };
        self.picker.clear_jira_detail();
        self.insert_tool_reference(
            ToolKind::Jira, &item,
        );
    }

    /// Return to Jira board from detail view
    fn back_from_jira_detail(&mut self) {
        self.picker.clear_jira_detail();
        self.picker
            .enter_tool_results(ToolKind::Jira);
    }
}
