use crate::app::App;
use crate::ui::components::picker
    ::render_jira_filter_logic::filter_tickets;
use super::picker_tool_results_jira::build_jira_rows;

impl App {
    pub(crate) fn jira_filtered_count(
        &self,
    ) -> usize {
        let Some(board) =
            self.picker.jira_board()
        else {
            return 0;
        };
        let query =
            self.picker.query().to_lowercase();
        let filter =
            self.picker.jira_assignee_filter();
        let project =
            self.picker.jira_selected_project();
        let filtered = filter_tickets(
            &board.tickets,
            &query,
            filter,
            project,
        );
        let rows = build_jira_rows(filtered);
        // +1 for the filter action row at index 0
        rows.len() + 1
    }

    pub(crate) fn generic_filtered_count(
        &self,
    ) -> usize {
        let query =
            self.picker.query().to_lowercase();
        if query.is_empty() {
            return self
                .picker
                .tool_results
                .len();
        }
        self.picker
            .tool_results
            .iter()
            .filter(|item| {
                item.display
                    .to_lowercase()
                    .contains(&query)
            })
            .count()
    }
}
