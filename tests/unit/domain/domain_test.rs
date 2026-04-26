//! Domain module unit tests
//!
//! Tests for CursorPosition, FilePath,
//! FileName, and FileReference.

use std::path::PathBuf;

use rustean::domain::{
    CursorPosition, FileName, FilePath,
    FileReference, InputSpan,
};

/// CursorPosition starts at zero by default
#[test]
fn cursor_new_starts_at_zero() {
    let cursor = CursorPosition::default();
    assert_eq!(cursor.get(), 0);
}

/// from_raw + get round-trips the value
#[test]
fn cursor_from_raw_returns_value() {
    let cursor = CursorPosition::from(42);
    assert_eq!(cursor.get(), 42);
}

/// move_left respects char boundaries
#[test]
fn cursor_move_left_cases() {
    let cases: &[(&str, usize, usize)] = &[
        ("hello", 3, 2),
        ("hello", 0, 0),
        ("h\u{e9}", 3, 1),
    ];
    for &(input, start, expected) in cases {
        let mut cursor =
            CursorPosition::from(start);
        cursor.move_left(input);
        assert_eq!(
            cursor.get(), expected,
            "move_left({input:?}, {start})",
        );
    }
}

/// move_right respects char boundaries
#[test]
fn cursor_move_right_cases() {
    let cases: &[(&str, usize, usize)] = &[
        ("hello", 2, 3),
        ("hello", 5, 5),
        ("h\u{e9}!", 1, 3),
    ];
    for &(input, start, expected) in cases {
        let mut cursor =
            CursorPosition::from(start);
        cursor.move_right(input);
        assert_eq!(
            cursor.get(), expected,
            "move_right({input:?}, {start})",
        );
    }
}

/// jump_to_start resets position to 0
#[test]
fn cursor_jump_to_start() {
    let mut cursor = CursorPosition::from(10);
    cursor.jump_to_start();
    assert_eq!(cursor.get(), 0);
}

/// jump_to_end sets position to max
#[test]
fn cursor_jump_to_end() {
    let mut cursor = CursorPosition::default();
    cursor.jump_to_end(99);
    assert_eq!(cursor.get(), 99);
}

/// set overwrites position to arbitrary value
#[test]
fn cursor_set() {
    let mut cursor = CursorPosition::default();
    cursor.set(25);
    assert_eq!(cursor.get(), 25);
}

/// Default and From traits work correctly
#[test]
fn cursor_trait_conversions() {
    assert_eq!(CursorPosition::default().get(), 0);

    let from_usize: CursorPosition = 15usize.into();
    assert_eq!(from_usize.get(), 15);

    let back: usize =
        CursorPosition::from(33).into();
    assert_eq!(back, 33);
}

/// Copy semantics: mutation of copy is independent
#[test]
fn cursor_copy_independence() {
    let original = CursorPosition::from(4);
    let mut copy = original;
    copy.set(99);
    assert_eq!(original.get(), 4);
    assert_eq!(copy.get(), 99);
}

/// Equality compares inner values
#[test]
fn cursor_equality() {
    let a = CursorPosition::from(5);
    let b = CursorPosition::from(5);
    let c = CursorPosition::from(6);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

/// FilePath wraps and exposes path data
#[test]
fn file_path_new_and_accessors() {
    let pb = PathBuf::from("/src/main.rs");
    let fp = FilePath::from(pb.clone());
    assert_eq!(fp.as_path(), pb.as_path());
    assert_eq!(fp.to_string(), "/src/main.rs");
}

/// FilePath file_name extracts last component
#[test]
fn file_path_file_name() {
    let fp: FilePath =
        PathBuf::from("/src/cursor.rs").into();
    assert_eq!(
        fp.file_name(),
        Some("cursor.rs".to_string()),
    );
}

/// Root path has no file_name
#[test]
fn file_path_root_has_no_name() {
    let fp = FilePath::from(PathBuf::from("/"));
    assert_eq!(fp.file_name(), None);
}

/// FilePath From trait conversions
#[test]
fn file_path_from_conversions() {
    let from_pb: FilePath =
        PathBuf::from("/a.rs").into();
    assert_eq!(from_pb.to_string(), "/a.rs");

    let from_string: FilePath =
        String::from("/b.rs").into();
    assert_eq!(from_string.to_string(), "/b.rs");

    let from_str: FilePath = "/c.rs".into();
    assert_eq!(from_str.to_string(), "/c.rs");
}

/// FilePath equality and clone
#[test]
fn file_path_equality_and_clone() {
    let a: FilePath = "/same.rs".into();
    let b: FilePath = "/same.rs".into();
    let c: FilePath = "/other.rs".into();
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_eq!(a.clone(), a);
}

/// FileName wraps and exposes string data
#[test]
fn file_name_accessors() {
    let name =
        FileName::from("main.rs".to_string());
    assert_eq!(name.as_str(), "main.rs");
    assert_eq!(name.to_string(), "main.rs");
    let s: &str = name.as_ref();
    assert_eq!(s, "main.rs");
}

/// FileName From trait conversions
#[test]
fn file_name_from_conversions() {
    let from_string: FileName =
        String::from("test.rs").into();
    assert_eq!(from_string.as_str(), "test.rs");

    let from_str: FileName = "cursor.rs".into();
    assert_eq!(from_str.as_str(), "cursor.rs");
}

/// FileName equality and clone
#[test]
fn file_name_equality_and_clone() {
    let a = FileName::from("same.rs".to_string());
    let b = FileName::from("same.rs".to_string());
    let c = FileName::from("other.rs".to_string());
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_eq!(a.clone(), a);
}

/// FileReference stores all fields correctly
#[test]
fn file_reference_stores_fields() {
    let span = InputSpan { start: 5, end: 20 };
    let path: FilePath =
        "./src/main.rs".into();
    let display =
        FileName::from("main.rs".to_string());
    let fref = FileReference::new(
        span, path, display, false,
    );

    assert_eq!(fref.start, 5);
    assert_eq!(fref.end, 20);
    assert_eq!(
        fref.full_path.to_string(), "./src/main.rs",
    );
    assert_eq!(fref.display_name.as_str(), "main.rs");
    assert!(!fref.is_dir);
}

/// FileReference for a directory
#[test]
fn file_reference_directory() {
    let span = InputSpan { start: 0, end: 6 };
    let path: FilePath = "./src/".into();
    let display =
        FileName::from("src".to_string());
    let fref = FileReference::new(
        span, path, display, true,
    );

    assert!(fref.is_dir);
    assert_eq!(
        fref.full_path.to_string(), "./src/",
    );
}

/// FileReference clone preserves all fields
#[test]
fn file_reference_clone() {
    let span = InputSpan { start: 0, end: 10 };
    let path: FilePath =
        "./Cargo.toml".into();
    let display =
        FileName::from("Cargo.toml".to_string());
    let original = FileReference::new(
        span, path, display, false,
    );
    let cloned = original.clone();

    assert_eq!(cloned.start, original.start);
    assert_eq!(cloned.end, original.end);
    assert_eq!(cloned.full_path, original.full_path);
    assert_eq!(
        cloned.display_name, original.display_name,
    );
    assert_eq!(cloned.is_dir, original.is_dir);
}
