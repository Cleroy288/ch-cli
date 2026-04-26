//! Tests for Jira project + sprint grouping logic.

use rustean::domain::jira::JiraTicketDetail;
use rustean::ui::components::picker::{
    render_jira_groups::{
        group_by_project_sprint, group_by_sprint,
        ticket_at_index, BoardRow,
    },
};

/// Single project inserts project header first
#[test]
fn single_project_has_project_header() {
    // Arrange
    let sprint = "Sprint 2 - Q1 2026";
    let tickets = vec![
        make_ticket("EVB-1", sprint),
        make_ticket("EVB-2", sprint),
    ];
    let refs: Vec<&JiraTicketDetail> =
        tickets.iter().collect();

    // Act
    let rows = group_by_project_sprint(refs);

    // Assert — project + sprint + 2 tickets
    assert_eq!(rows.len(), 4);
    assert!(matches!(
        &rows[0],
        BoardRow::ProjectHeader(k) if k == "EVB"
    ));
    assert!(matches!(
        &rows[1],
        BoardRow::SprintHeader(n) if n == sprint
    ));
}

/// Sprints sorted within a project
#[test]
fn sprints_sorted_within_project() {
    // Arrange — inserted in wrong order
    let tickets = vec![
        make_ticket("EVB-1", "Bridge 3 - Q1 2026"),
        make_ticket("EVB-2", "Sprint 2 - Q1 2026"),
        make_ticket("EVB-3", "Sprint 1 - Q2 2026"),
    ];
    let refs: Vec<&JiraTicketDetail> =
        tickets.iter().collect();

    // Act
    let rows = group_by_project_sprint(refs);

    // Assert — EVB + sorted sprints
    assert_eq!(rows.len(), 7);
    assert!(matches!(
        &rows[1],
        BoardRow::SprintHeader(n)
            if n == "Sprint 2 - Q1 2026"
    ));
    assert!(matches!(
        &rows[3],
        BoardRow::SprintHeader(n)
            if n == "Bridge 3 - Q1 2026"
    ));
    assert!(matches!(
        &rows[5],
        BoardRow::SprintHeader(n)
            if n == "Sprint 1 - Q2 2026"
    ));
}

/// Empty sprint becomes "No Sprint" at end
#[test]
fn empty_sprint_sorted_last() {
    // Arrange
    let tickets = vec![
        make_ticket("EVB-1", ""),
        make_ticket("EVB-2", "Sprint 1 - Q1 2026"),
    ];
    let refs: Vec<&JiraTicketDetail> =
        tickets.iter().collect();

    // Act
    let rows = group_by_project_sprint(refs);

    // Assert — project + sprint first, No Sprint last
    assert_eq!(rows.len(), 5);
    assert!(matches!(
        &rows[1],
        BoardRow::SprintHeader(n)
            if n == "Sprint 1 - Q1 2026"
    ));
    assert!(matches!(
        &rows[3],
        BoardRow::SprintHeader(n)
            if n == "No Sprint"
    ));
}

/// Multiple projects get separate headers sorted
#[test]
fn multi_project_separate_headers() {
    // Arrange
    let tickets = vec![
        make_ticket("EVB-1", "Sprint 1"),
        make_ticket("CC-1", "Sprint 1"),
    ];
    let refs: Vec<&JiraTicketDetail> =
        tickets.iter().collect();

    // Act
    let rows = group_by_project_sprint(refs);

    // Assert — CC first (alphabetical), then EVB
    assert!(matches!(
        &rows[0],
        BoardRow::ProjectHeader(k) if k == "CC"
    ));
    assert!(matches!(
        &rows[3],
        BoardRow::ProjectHeader(k) if k == "EVB"
    ));
}

/// ticket_at_index returns ticket, None for headers
#[test]
fn ticket_at_index_returns_ticket() {
    // Arrange
    let tickets = vec![
        make_ticket("EVB-1", "Sprint 1 - Q1 2026"),
    ];
    let refs: Vec<&JiraTicketDetail> =
        tickets.iter().collect();
    let rows = group_by_project_sprint(refs);

    // Act — 0=project, 1=sprint, 2=ticket
    let ticket = ticket_at_index(&rows, 2);
    let header = ticket_at_index(&rows, 0);

    // Assert
    assert_eq!(ticket.unwrap().key, "EVB-1");
    assert!(header.is_none());
}

/// group_by_sprint has no project headers
#[test]
fn sprint_only_no_project_headers() {
    // Arrange
    let tickets = vec![
        make_ticket("EVB-1", "Sprint 1 - Q1 2026"),
        make_ticket("EVB-2", "Sprint 2 - Q1 2026"),
    ];
    let refs: Vec<&JiraTicketDetail> =
        tickets.iter().collect();

    // Act
    let rows = group_by_sprint(refs);

    // Assert — sprint header + ticket each, no project
    assert_eq!(rows.len(), 4);
    assert!(matches!(
        &rows[0],
        BoardRow::SprintHeader(_)
    ));
    assert!(!rows.iter().any(|row| matches!(
        row,
        BoardRow::ProjectHeader(_)
    )));
}

/// Helper: make ticket with sprint name
fn make_ticket(
    key: &str,
    sprint: &str,
) -> JiraTicketDetail {
    let project = key
        .split('-')
        .next()
        .unwrap_or(key);
    JiraTicketDetail {
        key: key.into(),
        summary: "Test".into(),
        status: "Open".into(),
        status_category: "new".into(),
        priority: "Medium".into(),
        issue_type: "Story".into(),
        assignee: "Alice".into(),
        story_points: None,
        sprint_name: sprint.into(),
        project_key: project.into(),
    }
}
