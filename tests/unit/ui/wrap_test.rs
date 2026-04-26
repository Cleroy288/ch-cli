//! Tests for the pre_wrap engine (src/ui/wrap.rs).

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use rustean::ui::wrap::pre_wrap;

// -- single-line behavior --

#[test]
fn pre_wrap_empty_line_returns_one_line() {
    // Arrange
    let lines = vec![Line::from("")];

    // Act
    let result = pre_wrap(lines, 80);

    // Assert
    assert_eq!(result.len(), 1);
}

#[test]
fn pre_wrap_short_line_fits_no_split() {
    // Arrange
    let lines = vec![Line::from("hello")];

    // Act
    let result = pre_wrap(lines, 80);

    // Assert
    assert_eq!(result.len(), 1);
    let text: String = result[0]
        .spans
        .iter()
        .map(|s| s.content.as_ref())
        .collect();
    assert_eq!(text, "hello");
}

#[test]
fn pre_wrap_line_exactly_at_width_no_split() {
    // Arrange
    let text = "a".repeat(10);
    let lines = vec![Line::from(text.clone())];

    // Act
    let result = pre_wrap(lines, 10);

    // Assert
    assert_eq!(result.len(), 1);
    let out: String = result[0]
        .spans
        .iter()
        .map(|s| s.content.as_ref())
        .collect();
    assert_eq!(out, text);
}

// -- splitting --

#[test]
fn pre_wrap_long_plain_line_splits_at_width() {
    // Arrange
    let text = "a".repeat(20);
    let lines = vec![Line::from(text)];

    // Act
    let result = pre_wrap(lines, 10);

    // Assert
    assert_eq!(result.len(), 2);
    let row0: String = result[0]
        .spans
        .iter()
        .map(|s| s.content.as_ref())
        .collect();
    let row1: String = result[1]
        .spans
        .iter()
        .map(|s| s.content.as_ref())
        .collect();
    assert_eq!(row0.len(), 10);
    assert_eq!(row1.len(), 10);
}

#[test]
fn pre_wrap_multi_span_styles_preserved() {
    // Arrange
    let red = Style::default().fg(Color::Red);
    let blue = Style::default().fg(Color::Blue);
    let s1 = Span::styled("aaaa", red);
    let s2 = Span::styled("bbbbbb", blue);
    let lines = vec![Line::from(vec![s1, s2])];

    // Act — width 6: row0=6 cols, row1=4 cols
    let result = pre_wrap(lines, 6);

    // Assert
    assert_eq!(result.len(), 2);
    let has_red =
        result[0].spans.iter().any(|s| s.style == red);
    let has_blue =
        result[1].spans.iter().any(|s| s.style == blue);
    assert!(has_red, "row 0 must keep red style");
    assert!(has_blue, "row 1 must keep blue style");
}

// -- edge cases --

#[test]
fn pre_wrap_width_zero_returns_as_is() {
    // Arrange
    let lines = vec![Line::from("hello world")];

    // Act
    let result = pre_wrap(lines, 0);

    // Assert
    assert_eq!(result.len(), 1);
}

#[test]
fn pre_wrap_empty_spans_handled() {
    // Arrange
    let spans = vec![
        Span::raw(""),
        Span::raw("hello"),
        Span::raw(""),
    ];
    let lines = vec![Line::from(spans)];

    // Act
    let result = pre_wrap(lines, 80);

    // Assert
    assert_eq!(result.len(), 1);
    let text: String = result[0]
        .spans
        .iter()
        .map(|s| s.content.as_ref())
        .collect();
    assert_eq!(text, "hello");
}

// -- unicode --

#[test]
fn pre_wrap_wide_unicode_splits_correctly() {
    // Arrange — 5 CJK chars, each 2 columns wide
    let cjk = "\u{4e00}\u{4e01}\u{4e02}\u{4e03}\u{4e04}";
    let lines = vec![Line::from(cjk)];

    // Act — width 6 fits 3 CJK chars (6 cols)
    let result = pre_wrap(lines, 6);

    // Assert — row 0: 3 chars, row 1: 2 chars
    assert_eq!(result.len(), 2);
    let row0: String = result[0]
        .spans
        .iter()
        .map(|s| s.content.as_ref())
        .collect();
    let row1: String = result[1]
        .spans
        .iter()
        .map(|s| s.content.as_ref())
        .collect();
    assert_eq!(row0.chars().count(), 3);
    assert_eq!(row1.chars().count(), 2);
}

// -- multiple input lines --

#[test]
fn pre_wrap_multiple_lines_all_wrapped() {
    // Arrange
    let lines = vec![
        Line::from("a".repeat(15)), // 2 rows
        Line::from("b".repeat(5)),  // 1 row
        Line::from("c".repeat(25)), // 3 rows
    ];

    // Act
    let result = pre_wrap(lines, 10);

    // Assert — 2 + 1 + 3 = 6 rows
    assert_eq!(result.len(), 6);
}
