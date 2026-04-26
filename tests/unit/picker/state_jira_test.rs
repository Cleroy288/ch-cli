//! Tests for Jira picker state methods.

use rustean::domain::jira::{
    JiraBoardData, JiraSprint,
    JiraTicketDetail, SprintState,
};
use rustean::picker::Picker;

/// set_jira_board stores data
#[test]
fn set_jira_board_stores_data() {
    // Arrange
    let mut picker = Picker::default();
    let board = make_test_board();

    // Act
    picker.set_jira_board(board);

    // Assert
    assert!(picker.jira_board().is_some());
    let b = picker.jira_board().unwrap();
    assert_eq!(b.tickets.len(), 2);
}

/// clear_jira_board removes data
#[test]
fn clear_jira_board_removes_data() {
    // Arrange
    let mut picker = Picker::default();
    picker.set_jira_board(make_test_board());

    // Act
    picker.clear_jira_board();

    // Assert
    assert!(picker.jira_board().is_none());
    assert!(picker.jira_assignees().is_empty());
    assert!(picker.jira_assignee_filter().is_none());
}

/// jira_board returns None by default
#[test]
fn jira_board_none_by_default() {
    // Arrange & Act
    let picker = Picker::default();

    // Assert
    assert!(picker.jira_board().is_none());
}

/// set_jira_board extracts unique sorted assignees
#[test]
fn set_jira_board_extracts_assignees() {
    // Arrange
    let mut picker = Picker::default();
    let board = make_team_board();

    // Act
    picker.set_jira_board(board);

    // Assert
    let names = picker.jira_assignees();
    assert_eq!(names, &["Alice", "Bob"]);
}

/// set_jira_board deduplicates assignees
#[test]
fn set_jira_board_deduplicates_assignees() {
    // Arrange
    let mut picker = Picker::default();
    let board = make_test_board();

    // Act
    picker.set_jira_board(board);

    // Assert — Alice appears twice in tickets
    let names = picker.jira_assignees();
    assert_eq!(
        names.iter().filter(|n| *n == "Alice").count(),
        1,
    );
}

/// Single project auto-selects into ToolResults
#[test]
fn single_project_auto_selects() {
    // Arrange
    use rustean::picker::PickerMode;
    use rustean::domain::tool_ref::ToolKind;
    let mut picker = Picker::default();
    let board = make_test_board();

    // Act
    picker.set_jira_board(board);

    // Assert
    assert_eq!(
        *picker.mode(),
        PickerMode::ToolResults {
            tool: ToolKind::Jira,
        },
    );
    assert_eq!(
        picker.jira_selected_project(),
        Some("EVB"),
    );
}

/// Multiple projects enter space select mode
#[test]
fn multi_project_enters_space_select() {
    // Arrange
    use rustean::picker::PickerMode;
    let mut picker = Picker::default();
    let board = make_multi_project_board();

    // Act
    picker.set_jira_board(board);

    // Assert
    assert_eq!(
        *picker.mode(),
        PickerMode::JiraBoardSelect,
    );
    assert_eq!(
        picker.jira_project_keys(),
        &["CC", "EVB"],
    );
}

/// Helper: board with one ticket
fn make_test_board() -> JiraBoardData {
    JiraBoardData {
        sprint: Some(JiraSprint {
            name: "Sprint 1".into(),
            state: SprintState::Active,
            start_date: "2026-02-01".into(),
            end_date: "2026-02-14".into(),
        }),
        tickets: vec![
            make_ticket("EVB-1", "Alice"),
            make_ticket("EVB-2", "Alice"),
        ],
        total_points: 3.0,
    }
}

/// Helper: board with multiple assignees
fn make_team_board() -> JiraBoardData {
    JiraBoardData {
        sprint: None,
        tickets: vec![
            make_ticket("EVB-1", "Alice"),
            make_ticket("EVB-2", "Bob"),
            make_ticket("EVB-3", "Alice"),
        ],
        total_points: 0.0,
    }
}

/// Helper: board with tickets from CC + EVB
fn make_multi_project_board() -> JiraBoardData {
    JiraBoardData {
        sprint: None,
        tickets: vec![
            make_ticket("EVB-1", "Alice"),
            make_ticket("CC-1", "Bob"),
        ],
        total_points: 0.0,
    }
}

/// Helper: make a ticket with given assignee
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
        summary: "Test ticket".into(),
        status: "Open".into(),
        status_category: "new".into(),
        priority: "High".into(),
        issue_type: "Bug".into(),
        assignee: assignee.into(),
        story_points: Some(3.0),
        sprint_name: String::new(),
        project_key: project.into(),
    }
}
