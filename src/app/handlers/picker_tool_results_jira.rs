use crate::app::App;
use crate::domain::jira::JiraTicketDetail;
use crate::ui::components::picker::{
    render_jira_filter_logic::filter_tickets,
    render_jira_groups::{
        group_by_project_sprint, group_by_sprint,
        ticket_at_index, BoardRow,
    },
};

impl App {
    ///
    /// Returns None for sprint header rows.
    pub(crate) fn selected_jira_key(
        &self,
    ) -> Option<String> {
        let board = self.picker.jira_board()?;
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
        let raw = self.picker.selected_index();
        // Index 0 is the filter row
        let idx = raw.saturating_sub(1);
        ticket_at_index(&rows, idx)
            .map(|tkt| tkt.key.clone())
    }

    pub(crate) fn is_jira_filter_row(
        &self,
    ) -> bool {
        self.is_jira_board_active()
            && self.picker.selected_index() == 0
    }

    ///
    /// For Jira, filters from board tickets using
    /// text query, assignee, and project filter.
    pub(crate) fn filtered_results_count(
        &self,
    ) -> usize {
        if self.is_jira_board_active() {
            return self.jira_filtered_count();
        }
        self.generic_filtered_count()
    }
}

pub(super) fn build_jira_rows<'a>(
    filtered: Vec<&'a JiraTicketDetail>,
) -> Vec<BoardRow<'a>> {
    let has_sprint = filtered
        .iter()
        .any(|tkt| !tkt.sprint_name.is_empty());
    if has_sprint {
        group_by_sprint(filtered)
    } else {
        group_by_project_sprint(filtered)
    }
}
