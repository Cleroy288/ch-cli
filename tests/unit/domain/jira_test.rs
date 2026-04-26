//! Tests for Jira domain types.

use rustean::domain::jira::{
    JiraBoardData, JiraSprint,
    JiraTicketDetail, SprintState,
};

/// JiraBoardData default has no sprint
#[test]
fn board_data_default_has_no_sprint() {
    // Arrange & Act
    let board = JiraBoardData::default();

    // Assert
    assert!(board.sprint.is_none());
    assert!(board.tickets.is_empty());
    assert_eq!(board.total_points, 0.0);
}

/// JiraSprint default has empty fields
#[test]
fn sprint_default_has_empty_fields() {
    // Arrange & Act
    let sprint = JiraSprint::default();

    // Assert
    assert!(sprint.name.is_empty());
    assert_eq!(
        sprint.state,
        SprintState::Future,
    );
}

/// Board data stores sprint and tickets
#[test]
fn board_data_stores_sprint_and_tickets() {
    // Arrange
    let sprint = JiraSprint {
        name: "Sprint 1".into(),
        state: SprintState::Active,
        start_date: "2026-02-01".into(),
        end_date: "2026-02-14".into(),
    };
    let ticket = JiraTicketDetail {
        key: "EVB-1".into(),
        summary: "Test".into(),
        status: "Open".into(),
        status_category: "new".into(),
        priority: "High".into(),
        issue_type: "Bug".into(),
        assignee: "Alice".into(),
        story_points: Some(3.0),
        sprint_name: "Sprint 1".into(),
        project_key: "EVB".into(),
    };

    // Act
    let board = JiraBoardData {
        sprint: Some(sprint),
        tickets: vec![ticket],
        total_points: 3.0,
    };

    // Assert
    assert!(board.sprint.is_some());
    assert_eq!(board.tickets.len(), 1);
    assert_eq!(board.total_points, 3.0);
}

/// ToolFetchResult JiraBoard variant
#[test]
fn fetch_result_jira_board_variant() {
    use rustean::domain::tool_ref::ToolFetchResult;

    // Arrange
    let board = JiraBoardData::default();

    // Act
    let result =
        ToolFetchResult::JiraBoard(board);

    // Assert
    assert!(matches!(
        result,
        ToolFetchResult::JiraBoard(_)
    ));
}

/// ToolFetchResult Items variant
#[test]
fn fetch_result_items_variant() {
    use rustean::domain::tool_ref::{
        ToolFetchResult, ToolItem,
    };

    // Arrange
    let items = vec![ToolItem {
        key: "k".into(),
        display: "d".into(),
        description: "desc".into(),
    }];

    // Act
    let result = ToolFetchResult::Items(items);

    // Assert
    assert!(matches!(
        result,
        ToolFetchResult::Items(_)
    ));
}
