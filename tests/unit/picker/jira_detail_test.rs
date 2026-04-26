//! Tests for Jira detail picker state methods.

use rustean::domain::jira_detail::{
    JiraComment, JiraIssueDetail,
};
use rustean::picker::mode::PickerMode;
use rustean::picker::Picker;

/// set_jira_detail stores and switches mode
#[test]
fn set_detail_stores_and_switches_mode() {
    // Arrange
    let mut picker = Picker::default();

    // Act
    picker.set_jira_detail(make_detail());

    // Assert
    assert!(picker.jira_detail().is_some());
    assert_eq!(
        *picker.mode(),
        PickerMode::JiraTicketDetail,
    );
}

/// jira_detail is None by default
#[test]
fn jira_detail_none_by_default() {
    // Arrange & Act
    let picker = Picker::default();

    // Assert
    assert!(picker.jira_detail().is_none());
}

/// clear_jira_detail removes data and scroll
#[test]
fn clear_detail_removes_data() {
    // Arrange
    let mut picker = Picker::default();
    picker.set_jira_detail(make_detail());
    picker.scroll_detail_down();

    // Act
    picker.clear_jira_detail();

    // Assert
    assert!(picker.jira_detail().is_none());
    assert_eq!(picker.jira_detail_scroll(), 0);
}

/// Scroll up decreases offset
#[test]
fn scroll_up_decreases_offset() {
    // Arrange
    let mut picker = Picker::default();
    picker.set_jira_detail(make_detail());
    picker.scroll_detail_down();
    picker.scroll_detail_down();

    // Act
    picker.scroll_detail_up();

    // Assert
    assert_eq!(picker.jira_detail_scroll(), 1);
}

/// Scroll up at zero stays at zero
#[test]
fn scroll_up_at_zero_stays_zero() {
    // Arrange
    let mut picker = Picker::default();
    picker.set_jira_detail(make_detail());

    // Act
    picker.scroll_detail_up();

    // Assert
    assert_eq!(picker.jira_detail_scroll(), 0);
}

/// scroll_detail_down increments unbounded
#[test]
fn scroll_down_increments_raw_offset() {
    // Arrange
    let mut picker = Picker::default();
    picker.set_jira_detail(make_detail());

    // Act
    picker.scroll_detail_down();
    picker.scroll_detail_down();
    picker.scroll_detail_down();

    // Assert — raw offset, render clamps
    assert_eq!(picker.jira_detail_scroll(), 3);
}

/// Helper: make test detail
fn make_detail() -> JiraIssueDetail {
    JiraIssueDetail {
        key: "EVB-1".into(),
        summary: "Test".into(),
        status: "Open".into(),
        priority: "High".into(),
        issue_type: "Bug".into(),
        assignee: "Alice".into(),
        story_points: None,
        description: "desc".into(),
        comments: vec![JiraComment {
            author: "Bob".into(),
            body: "comment".into(),
            created: "2026-01-01".into(),
        }],
    }
}
