//! Tests for compute_input_height (text wrapping).

use rustean::ui::components::input::compute_input_height;

/// Build a string of `n` ASCII characters.
fn make_input(n: usize) -> String {
    "a".repeat(n)
}

// -- happy path --

#[test]
fn empty_input_returns_min_height() {
    // 0 chars → 1 line → 1+1+1=3
    assert_eq!(
        compute_input_height("", 80, 40),
        3,
    );
}

#[test]
fn short_input_returns_min_height() {
    // 10 chars, first_row=78 → fits → 3
    let s = make_input(10);
    assert_eq!(
        compute_input_height(&s, 80, 40),
        3,
    );
}

#[test]
fn one_wrap_adds_one_line() {
    // 79 chars, first=78 → 1 leftover
    let s = make_input(79);
    assert_eq!(
        compute_input_height(&s, 80, 40),
        4,
    );
}

#[test]
fn two_wraps_adds_two_lines() {
    // past = 160-78 = 82 → ceil(82/80)=2
    let s = make_input(160);
    assert_eq!(
        compute_input_height(&s, 80, 40),
        5,
    );
}

#[test]
fn capped_by_max_frac() {
    // 1000 chars → raw=15, cap=24/3=8
    let s = make_input(1000);
    assert_eq!(
        compute_input_height(&s, 80, 24),
        8,
    );
}

// -- zero / narrow --

#[test]
fn zero_width_returns_min_height() {
    let s = make_input(50);
    assert_eq!(
        compute_input_height(&s, 0, 40),
        3,
    );
}

#[test]
fn width_1_returns_min_height() {
    let s = make_input(50);
    assert_eq!(
        compute_input_height(&s, 1, 40),
        3,
    );
}

#[test]
fn width_2_returns_min_height() {
    let s = make_input(50);
    assert_eq!(
        compute_input_height(&s, 2, 40),
        3,
    );
}

#[test]
fn width_3_single_col_wraps() {
    // first=1, rest=3 → past=4 → ceil(4/3)=2
    let s = make_input(5);
    assert_eq!(
        compute_input_height(&s, 3, 40),
        5,
    );
}

// -- small area / boundary --

#[test]
fn area_height_2_returns_min() {
    let s = make_input(10);
    assert_eq!(
        compute_input_height(&s, 80, 2),
        3,
    );
}

#[test]
fn input_len_1_returns_min() {
    assert_eq!(
        compute_input_height("a", 80, 40),
        3,
    );
}

#[test]
fn input_exactly_fills_first_row() {
    // first=78, input=78 → fits in row 0 → 3
    let s = make_input(78);
    assert_eq!(
        compute_input_height(&s, 80, 40),
        3,
    );
}
