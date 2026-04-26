//! Tests for Jira assignee filter state methods.

use rustean::domain::jira::{
    JiraBoardData, JiraTicketDetail,
};
use rustean::picker::mode::PickerMode;
use rustean::picker::Picker;

/// enter_assignee_filter switches mode
#[test]
fn enter_filter_switches_mode() {
    // Arrange
    let mut picker = Picker::default();
    picker.set_jira_board(make_board());

    // Act
    picker.enter_assignee_filter();

    // Assert
    assert_eq!(
        *picker.mode(),
        PickerMode::JiraAssigneeFilter,
    );
}

/// enter_assignee_filter clears query
#[test]
fn enter_filter_clears_query() {
    // Arrange
    let mut picker = Picker::default();
    picker.set_jira_board(make_board());
    picker.push_query('a');

    // Act
    picker.enter_assignee_filter();

    // Assert
    assert!(picker.query().is_empty());
}

/// set/get assignee filter round-trips
#[test]
fn set_get_assignee_filter() {
    // Arrange
    let mut picker = Picker::default();

    // Act
    picker.set_jira_assignee_filter(
        Some("Alice".into()),
    );

    // Assert
    assert_eq!(
        picker.jira_assignee_filter(),
        Some("Alice"),
    );
}

/// set_jira_assignee_filter None clears filter
#[test]
fn set_assignee_filter_none_clears() {
    // Arrange
    let mut picker = Picker::default();
    picker.set_jira_assignee_filter(
        Some("Bob".into()),
    );

    // Act
    picker.set_jira_assignee_filter(None);

    // Assert
    assert!(picker.jira_assignee_filter().is_none());
}

/// activate_assignee_picker stores names + mode
#[test]
fn activate_picker_stores_names_and_mode() {
    // Arrange
    let mut picker = Picker::default();
    let names =
        vec!["Alice".into(), "Bob".into()];

    // Act
    picker.activate_assignee_picker(names);

    // Assert
    assert_eq!(
        picker.jira_assignees(),
        &["Alice", "Bob"],
    );
    assert_eq!(
        *picker.mode(),
        PickerMode::JiraAssigneeFilter,
    );
}

/// Helper: minimal board with 2 assignees
fn make_board() -> JiraBoardData {
    JiraBoardData {
        sprint: None,
        tickets: vec![
            make_ticket("EVB-1", "Alice"),
            make_ticket("EVB-2", "Bob"),
        ],
        total_points: 0.0,
    }
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
