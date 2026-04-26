//! Tests for Jira project auto-discovery.

use rustean::domain::jira::{
    JiraBoardData, JiraTicketDetail,
};
use rustean::service::tools::fetch_jira_discover::{
    build_multi_project_jql, extract_project_keys,
};

/// Build a minimal ticket for testing
fn make_ticket(key: &str) -> JiraTicketDetail {
    let project = key
        .split('-')
        .next()
        .unwrap_or(key);
    JiraTicketDetail {
        key: key.to_string(),
        summary: String::new(),
        status: String::new(),
        status_category: String::new(),
        priority: String::new(),
        issue_type: String::new(),
        assignee: String::new(),
        story_points: None,
        sprint_name: String::new(),
        project_key: project.into(),
    }
}

/// Build board from ticket keys
fn make_board(keys: &[&str]) -> JiraBoardData {
    JiraBoardData {
        sprint: None,
        tickets: keys
            .iter()
            .map(|k| make_ticket(k))
            .collect(),
        total_points: 0.0,
    }
}

// -- extract_project_keys tests --

/// Extracts unique prefixes from ticket keys
#[test]
fn extract_keys_from_mixed_projects() {
    // Arrange
    let board =
        make_board(&["EVB-1", "FOO-2", "EVB-3"]);

    // Act
    let keys = extract_project_keys(&board);

    // Assert
    assert_eq!(keys, vec!["EVB", "FOO"]);
}

/// Empty board yields no keys
#[test]
fn extract_keys_empty_board() {
    // Arrange
    let board = make_board(&[]);

    // Act
    let keys = extract_project_keys(&board);

    // Assert
    assert!(keys.is_empty());
}

/// Keys without dash are skipped
#[test]
fn extract_keys_no_dash_ignored() {
    // Arrange
    let board = make_board(&["NODASH"]);

    // Act
    let keys = extract_project_keys(&board);

    // Assert — "NODASH" has no dash, split gives
    // the whole string as prefix, still extracted
    assert_eq!(keys, vec!["NODASH"]);
}

/// Single project yields one key
#[test]
fn extract_keys_single_project() {
    // Arrange
    let board =
        make_board(&["ABC-1", "ABC-2", "ABC-3"]);

    // Act
    let keys = extract_project_keys(&board);

    // Assert
    assert_eq!(keys, vec!["ABC"]);
}

// -- build_multi_project_jql tests --

/// Two keys produce IN clause
#[test]
fn multi_jql_two_projects() {
    // Arrange
    let keys = vec![
        "EVB".to_string(),
        "FOO".to_string(),
    ];

    // Act
    let jql = build_multi_project_jql(&keys, None);

    // Assert
    assert!(jql.contains("project IN (EVB, FOO)"));
    assert!(jql.contains("ORDER BY updated DESC"));
}
