//! Tests for cursor_grid — multi-line cursor math.

use rustean::domain::cursor_grid::{
    cursor_to_visual, logical_line_at,
    logical_lines, total_visual_rows,
};

// -- logical_lines --

#[test]
fn logical_lines_empty_returns_one() {
    assert_eq!(logical_lines(""), vec![""]);
}

#[test]
fn logical_lines_no_newline_single() {
    assert_eq!(logical_lines("hello"), vec!["hello"]);
}

#[test]
fn logical_lines_two_lines() {
    assert_eq!(
        logical_lines("abc\ndef"),
        vec!["abc", "def"],
    );
}

#[test]
fn logical_lines_trailing_newline() {
    assert_eq!(
        logical_lines("abc\n"),
        vec!["abc", ""],
    );
}

// -- logical_line_at --

#[test]
fn line_at_start_is_zero() {
    assert_eq!(logical_line_at("abc\ndef", 0), 0);
}

#[test]
fn line_at_before_newline() {
    assert_eq!(logical_line_at("abc\ndef", 3), 0);
}

#[test]
fn line_at_after_newline() {
    // offset 4 = 'd' on line 1
    assert_eq!(logical_line_at("abc\ndef", 4), 1);
}

#[test]
fn line_at_end_of_text() {
    assert_eq!(logical_line_at("abc\ndef", 7), 1);
}

// -- cursor_to_visual --

#[test]
fn visual_empty_at_origin() {
    assert_eq!(
        cursor_to_visual("", 0, 78, 80),
        (0, 0),
    );
}

#[test]
fn visual_single_line_mid() {
    assert_eq!(
        cursor_to_visual("hello", 3, 78, 80),
        (0, 3),
    );
}

#[test]
fn visual_two_lines_second_start() {
    // "abc\ndef" → line 0 len=3, line 1 starts at 4
    // first_cols=78 → line 0 fits on row 0
    // cursor at 4 → row 1, col 0
    assert_eq!(
        cursor_to_visual("abc\ndef", 4, 78, 80),
        (1, 0),
    );
}

#[test]
fn visual_two_lines_second_mid() {
    // cursor at 5 → 'd','e' → col 1 on row 1
    assert_eq!(
        cursor_to_visual("abc\ndef", 5, 78, 80),
        (1, 1),
    );
}

#[test]
fn visual_zero_width_returns_origin() {
    assert_eq!(
        cursor_to_visual("abc", 2, 0, 0),
        (0, 0),
    );
}

// -- total_visual_rows --

#[test]
fn total_rows_empty() {
    assert_eq!(total_visual_rows("", 78, 80), 1);
}

#[test]
fn total_rows_single_short_line() {
    assert_eq!(
        total_visual_rows("hello", 78, 80),
        1,
    );
}

#[test]
fn total_rows_two_short_lines() {
    assert_eq!(
        total_visual_rows("abc\ndef", 78, 80),
        2,
    );
}

#[test]
fn total_rows_long_first_line_wraps() {
    // first_cols=5, full=10
    // 8 chars on line 0: first 5 fit, 3 left
    // → ceil(3/10) = 1 → 2 rows for line 0
    let s = "abcdefgh";
    assert_eq!(total_visual_rows(s, 5, 10), 2);
}

#[test]
fn total_rows_zero_width() {
    assert_eq!(
        total_visual_rows("abc", 0, 0),
        1,
    );
}

// -- line 2+ uses full_width, not first_cols --

#[test]
fn visual_line2_uses_full_width() {
    // first_cols=5, full=10
    // "abc\n1234567890" → line 0 on row 0,
    // line 1 has 10 chars = fits in 10 cols (full)
    // cursor at end of line 1 → col 10
    let s = "abc\n1234567890";
    // byte offset = 3(\n) + 1 + 10 = 14
    assert_eq!(
        cursor_to_visual(s, 14, 5, 10),
        (1, 10),
    );
}

#[test]
fn total_rows_line2_no_prompt_penalty() {
    // first_cols=5, full=10
    // Line 0: "abcde" (5 chars) → 1 row (fits in 5)
    // Line 1: "1234567890" (10 chars) → 1 row
    //   (full_width=10, NOT first_cols=5)
    let s = "abcde\n1234567890";
    assert_eq!(total_visual_rows(s, 5, 10), 2);
}

#[test]
fn visual_line2_long_wraps_at_full_width() {
    // first_cols=5, full=10
    // Line 1: 15 chars → first 10 fit, 5 left
    // → 2 visual rows for line 1
    let s = "abc\naaaaabbbbbccccc";
    // cursor at end (byte 19) → line 1 row 1 col 5
    assert_eq!(
        cursor_to_visual(s, 19, 5, 10),
        (2, 5),
    );
}
