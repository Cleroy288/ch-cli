//! Tests for ToDomain trait impls on Jira types.

use rustean::domain::jira::SprintState;
use rustean::domain::ToDomain;
use rustean::service::tools::jira_types::{
    JiraIssue, JiraSearchResult, JiraSprintRaw,
};

/// Helper: deserialize issues from JSON
fn issues_from_json(
    json: &str,
) -> Vec<JiraIssue> {
    let result: JiraSearchResult =
        serde_json::from_str(json).unwrap();
    result.issues
}

/// SprintRaw.to_domain maps all fields
#[test]
fn to_domain_sprint_maps_fields() {
    // Arrange
    let raw = JiraSprintRaw {
        name: Some("Sprint 1".into()),
        state: Some("active".into()),
        start_date: Some("2026-01-01".into()),
        end_date: Some("2026-01-14".into()),
    };

    // Act
    let sprint = raw.to_domain();

    // Assert
    assert_eq!(sprint.name, "Sprint 1");
    assert_eq!(sprint.state, SprintState::Active);
    assert_eq!(sprint.start_date, "2026-01-01");
    assert_eq!(sprint.end_date, "2026-01-14");
}

/// SprintRaw with None fields defaults cleanly
#[test]
fn to_domain_sprint_defaults_on_none() {
    // Arrange
    let raw = JiraSprintRaw {
        name: None,
        state: None,
        start_date: None,
        end_date: None,
    };

    // Act
    let sprint = raw.to_domain();

    // Assert
    assert_eq!(sprint.name, "");
    assert_eq!(sprint.state, SprintState::Future);
    assert_eq!(sprint.start_date, "");
}

/// Empty issues vec produces empty board
#[test]
fn to_domain_empty_issues_empty_board() {
    // Arrange
    let issues: Vec<JiraIssue> = vec![];

    // Act
    let board = issues.to_domain();

    // Assert
    assert!(board.tickets.is_empty());
    assert!(board.sprint.is_none());
    assert_eq!(board.total_points, 0.0);
}

/// Single issue maps via trait
#[test]
fn to_domain_issue_maps_ticket() {
    // Arrange
    let issues = issues_from_json(r#"{
        "issues": [{
            "key": "TST-7",
            "fields": {
                "summary": "Trait test",
                "status": {"name": "Done"},
                "customfield_10016": 2.0
            }
        }]
    }"#);

    // Act
    let ticket = issues[0].to_domain();

    // Assert
    assert_eq!(ticket.key, "TST-7");
    assert_eq!(ticket.summary, "Trait test");
    assert_eq!(ticket.story_points, Some(2.0));
}

/// Vec<JiraIssue>.to_domain sums story points
#[test]
fn to_domain_board_sums_points() {
    // Arrange
    let issues = issues_from_json(r#"{
        "issues": [
            {
                "key": "TST-1",
                "fields": {
                    "summary": "A",
                    "status": {"name": "Open"},
                    "customfield_10016": 3.0
                }
            },
            {
                "key": "TST-2",
                "fields": {
                    "summary": "B",
                    "status": {"name": "Open"},
                    "customfield_10016": 5.0
                }
            }
        ]
    }"#);

    // Act
    let board = issues.to_domain();

    // Assert
    assert_eq!(board.total_points, 8.0);
    assert_eq!(board.tickets.len(), 2);
}
