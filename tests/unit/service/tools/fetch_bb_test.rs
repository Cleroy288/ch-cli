//! Tests for BB type deserialization.

use rustean::service::tools::bb_types::{
    BbBranch, BbPage, BbPullRequest,
};

/// Deserialize branch page JSON
#[test]
fn deserialize_branch_page() {
    // Arrange
    let json = r#"{
        "values": [
            {"name": "main"},
            {"name": "feat/login"}
        ]
    }"#;

    // Act
    let page: BbPage<BbBranch> =
        serde_json::from_str(json).unwrap();

    // Assert
    assert_eq!(page.values.len(), 2);
    assert_eq!(page.values[0].name, "main");
    assert_eq!(page.values[1].name, "feat/login");
}

/// Deserialize PR page JSON
#[test]
fn deserialize_pr_page() {
    // Arrange
    let json = r#"{
        "values": [
            {
                "id": 42,
                "title": "Fix bug",
                "state": "OPEN"
            }
        ]
    }"#;

    // Act
    let page: BbPage<BbPullRequest> =
        serde_json::from_str(json).unwrap();

    // Assert
    assert_eq!(page.values.len(), 1);
    assert_eq!(page.values[0].id, 42);
    assert_eq!(page.values[0].title, "Fix bug");
    assert_eq!(page.values[0].state, "OPEN");
}

/// Empty values list deserializes correctly
#[test]
fn deserialize_empty_page() {
    // Arrange
    let json = r#"{"values": []}"#;

    // Act
    let page: BbPage<BbBranch> =
        serde_json::from_str(json).unwrap();

    // Assert
    assert!(page.values.is_empty());
}
