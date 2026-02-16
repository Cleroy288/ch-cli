//! Domain module unit tests
//!
//! Unit tests for CursorPosition, FilePath, FileName, and FileReference.

use std::path::PathBuf;

use rustean::domain::{
    CursorPosition, FileName, FilePath,
    FileReference, InputSpan,
};

// ─── CursorPosition tests ───────────────────────────────────────────────────

/// CursorPosition::new creates a cursor at position 0
#[test]
fn test_cursor_new_starts_at_zero() {
    let cursor = CursorPosition::new(); // cursor at default start
    assert_eq!(cursor.get(), 0);
}

/// CursorPosition::from_raw creates a cursor at the given position
#[test]
fn test_cursor_from_raw() {
    let cursor = CursorPosition::from_raw(42); // cursor at position 42
    assert_eq!(cursor.get(), 42);
}

/// CursorPosition::get returns the inner value
#[test]
fn test_cursor_get() {
    let cursor = CursorPosition::from_raw(7); // cursor at position 7
    let val = cursor.get(); // extracted raw value
    assert_eq!(val, 7);
}

/// move_left decreases position by 1
#[test]
fn test_cursor_move_left() {
    let mut cursor = CursorPosition::from_raw(3); // cursor at position 3
    cursor.move_left();
    assert_eq!(cursor.get(), 2);
}

/// move_left does not go below zero
#[test]
fn test_cursor_move_left_at_zero() {
    let mut cursor = CursorPosition::new(); // cursor at position 0
    cursor.move_left();
    assert_eq!(cursor.get(), 0);
}

/// move_right increases position by 1 when below max
#[test]
fn test_cursor_move_right() {
    let mut cursor = CursorPosition::from_raw(2); // cursor at position 2
    let max = 5; // upper bound
    cursor.move_right(max);
    assert_eq!(cursor.get(), 3);
}

/// move_right does not exceed max
#[test]
fn test_cursor_move_right_at_max() {
    let mut cursor = CursorPosition::from_raw(5); // cursor at max position
    let max = 5; // upper bound
    cursor.move_right(max);
    assert_eq!(cursor.get(), 5);
}

/// jump_to_start resets position to 0
#[test]
fn test_cursor_jump_to_start() {
    let mut cursor = CursorPosition::from_raw(10); // cursor at position 10
    cursor.jump_to_start();
    assert_eq!(cursor.get(), 0);
}

/// jump_to_end sets position to max
#[test]
fn test_cursor_jump_to_end() {
    let mut cursor = CursorPosition::new(); // cursor at position 0
    let max = 99; // upper bound
    cursor.jump_to_end(max);
    assert_eq!(cursor.get(), 99);
}

/// set overwrites the position to an arbitrary value
#[test]
fn test_cursor_set() {
    let mut cursor = CursorPosition::new(); // cursor at position 0
    cursor.set(25);
    assert_eq!(cursor.get(), 25);
}

/// Default trait produces position 0
#[test]
fn test_cursor_default() {
    let cursor = CursorPosition::default(); // default cursor
    assert_eq!(cursor.get(), 0);
}

/// From<usize> creates cursor from usize
#[test]
fn test_cursor_from_usize() {
    let cursor: CursorPosition = 15usize.into(); // cursor from usize conversion
    assert_eq!(cursor.get(), 15);
}

/// From<CursorPosition> for usize extracts the inner value
#[test]
fn test_cursor_into_usize() {
    let cursor = CursorPosition::from_raw(33); // cursor at position 33
    let val: usize = cursor.into(); // converted back to usize
    assert_eq!(val, 33);
}

/// CursorPosition supports equality comparison
#[test]
fn test_cursor_equality() {
    let a = CursorPosition::from_raw(5); // first cursor
    let b = CursorPosition::from_raw(5); // second cursor, same position
    let c = CursorPosition::from_raw(6); // third cursor, different position
    assert_eq!(a, b);
    assert_ne!(a, c);
}

/// CursorPosition supports clone
#[test]
fn test_cursor_clone() {
    let original = CursorPosition::from_raw(8); // original cursor
    let cloned = original; // cloned via Copy
    assert_eq!(original, cloned);
}

/// CursorPosition supports copy (mutation of copy does not affect original)
#[test]
fn test_cursor_copy() {
    let original = CursorPosition::from_raw(4); // original cursor
    let mut copy = original; // copy via Copy trait
    copy.set(99);
    assert_eq!(original.get(), 4);
    assert_eq!(copy.get(), 99);
}

// ─── FilePath tests ─────────────────────────────────────────────────────────

/// FilePath::new wraps a PathBuf
#[test]
fn test_file_path_new() {
    let pb = PathBuf::from("/src/main.rs"); // path buffer input
    let fp = FilePath::new(pb.clone()); // file path wrapper
    assert_eq!(fp.as_path_buf(), &pb);
}

/// FilePath::from_string creates from a string slice
#[test]
fn test_file_path_from_string() {
    let fp = FilePath::from_string("/src/lib.rs"); // file path from str
    let expected = PathBuf::from("/src/lib.rs"); // expected path buf
    assert_eq!(fp.as_path_buf(), &expected);
}

/// as_path_buf returns a reference to the inner PathBuf
#[test]
fn test_file_path_as_path_buf() {
    let fp = FilePath::from_string("/tmp/test.txt"); // file path
    let pb = fp.as_path_buf(); // reference to inner PathBuf
    assert_eq!(pb, &PathBuf::from("/tmp/test.txt"));
}

/// as_string returns the string representation
#[test]
fn test_file_path_as_string() {
    let fp = FilePath::from_string("/src/domain/mod.rs"); // file path
    let s = fp.as_string(); // string output
    assert_eq!(s, "/src/domain/mod.rs");
}

/// file_name returns the last path component
#[test]
fn test_file_path_file_name() {
    let fp = FilePath::from_string("/src/domain/cursor.rs"); // file path
    let name = fp.file_name(); // extracted filename
    assert_eq!(name, Some("cursor.rs".to_string()));
}

/// file_name returns None for root path
#[test]
fn test_file_path_file_name_root() {
    let fp = FilePath::from_string("/"); // root path
    let name = fp.file_name(); // should be None for root
    assert_eq!(name, None);
}

/// From<PathBuf> creates FilePath
#[test]
fn test_file_path_from_pathbuf() {
    let pb = PathBuf::from("/a/b/c.rs"); // path buffer
    let fp: FilePath = pb.clone().into(); // converted via From
    assert_eq!(fp.as_path_buf(), &pb);
}

/// From<String> creates FilePath
#[test]
fn test_file_path_from_owned_string() {
    let s = String::from("/x/y.rs"); // owned string
    let fp: FilePath = s.into(); // converted via From
    assert_eq!(fp.as_string(), "/x/y.rs");
}

/// From<&str> creates FilePath
#[test]
fn test_file_path_from_str_ref() {
    let fp: FilePath = "/hello/world.rs".into(); // converted via From<&str>
    assert_eq!(fp.as_string(), "/hello/world.rs");
}

/// FilePath supports equality comparison
#[test]
fn test_file_path_equality() {
    let a = FilePath::from_string("/same/path.rs"); // first path
    let b = FilePath::from_string("/same/path.rs"); // second path, same
    let c = FilePath::from_string("/other/path.rs"); // third path, different
    assert_eq!(a, b);
    assert_ne!(a, c);
}

/// FilePath supports clone
#[test]
fn test_file_path_clone() {
    let original = FilePath::from_string("/clone/test.rs"); // original
    let cloned = original.clone(); // cloned
    assert_eq!(original, cloned);
}

// ─── FileName tests ─────────────────────────────────────────────────────────

/// FileName::new wraps a String
#[test]
fn test_file_name_new() {
    let name = FileName::new("main.rs".to_string()); // file name wrapper
    assert_eq!(name.as_str(), "main.rs");
}

/// as_str returns a string slice reference
#[test]
fn test_file_name_as_str() {
    let name = FileName::new("lib.rs".to_string()); // file name
    let s = name.as_str(); // string slice
    assert_eq!(s, "lib.rs");
}

/// as_string returns an owned String clone
#[test]
fn test_file_name_as_string() {
    let name = FileName::new("mod.rs".to_string()); // file name
    let s = name.as_string(); // owned string
    assert_eq!(s, "mod.rs");
}

/// From<String> creates FileName
#[test]
fn test_file_name_from_string() {
    let name: FileName = String::from("test.rs").into(); // converted via From
    assert_eq!(name.as_str(), "test.rs");
}

/// From<&str> creates FileName
#[test]
fn test_file_name_from_str_ref() {
    let name: FileName = "cursor.rs".into(); // converted via From<&str>
    assert_eq!(name.as_str(), "cursor.rs");
}

/// AsRef<str> returns a string slice
#[test]
fn test_file_name_as_ref() {
    let name = FileName::new("file_ref.rs".to_string()); // file name
    let s: &str = name.as_ref(); // via AsRef<str>
    assert_eq!(s, "file_ref.rs");
}

/// FileName supports equality comparison
#[test]
fn test_file_name_equality() {
    let a = FileName::new("same.rs".to_string()); // first name
    let b = FileName::new("same.rs".to_string()); // second name, same
    let c = FileName::new("other.rs".to_string()); // third name, different
    assert_eq!(a, b);
    assert_ne!(a, c);
}

/// FileName supports clone
#[test]
fn test_file_name_clone() {
    let original = FileName::new("clone.rs".to_string()); // original
    let cloned = original.clone(); // cloned
    assert_eq!(original, cloned);
}

// ─── FileReference tests ────────────────────────────────────────────────────

/// FileReference::new sets all fields correctly
#[test]
fn test_file_reference_new() {
    let start = 5usize; // start position in input
    let end = 20usize; // end position in input
    let path = FilePath::from_string("./src/main.rs"); // full path
    let display = FileName::new("main.rs".to_string()); // display name
    let is_dir = false; // not a directory

    let span = InputSpan { start, end };
    let fref = FileReference::new(span, path, display, is_dir);

    assert_eq!(fref.start, 5);
    assert_eq!(fref.end, 20);
    assert_eq!(fref.full_path.as_string(), "./src/main.rs");
    assert_eq!(fref.display_name.as_str(), "main.rs");
    assert!(!fref.is_dir);
}

/// FileReference::new for a directory
#[test]
fn test_file_reference_new_directory() {
    let start = 0usize; // start position
    let end = 6usize; // end position
    let path = FilePath::from_string("./src/"); // directory path
    let display = FileName::new("src".to_string()); // display name
    let is_dir = true; // is a directory

    let span = InputSpan { start, end };
    let fref = FileReference::new(span, path, display, is_dir);

    assert_eq!(fref.start, 0);
    assert_eq!(fref.end, 6);
    assert_eq!(fref.full_path.as_string(), "./src/");
    assert_eq!(fref.display_name.as_str(), "src");
    assert!(fref.is_dir);
}

/// FileReference supports clone
#[test]
fn test_file_reference_clone() {
    let path = FilePath::from_string("./Cargo.toml"); // full path
    let display = FileName::new("Cargo.toml".to_string()); // display name
    let span = InputSpan { start: 0, end: 10 };
    let original = FileReference::new(span, path, display, false); // original ref

    let cloned = original.clone(); // cloned ref

    assert_eq!(cloned.start, original.start);
    assert_eq!(cloned.end, original.end);
    assert_eq!(cloned.full_path, original.full_path);
    assert_eq!(cloned.display_name, original.display_name);
    assert_eq!(cloned.is_dir, original.is_dir);
}
