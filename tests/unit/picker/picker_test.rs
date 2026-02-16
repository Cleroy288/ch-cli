//! Picker module tests
//!
//! Unit tests for Picker state management
//! and PickerMode enum.

use std::path::PathBuf;

use rustean::picker::{Picker, PickerMode};

// -----------------------------------------------------------
// PickerMode tests
// -----------------------------------------------------------

/// Browse variant stores its directory path
#[test]
fn test_picker_mode_browse_stores_dir() {
    let mode = PickerMode::Browse {
        dir: PathBuf::from("src"),
    };

    match mode {
        PickerMode::Browse { dir } => {
            assert_eq!(dir, PathBuf::from("src"));
        }
        _ => panic!("expected Browse"),
    }
}

/// Inactive is distinct from Browse
#[test]
fn test_picker_mode_inactive_ne_browse() {
    let inactive = PickerMode::Inactive;
    let browse = PickerMode::Browse {
        dir: PathBuf::from("."),
    };
    assert_ne!(inactive, browse);
}

/// PickerMode supports Debug formatting
#[test]
fn test_picker_mode_debug() {
    let mode = PickerMode::Browse {
        dir: PathBuf::from("."),
    };
    let debug_str = format!("{:?}", mode);
    assert!(debug_str.contains("Browse"));
}

// -----------------------------------------------------------
// Picker::new tests
// -----------------------------------------------------------

/// New picker starts Inactive with empty query
#[test]
fn test_picker_new_defaults() {
    let picker = Picker::default();

    assert_eq!(*picker.mode(), PickerMode::Inactive);
    assert_eq!(picker.query(), "");
    assert_eq!(picker.selected_index(), 0);
    assert_eq!(picker.trigger_position(), 0);
}

/// New picker is not active
#[test]
fn test_picker_new_is_not_active() {
    let picker = Picker::default();
    assert!(!picker.is_active());
}

// -----------------------------------------------------------
// Picker::activate / deactivate tests
// -----------------------------------------------------------

/// Activating sets Browse mode at root
#[test]
fn test_picker_activate() {
    let mut picker = Picker::default();
    let trigger_pos = 42;

    picker.activate(trigger_pos);

    assert!(picker.is_active());
    assert_eq!(
        *picker.mode(),
        PickerMode::Browse {
            dir: PathBuf::from(".")
        }
    );
    assert_eq!(picker.trigger_position(), trigger_pos);
    assert_eq!(picker.selected_index(), 0);
    assert_eq!(picker.query(), "");
}

/// Activating clears any previous query
#[test]
fn test_picker_activate_clears_state() {
    let mut picker = Picker::default();
    picker.activate(10);
    picker.push_query('x');
    picker.move_down(5);

    picker.activate(99);

    assert_eq!(picker.query(), "");
    assert_eq!(picker.selected_index(), 0);
    assert_eq!(picker.trigger_position(), 99);
}

/// Deactivating returns picker to Inactive
#[test]
fn test_picker_deactivate() {
    let mut picker = Picker::default();
    picker.activate(5);
    picker.push_query('a');

    picker.deactivate();

    assert!(!picker.is_active());
    assert_eq!(*picker.mode(), PickerMode::Inactive);
    assert_eq!(picker.query(), "");
    assert_eq!(picker.selected_index(), 0);
}

// -----------------------------------------------------------
// Picker query manipulation tests
// -----------------------------------------------------------

/// push_query appends characters
#[test]
fn test_picker_push_query() {
    let mut picker = Picker::default();
    picker.activate(0);

    picker.push_query('h');
    picker.push_query('e');
    picker.push_query('l');

    assert_eq!(picker.query(), "hel");
    assert_eq!(picker.selected_index(), 0);
}

/// pop_query removes the last character
#[test]
fn test_picker_pop_query() {
    let mut picker = Picker::default();
    picker.activate(0);
    picker.push_query('a');
    picker.push_query('b');
    picker.push_query('c');

    picker.pop_query();

    assert_eq!(picker.query(), "ab");
}

/// pop_query on empty does not panic
#[test]
fn test_picker_pop_query_on_empty() {
    let mut picker = Picker::default();
    picker.activate(0);
    picker.pop_query();
    assert_eq!(picker.query(), "");
}

/// clear_query empties the query
#[test]
fn test_picker_clear_query() {
    let mut picker = Picker::default();
    picker.activate(0);
    picker.push_query('x');
    picker.push_query('y');

    picker.clear_query();

    assert_eq!(picker.query(), "");
    assert_eq!(picker.selected_index(), 0);
}

// -----------------------------------------------------------
// Picker move tests
// -----------------------------------------------------------

/// move_down increments selected index
#[test]
fn test_picker_move_down() {
    let mut picker = Picker::default();

    picker.move_down(5);
    assert_eq!(picker.selected_index(), 1);

    picker.move_down(5);
    assert_eq!(picker.selected_index(), 2);
}

/// move_down clamps at max
#[test]
fn test_picker_move_down_clamps_at_max() {
    let mut picker = Picker::default();

    picker.move_down(3);
    picker.move_down(3);
    picker.move_down(3);
    picker.move_down(3);

    assert_eq!(picker.selected_index(), 2);
}

/// move_up decrements selected index
#[test]
fn test_picker_move_up() {
    let mut picker = Picker::default();
    picker.move_down(10);
    picker.move_down(10);
    assert_eq!(picker.selected_index(), 2);

    picker.move_up();
    assert_eq!(picker.selected_index(), 1);
}

/// move_up at 0 stays at 0
#[test]
fn test_picker_move_up_clamps_at_zero() {
    let mut picker = Picker::default();
    picker.move_up();
    assert_eq!(picker.selected_index(), 0);
}

// -----------------------------------------------------------
// Default trait test
// -----------------------------------------------------------

/// Default creates inactive picker with empty query
#[test]
fn test_picker_default_state() {
    let picker = Picker::default();

    assert_eq!(
        *picker.mode(),
        PickerMode::Inactive
    );
    assert_eq!(picker.query(), "");
    assert_eq!(picker.selected_index(), 0);
}

// -----------------------------------------------------------
// is_symbol_mode tests
// -----------------------------------------------------------

/// is_symbol_mode returns false when Inactive
#[test]
fn test_is_symbol_mode_inactive() {
    let picker = Picker::default();
    assert!(!picker.is_symbol_mode());
}

/// is_symbol_mode returns false when Browse
#[test]
fn test_is_symbol_mode_browse() {
    let mut picker = Picker::default();
    picker.activate(0);
    assert!(!picker.is_symbol_mode());
}

/// is_symbol_mode returns true when Symbols
#[test]
fn test_is_symbol_mode_symbols() {
    let mut picker = Picker::default();
    picker.activate_symbols(
        0,
        PathBuf::from("src/lib.rs"),
        vec![],
    );
    assert!(picker.is_symbol_mode());
}
