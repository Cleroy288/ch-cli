//! Tests for Jira project key picker state.

use rustean::picker::Picker;
use rustean::picker::PickerMode;

/// enter_space_select stores keys
#[test]
fn enter_space_select_stores_keys() {
    // Arrange
    let mut picker = Picker::default();
    let keys = vec!["CC".into(), "EVB".into()];

    // Act
    picker.enter_space_select(keys);

    // Assert
    assert_eq!(picker.jira_project_keys().len(), 2);
    assert_eq!(picker.jira_project_keys()[0], "CC");
}

/// enter_space_select switches mode
#[test]
fn enter_space_select_changes_mode() {
    // Arrange
    let mut picker = Picker::default();

    // Act
    picker.enter_space_select(vec![
        "EVB".into(),
    ]);

    // Assert
    assert_eq!(
        *picker.mode(),
        PickerMode::JiraBoardSelect,
    );
}

/// set_jira_selected_project round-trips
#[test]
fn selected_project_round_trip() {
    // Arrange
    let mut picker = Picker::default();

    // Act
    picker.set_jira_selected_project(
        Some("CC".into()),
    );

    // Assert
    assert_eq!(
        picker.jira_selected_project(),
        Some("CC"),
    );
}

/// clear_jira_projects resets state
#[test]
fn clear_projects_resets_state() {
    // Arrange
    let mut picker = Picker::default();
    picker.enter_space_select(vec![
        "CC".into(), "EVB".into(),
    ]);
    picker.set_jira_selected_project(
        Some("CC".into()),
    );

    // Act
    picker.clear_jira_projects();

    // Assert
    assert!(picker.jira_project_keys().is_empty());
    assert!(
        picker.jira_selected_project().is_none()
    );
}

/// jira_project_keys empty by default
#[test]
fn project_keys_empty_by_default() {
    // Arrange & Act
    let picker = Picker::default();

    // Assert
    assert!(picker.jira_project_keys().is_empty());
    assert!(
        picker.jira_selected_project().is_none()
    );
}
