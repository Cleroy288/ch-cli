//! Tests for sprint name sort key parsing.

use rustean::ui::components::picker::{
    render_jira_sort::sprint_sort_key,
};

/// Standard sprint name parsed correctly
#[test]
fn parse_sprint_name_standard() {
    // Arrange + Act
    let key =
        sprint_sort_key("Sprint 2 - Q1 2026");

    // Assert — (year, quarter, number)
    assert_eq!(key, (2026, 1, 2));
}

/// Bridge name parsed same as sprint
#[test]
fn parse_bridge_name() {
    // Arrange + Act
    let key =
        sprint_sort_key("Bridge 3 - Q1 2026");

    // Assert
    assert_eq!(key, (2026, 1, 3));
}

/// Different quarter and year parsed
#[test]
fn parse_different_period() {
    // Arrange + Act
    let key =
        sprint_sort_key("Sprint 1 - Q4 2025");

    // Assert
    assert_eq!(key, (2025, 4, 1));
}

/// Sprint 2 Q1 sorts before Bridge 3 Q1
#[test]
fn sort_within_same_quarter() {
    // Arrange + Act
    let key_a =
        sprint_sort_key("Sprint 2 - Q1 2026");
    let key_b =
        sprint_sort_key("Bridge 3 - Q1 2026");

    // Assert — number 2 < number 3
    assert!(key_a < key_b);
}

/// No-match names sort last (MAX values)
#[test]
fn unparseable_name_sorts_last() {
    // Arrange + Act
    let valid =
        sprint_sort_key("Sprint 1 - Q1 2026");
    let invalid =
        sprint_sort_key("No Sprint");

    // Assert
    assert!(valid < invalid);
}
