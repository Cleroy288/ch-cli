//! Tests for Jira detail mapping and comments.

use rustean::service::tools::jira_types::{
    JiraIssue, JiraSearchResult,
};

/// Helper: deserialize a single issue from JSON
fn issue_from_json(json: &str) -> JiraIssue {
    serde_json::from_str(json).unwrap()
}

/// Comment entries deserialized from JSON
#[test]
fn comments_deserialized_from_json() {
    // Arrange
    let issue = issue_from_json(r#"{
        "key": "EVB-1",
        "fields": {
            "summary": "Test",
            "status": {"name": "Open"},
            "comment": {
                "total": 2,
                "comments": [
                    {
                        "author": {
                            "displayName": "Bob"
                        },
                        "body": {
                            "type": "doc",
                            "content": [{
                                "type": "paragraph",
                                "content": [{
                                    "type": "text",
                                    "text": "Hello"
                                }]
                            }]
                        },
                        "created": "2026-01-15T10:00:00"
                    }
                ]
            }
        }
    }"#);

    // Assert
    let container =
        issue.fields.comment.unwrap();
    assert_eq!(container.total, Some(2));
    let entries = container.comments.unwrap();
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];
    assert_eq!(
        entry.author.as_ref().unwrap()
            .display_name,
        "Bob",
    );
    assert!(entry.body.is_some());
    assert_eq!(
        entry.created.as_deref(),
        Some("2026-01-15T10:00:00"),
    );
}

/// Missing comments field yields None
#[test]
fn no_comments_field_yields_none() {
    // Arrange
    let issue = issue_from_json(r#"{
        "key": "EVB-2",
        "fields": {
            "summary": "Test",
            "status": {"name": "Open"}
        }
    }"#);

    // Assert
    assert!(issue.fields.comment.is_none());
}

/// Empty comments array deserialized
#[test]
fn empty_comments_array_deserialized() {
    // Arrange
    let issue = issue_from_json(r#"{
        "key": "EVB-3",
        "fields": {
            "summary": "Test",
            "status": {"name": "Open"},
            "comment": {
                "total": 0,
                "comments": []
            }
        }
    }"#);

    // Assert
    let container =
        issue.fields.comment.unwrap();
    let entries = container.comments.unwrap();
    assert!(entries.is_empty());
}

/// Comment with missing author defaults safely
#[test]
fn comment_missing_author_is_none() {
    // Arrange
    let issue = issue_from_json(r#"{
        "key": "EVB-4",
        "fields": {
            "summary": "Test",
            "status": {"name": "Open"},
            "comment": {
                "total": 1,
                "comments": [{
                    "body": null,
                    "created": "2026-01-01"
                }]
            }
        }
    }"#);

    // Assert
    let entries = issue.fields.comment.unwrap()
        .comments.unwrap();
    assert!(entries[0].author.is_none());
    assert!(entries[0].body.is_none());
}
