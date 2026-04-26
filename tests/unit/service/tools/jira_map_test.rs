//! Tests for Jira issue → domain type mapping.

use rustean::domain::jira::SprintState;
use rustean::service::tools::jira_map::map_board;
use rustean::service::tools::jira_types::{
    JiraIssue, JiraSearchResult,
};

/// Helper: deserialize issues from JSON
fn issues_from_json(
    json: &str,
) -> Vec<JiraIssue> {
    let result: JiraSearchResult =
        serde_json::from_str(json).unwrap();
    result.issues
}

/// Empty issues vec produces empty board
#[test]
fn map_board_empty_issues_empty_board() {
    // Arrange
    let issues = vec![];

    // Act
    let board = map_board(issues);

    // Assert
    assert!(board.tickets.is_empty());
    assert!(board.sprint.is_none());
    assert_eq!(board.total_points, 0.0);
}

/// Issues without sprint leaves board.sprint None
#[test]
fn map_board_no_sprint_is_none() {
    // Arrange
    let issues = issues_from_json(r#"{
        "issues": [{
            "key": "EVB-1",
            "fields": {
                "summary": "Task A",
                "status": {"name": "Open"}
            }
        }]
    }"#);

    // Act
    let board = map_board(issues);

    // Assert
    assert!(board.sprint.is_none());
    assert_eq!(board.tickets.len(), 1);
}

/// Active sprint extracted from ticket data
#[test]
fn map_board_extracts_active_sprint() {
    // Arrange
    let issues = issues_from_json(r#"{
        "issues": [{
            "key": "EVB-1",
            "fields": {
                "summary": "Task A",
                "status": {"name": "Open"},
                "customfield_10021": [{
                    "name": "Sprint 5",
                    "state": "active",
                    "startDate": "2026-02-01",
                    "endDate": "2026-02-14"
                }]
            }
        }]
    }"#);

    // Act
    let board = map_board(issues);

    // Assert
    let sprint = board.sprint.unwrap();
    assert_eq!(sprint.name, "Sprint 5");
    assert_eq!(sprint.state, SprintState::Active);
}

/// Story points accumulated across tickets
#[test]
fn map_board_story_points_summed() {
    // Arrange
    let issues = issues_from_json(r#"{
        "issues": [
            {
                "key": "EVB-1",
                "fields": {
                    "summary": "A",
                    "status": {"name": "Open"},
                    "customfield_10016": 3.0
                }
            },
            {
                "key": "EVB-2",
                "fields": {
                    "summary": "B",
                    "status": {"name": "Open"},
                    "customfield_10016": 5.0
                }
            }
        ]
    }"#);

    // Act
    let board = map_board(issues);

    // Assert
    assert_eq!(board.total_points, 8.0);
    assert_eq!(board.tickets.len(), 2);
}

/// Sprint name populated on each ticket
#[test]
fn map_board_ticket_has_sprint_name() {
    // Arrange
    let issues = issues_from_json(r#"{
        "issues": [{
            "key": "EVB-1",
            "fields": {
                "summary": "Task A",
                "status": {"name": "Open"},
                "customfield_10021": [{
                    "name": "Sprint 5",
                    "state": "active"
                }]
            }
        }]
    }"#);

    // Act
    let board = map_board(issues);

    // Assert
    assert_eq!(
        board.tickets[0].sprint_name,
        "Sprint 5",
    );
}

/// Ticket fields mapped correctly to domain type
#[test]
fn map_board_ticket_fields_mapped() {
    // Arrange
    let issues = issues_from_json(r#"{
        "issues": [{
            "key": "EVB-42",
            "fields": {
                "summary": "Add login",
                "status": {
                    "name": "In Progress",
                    "statusCategory": {
                        "name": "In Progress",
                        "key": "indeterminate"
                    }
                },
                "priority": {"name": "High"},
                "issuetype": {"name": "Story"},
                "assignee": {
                    "displayName": "Bob"
                }
            }
        }]
    }"#);

    // Act
    let board = map_board(issues);

    // Assert
    let tkt = &board.tickets[0];
    assert_eq!(tkt.key, "EVB-42");
    assert_eq!(tkt.summary, "Add login");
    assert_eq!(tkt.status, "In Progress");
    assert_eq!(
        tkt.status_category, "indeterminate",
    );
    assert_eq!(tkt.priority, "High");
    assert_eq!(tkt.issue_type, "Story");
    assert_eq!(tkt.assignee, "Bob");
}
