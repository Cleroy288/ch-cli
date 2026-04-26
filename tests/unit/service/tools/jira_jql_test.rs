//! Tests for build_project_jql() JQL generation.

use rustean::service::tools::fetch_jira::{
    build_project_jql,
};

/// Single project: contains project = KEY
#[test]
fn build_project_jql_contains_project_key() {
    // Act
    let jql = build_project_jql("EVB", None);

    // Assert
    assert!(jql.contains("project = EVB"));
    assert!(jql.contains("ORDER BY updated DESC"));
}

/// Single project: no currentUser() clause
#[test]
fn build_project_jql_no_current_user() {
    // Act
    let jql = build_project_jql("EVB", None);

    // Assert
    assert!(!jql.contains("currentUser()"));
}

/// Single project: no OR clause present
#[test]
fn build_project_jql_no_or_clause() {
    // Act
    let jql = build_project_jql("PROJ", None);

    // Assert
    assert!(!jql.contains(" OR "));
}

/// With assignee: adds AND assignee clause
#[test]
fn build_project_jql_with_assignee() {
    // Act
    let jql = build_project_jql(
        "EVB",
        Some("Charles Leroy"),
    );

    // Assert
    assert!(jql.contains("project = EVB"));
    assert!(jql.contains(
        "AND assignee = \"Charles Leroy\""
    ));
    assert!(jql.contains("ORDER BY updated DESC"));
}

/// Without assignee: no assignee clause
#[test]
fn build_project_jql_no_assignee_clause() {
    // Act
    let jql = build_project_jql("EVB", None);

    // Assert
    assert!(!jql.contains("assignee"));
}
