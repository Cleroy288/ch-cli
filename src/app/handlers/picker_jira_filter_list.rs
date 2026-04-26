use crate::app::App;

impl App {
    pub(crate) fn assignee_list_count(
        &self,
    ) -> usize {
        self.filtered_assignee_list().len()
    }

    ///
    /// Returns unique assignees sorted alpha,
    /// filtered by the current query.
    pub(crate) fn filtered_assignee_list(
        &self,
    ) -> Vec<String> {
        let query =
            self.picker.query().to_lowercase();
        let list: Vec<String> = self
            .picker
            .jira_assignees()
            .to_vec();
        if query.is_empty() {
            return list;
        }
        list.into_iter()
            .filter(|name| {
                name.to_lowercase()
                    .contains(&query)
            })
            .collect()
    }
}
