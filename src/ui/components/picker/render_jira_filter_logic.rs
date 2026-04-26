use crate::domain::jira::JiraTicketDetail;

pub fn filter_tickets<'a>(
    tickets: &'a [JiraTicketDetail],
    query: &str,
    assignee: Option<&str>,
    project: Option<&str>,
) -> Vec<&'a JiraTicketDetail> {
    tickets
        .iter()
        .filter(|tkt| {
            matches_query(tkt, query)
                && matches_assignee(tkt, assignee)
                && matches_project(tkt, project)
        })
        .collect()
}

fn matches_query(
    tkt: &JiraTicketDetail,
    query: &str,
) -> bool {
    if query.is_empty() {
        return true;
    }
    tkt.key.to_lowercase().contains(query)
        || tkt
            .summary
            .to_lowercase()
            .contains(query)
}

fn matches_assignee(
    tkt: &JiraTicketDetail,
    filter: Option<&str>,
) -> bool {
    let Some(name) = filter else {
        return true;
    };
    if tkt.assignee.is_empty() {
        name == "Unassigned"
    } else {
        tkt.assignee == name
    }
}

fn matches_project(
    tkt: &JiraTicketDetail,
    project: Option<&str>,
) -> bool {
    let Some(key) = project else {
        return true;
    };
    tkt.project_key == key
}
