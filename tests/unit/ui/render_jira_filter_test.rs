//! Tests for Jira ticket filter logic.

use rustean::domain::jira::JiraTicketDetail;
use rustean::ui::components::picker::{
    render_jira_filter_logic::filter_tickets,
};

/// No filter returns all tickets
#[test]
fn filter_no_query_no_assignee_returns_all() {
    // Arrange
    let tickets = make_tickets();

    // Act
    let result =
        filter_tickets(&tickets, "", None, None);

    // Assert
    assert_eq!(result.len(), 3);
}

/// Text query filters by key
#[test]
fn filter_by_key_query() {
    // Arrange
    let tickets = make_tickets();

    // Act
    let result =
        filter_tickets(
            &tickets, "evb-1", None, None,
        );

    // Assert
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, "EVB-1");
}

/// Assignee filter shows only matching
#[test]
fn filter_by_assignee_alice() {
    // Arrange
    let tickets = make_tickets();

    // Act
    let result = filter_tickets(
        &tickets,
        "",
        Some("Alice"),
        None,
    );

    // Assert
    assert_eq!(result.len(), 2);
    assert!(
        result.iter().all(|t| t.assignee == "Alice")
    );
}

/// Combined query + assignee filter
#[test]
fn filter_combined_query_and_assignee() {
    // Arrange
    let tickets = make_tickets();

    // Act
    let result = filter_tickets(
        &tickets,
        "evb-1",
        Some("Alice"),
        None,
    );

    // Assert
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, "EVB-1");
}

/// Project filter limits to matching key
#[test]
fn filter_by_project_key() {
    // Arrange
    let tickets = vec![
        make_ticket("EVB-1", "Alice"),
        make_ticket("CC-1", "Alice"),
    ];

    // Act
    let result = filter_tickets(
        &tickets, "", None, Some("CC"),
    );

    // Assert
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, "CC-1");
}

/// Helper: 3 tickets, 2 Alice + 1 Bob
fn make_tickets() -> Vec<JiraTicketDetail> {
    vec![
        make_ticket("EVB-1", "Alice"),
        make_ticket("EVB-2", "Bob"),
        make_ticket("EVB-3", "Alice"),
    ]
}

/// Helper: make ticket with assignee
fn make_ticket(
    key: &str,
    assignee: &str,
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
        assignee: assignee.into(),
        story_points: None,
        sprint_name: String::new(),
        project_key: project.into(),
    }
}
