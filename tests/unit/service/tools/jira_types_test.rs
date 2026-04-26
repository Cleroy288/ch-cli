//! Tests for expanded Jira type deserialization.

use rustean::service::tools::jira_types::{
    JiraSearchResult,
};

/// Deserialize issue with full fields
#[test]
fn deserialize_full_fields() {
    // Arrange
    let json = r#"{
        "issues": [{
            "key": "EVB-42",
            "fields": {
                "summary": "Add auth",
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
                    "displayName": "Alice"
                },
                "description": null,
                "customfield_10016": 5.0,
                "customfield_10021": [{
                    "name": "Sprint 1",
                    "state": "active",
                    "startDate": "2026-02-01",
                    "endDate": "2026-02-14"
                }],
                "comment": {
                    "total": 3,
                    "comments": []
                }
            }
        }]
    }"#;

    // Act
    let result: JiraSearchResult =
        serde_json::from_str(json).unwrap();

    // Assert
    let issue = &result.issues[0];
    assert_eq!(issue.key, "EVB-42");
    let f = &issue.fields;
    assert_eq!(f.summary, "Add auth");
    assert_eq!(f.story_points, Some(5.0));
}

/// Deserialize issue with null optional fields
#[test]
fn deserialize_null_optionals() {
    // Arrange
    let json = r#"{
        "issues": [{
            "key": "EVB-1",
            "fields": {
                "summary": "Test",
                "status": {"name": "Open"},
                "priority": null,
                "issuetype": null,
                "assignee": null,
                "description": null,
                "customfield_10016": null,
                "customfield_10021": null,
                "comment": null
            }
        }]
    }"#;

    // Act
    let result: JiraSearchResult =
        serde_json::from_str(json).unwrap();

    // Assert
    let f = &result.issues[0].fields;
    assert!(f.priority.is_none());
    assert!(f.assignee.is_none());
    assert!(f.story_points.is_none());
    assert!(f.sprint.is_none());
}

/// Deserialize with missing optional fields
#[test]
fn deserialize_missing_fields() {
    // Arrange
    let json = r#"{
        "issues": [{
            "key": "EVB-1",
            "fields": {
                "summary": "Minimal",
                "status": {"name": "Open"}
            }
        }]
    }"#;

    // Act
    let result: JiraSearchResult =
        serde_json::from_str(json).unwrap();

    // Assert
    let f = &result.issues[0].fields;
    assert!(f.priority.is_none());
    assert!(f.sprint.is_none());
    assert!(f.comment.is_none());
}

/// Sprint field deserializes from array
#[test]
fn deserialize_sprint_array() {
    // Arrange
    let json = r#"{
        "issues": [{
            "key": "EVB-1",
            "fields": {
                "summary": "Test",
                "status": {"name": "Open"},
                "customfield_10021": [
                    {
                        "name": "Sprint 2",
                        "state": "active",
                        "startDate": "2026-02-09",
                        "endDate": "2026-02-23"
                    },
                    {
                        "name": "Sprint 1",
                        "state": "closed"
                    }
                ]
            }
        }]
    }"#;

    // Act
    let result: JiraSearchResult =
        serde_json::from_str(json).unwrap();

    // Assert
    let sprints = result.issues[0]
        .fields
        .sprint
        .as_ref()
        .unwrap();
    assert_eq!(sprints.len(), 2);
    assert_eq!(
        sprints[0].name.as_deref(),
        Some("Sprint 2"),
    );
    assert_eq!(
        sprints[0].state.as_deref(),
        Some("active"),
    );
}

/// Comment container deserializes total
#[test]
fn deserialize_comment_total() {
    // Arrange
    let json = r#"{
        "issues": [{
            "key": "EVB-1",
            "fields": {
                "summary": "Test",
                "status": {"name": "Open"},
                "comment": {
                    "total": 7,
                    "comments": []
                }
            }
        }]
    }"#;

    // Act
    let result: JiraSearchResult =
        serde_json::from_str(json).unwrap();

    // Assert
    let comment = result.issues[0]
        .fields
        .comment
        .as_ref()
        .unwrap();
    assert_eq!(comment.total, Some(7));
}
