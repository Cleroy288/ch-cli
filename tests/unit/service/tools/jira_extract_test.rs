//! Tests for Jira field extraction helpers.

use rustean::service::tools::jira_extract::{
    extract_assignee, extract_type,
};
use rustean::service::tools::jira_types::JiraFields;

/// Helper: deserialize JiraFields from JSON
fn fields_from_json(json: &str) -> JiraFields {
    serde_json::from_str(json).unwrap()
}

// --- extract_type ---

/// Issue type present returns its name
#[test]
fn extract_type_present_returns_name() {
    // Arrange
    let f = fields_from_json(r#"{
        "summary": "t",
        "status": {"name": "Open"},
        "issuetype": {"name": "Bug"}
    }"#);

    // Act
    let result = extract_type(&f);

    // Assert
    assert_eq!(result, "Bug");
}

/// Issue type absent defaults to "Task"
#[test]
fn extract_type_absent_defaults_task() {
    // Arrange
    let f = fields_from_json(r#"{
        "summary": "t",
        "status": {"name": "Open"}
    }"#);

    // Act
    let result = extract_type(&f);

    // Assert
    assert_eq!(result, "Task");
}

// --- extract_assignee ---

/// Assignee present returns display name
#[test]
fn extract_assignee_present_returns_name() {
    // Arrange
    let f = fields_from_json(r#"{
        "summary": "t",
        "status": {"name": "Open"},
        "assignee": {"displayName": "Alice"}
    }"#);

    // Act
    let result = extract_assignee(&f);

    // Assert
    assert_eq!(result, "Alice");
}

/// Assignee absent returns "Unassigned"
#[test]
fn extract_assignee_absent_returns_unassigned() {
    // Arrange
    let f = fields_from_json(r#"{
        "summary": "t",
        "status": {"name": "Open"}
    }"#);

    // Act
    let result = extract_assignee(&f);

    // Assert
    assert_eq!(result, "Unassigned");
}
