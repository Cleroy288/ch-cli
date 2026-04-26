use crate::app::App;
use crate::domain::tool_ref::{ToolItem, ToolKind};
use crate::picker::mode::PickerMode;

impl App {
    pub(crate) fn filter_tool_results(
        &self,
        query: &str,
    ) -> Vec<ToolItem> {
        if query.is_empty() {
            return self
                .picker
                .tool_results
                .clone();
        }
        self.picker
            .tool_results
            .iter()
            .filter(|item| {
                item.display
                    .to_lowercase()
                    .contains(query)
            })
            .cloned()
            .collect()
    }

    pub fn tool_results(
        &self,
    ) -> &[ToolItem] {
        &self.picker.tool_results
    }

    pub(crate) fn is_jira_board_active(
        &self,
    ) -> bool {
        matches!(
            self.picker.mode(),
            PickerMode::ToolResults { tool }
                if *tool == ToolKind::Jira
        ) && self.picker.jira_board().is_some()
    }

    /// Select a tool result and insert reference.
    ///
    /// On Jira board, index is offset by 1 for
    /// the filter action row at position 0.
    pub(crate) fn select_tool_result(
        &mut self,
    ) {
        let tool = match self.picker.mode() {
            PickerMode::ToolResults { tool } => {
                *tool
            }
            _ => return,
        };
        let query =
            self.picker.query().to_lowercase();
        let filtered =
            self.filter_tool_results(&query);
        let raw = self.picker.selected_index();
        let idx = if self.is_jira_board_active() {
            raw.saturating_sub(1)
        } else {
            raw
        };
        let Some(item) = filtered.get(idx) else {
            return;
        };
        let item = item.clone();
        self.insert_tool_reference(tool, &item);
    }
}
