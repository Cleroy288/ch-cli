//! Tests for Jira type deserialization.

use rustean::service::tools::jira_types::{
    JiraSearchResult,
};

/// Deserialize Jira search result
#[test]
fn deserialize_jira_search_result() {
    // Arrange
    let json = r#"{
        "issues": [
            {
                "key": "EVB-123",
                "fields": {
                    "summary": "Fix login",
                    "status": {"name": "Open"}
                }
            }
        ]
    }"#;

    // Act
    let result: JiraSearchResult =
        serde_json::from_str(json).unwrap();

    // Assert
    assert_eq!(result.issues.len(), 1);
    assert_eq!(result.issues[0].key, "EVB-123");
    assert_eq!(
        result.issues[0].fields.summary,
        "Fix login"
    );
    assert_eq!(
        result.issues[0].fields.status.name,
        "Open"
    );
}

/// Deserialize empty issues list
#[test]
fn deserialize_empty_issues() {
    // Arrange
    let json = r#"{"issues": []}"#;

    // Act
    let result: JiraSearchResult =
        serde_json::from_str(json).unwrap();

    // Assert
    assert!(result.issues.is_empty());
}

/// Multiple issues deserialize correctly
#[test]
fn deserialize_multiple_issues() {
    // Arrange
    let json = r#"{
        "issues": [
            {
                "key": "EVB-1",
                "fields": {
                    "summary": "First",
                    "status": {"name": "Open"}
                }
            },
            {
                "key": "EVB-2",
                "fields": {
                    "summary": "Second",
                    "status": {"name": "Done"}
                }
            }
        ]
    }"#;

    // Act
    let result: JiraSearchResult =
        serde_json::from_str(json).unwrap();

    // Assert
    assert_eq!(result.issues.len(), 2);
    assert_eq!(result.issues[0].key, "EVB-1");
    assert_eq!(result.issues[1].key, "EVB-2");
}
