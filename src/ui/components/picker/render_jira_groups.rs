use std::collections::HashMap;

use crate::domain::jira::JiraTicketDetail;

use super::render_jira_sort::sprint_sort_key;

const NO_SPRINT: &str = "No Sprint";

pub enum BoardRow<'a> {
    ProjectHeader(String),
    SprintHeader(String),
    Ticket(&'a JiraTicketDetail),
}

type NamedGroup<'a> =
    (String, Vec<&'a JiraTicketDetail>);

pub fn group_by_project_sprint<'a>(
    tickets: Vec<&'a JiraTicketDetail>,
) -> Vec<BoardRow<'a>> {
    let mut projects =
        collect_by_field(tickets, |tkt| {
            tkt.project_key.clone()
        });
    projects.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
    let mut rows = Vec::new();
    for (key, tkts) in projects {
        rows.push(BoardRow::ProjectHeader(key));
        append_sprint_rows(&mut rows, tkts);
    }
    rows
}

/// Used when a single project is selected.
pub fn group_by_sprint<'a>(
    tickets: Vec<&'a JiraTicketDetail>,
) -> Vec<BoardRow<'a>> {
    let mut rows = Vec::new();
    append_sprint_rows(&mut rows, tickets);
    rows
}

pub fn ticket_at_index<'a>(
    rows: &[BoardRow<'a>],
    idx: usize,
) -> Option<&'a JiraTicketDetail> {
    match rows.get(idx) {
        Some(BoardRow::Ticket(tkt)) => Some(tkt),
        _ => None,
    }
}

fn collect_by_field<'a, F>(
    tickets: Vec<&'a JiraTicketDetail>,
    key_fn: F,
) -> Vec<NamedGroup<'a>>
where
    F: Fn(&JiraTicketDetail) -> String,
{
    let mut index: HashMap<String, usize> =
        HashMap::new();
    let mut groups: Vec<NamedGroup<'a>> = Vec::new();
    for tkt in tickets {
        let label = key_fn(tkt);
        if let Some(&idx) = index.get(&label) {
            groups[idx].1.push(tkt);
        } else {
            index.insert(label.clone(), groups.len());
            groups.push((label, vec![tkt]));
        }
    }
    groups
}

fn append_sprint_rows<'a>(
    rows: &mut Vec<BoardRow<'a>>,
    tickets: Vec<&'a JiraTicketDetail>,
) {
    let mut sprints =
        collect_by_field(tickets, |tkt| {
            if tkt.sprint_name.is_empty() {
                NO_SPRINT.to_string()
            } else {
                tkt.sprint_name.clone()
            }
        });
    sprints.sort_by_key(|(name, _)| {
        sprint_sort_key(name)
    });
    for (name, tkts) in sprints {
        rows.push(BoardRow::SprintHeader(name));
        for tkt in tkts {
            rows.push(BoardRow::Ticket(tkt));
        }
    }
}
