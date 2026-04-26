//! Tests for Jira detail domain types.

use rustean::domain::jira_detail::{
    JiraComment, JiraIssueDetail,
};

/// JiraIssueDetail can be constructed
#[test]
fn issue_detail_constructs_with_all_fields() {
    // Arrange & Act
    let detail = make_detail();

    // Assert
    assert_eq!(detail.key, "EVB-42");
    assert_eq!(detail.summary, "Fix bug");
    assert_eq!(detail.comments.len(), 1);
}

/// JiraIssueDetail with empty description
#[test]
fn issue_detail_empty_description() {
    // Arrange & Act
    let detail = JiraIssueDetail {
        description: String::new(),
        ..make_detail()
    };

    // Assert
    assert!(detail.description.is_empty());
}

/// JiraIssueDetail with no comments
#[test]
fn issue_detail_no_comments() {
    // Arrange & Act
    let detail = JiraIssueDetail {
        comments: Vec::new(),
        ..make_detail()
    };

    // Assert
    assert!(detail.comments.is_empty());
}

/// JiraComment stores all fields
#[test]
fn comment_stores_fields() {
    // Arrange & Act
    let comment = JiraComment {
        author: "Alice".into(),
        body: "Fixed in v2".into(),
        created: "2026-02-15".into(),
    };

    // Assert
    assert_eq!(comment.author, "Alice");
    assert_eq!(comment.body, "Fixed in v2");
    assert_eq!(comment.created, "2026-02-15");
}

/// Helper: build a test detail
fn make_detail() -> JiraIssueDetail {
    JiraIssueDetail {
        key: "EVB-42".into(),
        summary: "Fix bug".into(),
        status: "Open".into(),
        priority: "High".into(),
        issue_type: "Bug".into(),
        assignee: "Alice".into(),
        story_points: Some(3.0),
        description: "A detailed desc".into(),
        comments: vec![JiraComment {
            author: "Bob".into(),
            body: "Looks good".into(),
            created: "2026-01-15".into(),
        }],
    }
}
