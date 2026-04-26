use crate::domain::jira_detail::JiraIssueDetail;
use crate::picker::mode::PickerMode;

use super::state::Picker;

impl Picker {
    /// Store detail and switch to detail mode
    pub fn set_jira_detail(
        &mut self,
        detail: JiraIssueDetail,
    ) {
        self.jira_detail = Some(detail);
        self.jira_detail_scroll = 0;
        self.mode = PickerMode::JiraTicketDetail;
    }

    /// Current detail (if viewing)
    pub fn jira_detail(
        &self,
    ) -> Option<&JiraIssueDetail> {
        self.jira_detail.as_ref()
    }

    /// Current scroll offset in detail view
    pub fn jira_detail_scroll(&self) -> usize {
        self.jira_detail_scroll
    }

    /// Scroll detail view up by one line
    pub fn scroll_detail_up(&mut self) {
        self.jira_detail_scroll =
            self.jira_detail_scroll.saturating_sub(1);
    }

    /// Scroll detail view down by one line
    pub fn scroll_detail_down(&mut self) {
        self.jira_detail_scroll += 1;
    }

    pub fn clear_jira_detail(&mut self) {
        self.jira_detail = None;
        self.jira_detail_scroll = 0;
    }
}
