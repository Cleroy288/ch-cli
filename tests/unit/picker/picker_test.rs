//! Picker module tests
//!
//! Unit tests for Picker state management and PickerMode enum.

use ch_cli::picker::{Picker, PickerMode};

// ---------------------------------------------------------------------------
// PickerMode tests
// ---------------------------------------------------------------------------

/// PickerMode variants should support equality comparison
#[test]
fn test_picker_mode_equality() {
    let a = PickerMode::File; // first mode instance
    let b = PickerMode::File; // second mode instance with same variant
    let c = PickerMode::Folder; // different variant

    assert_eq!(a, b);
    assert_ne!(a, c);
}

/// PickerMode should implement Clone and produce equal copies
#[test]
fn test_picker_mode_clone() {
    let original = PickerMode::ChoosingType; // mode to clone
    let cloned = original.clone(); // cloned copy

    assert_eq!(original, cloned);
}

/// PickerMode should implement Copy (value semantics)
#[test]
fn test_picker_mode_copy() {
    let mode = PickerMode::Inactive; // mode value
    let copied = mode; // copied via Copy trait

    // original is still usable after copy
    assert_eq!(mode, copied);
}

/// PickerMode should implement Debug for formatting
#[test]
fn test_picker_mode_debug() {
    let mode = PickerMode::File; // mode to format
    let debug_str = format!("{:?}", mode); // debug representation

    assert!(debug_str.contains("File"));
}

/// All four PickerMode variants exist and are distinct
#[test]
fn test_picker_mode_all_variants() {
    let inactive = PickerMode::Inactive;
    let choosing = PickerMode::ChoosingType;
    let file = PickerMode::File;
    let folder = PickerMode::Folder;

    assert_ne!(inactive, choosing);
    assert_ne!(choosing, file);
    assert_ne!(file, folder);
    assert_ne!(folder, inactive);
}

// ---------------------------------------------------------------------------
// Picker::new tests
// ---------------------------------------------------------------------------

/// New picker starts in Inactive mode with empty query
#[test]
fn test_picker_new_defaults() {
    let picker = Picker::new(); // freshly created picker

    assert_eq!(picker.mode(), PickerMode::Inactive);
    assert_eq!(picker.query(), "");
    assert_eq!(picker.selected_index(), 0);
    assert_eq!(picker.trigger_position(), 0);
}

/// New picker is not active
#[test]
fn test_picker_new_is_not_active() {
    let picker = Picker::new(); // freshly created picker

    assert!(!picker.is_active());
}

/// New picker has a non-zero last_scan_time (set to current epoch)
#[test]
fn test_picker_new_has_scan_time() {
    let picker = Picker::new(); // freshly created picker
    let scan_time = picker.last_scan_time(); // epoch seconds of last scan

    assert!(scan_time > 0);
}

// ---------------------------------------------------------------------------
// Picker::activate / deactivate tests
// ---------------------------------------------------------------------------

/// Activating the picker sets ChoosingType mode and stores trigger position
#[test]
fn test_picker_activate() {
    let mut picker = Picker::new(); // picker to activate
    let trigger_pos = 42; // cursor position where @ was typed

    picker.activate(trigger_pos);

    assert!(picker.is_active());
    assert_eq!(picker.mode(), PickerMode::ChoosingType);
    assert_eq!(picker.trigger_position(), trigger_pos);
    assert_eq!(picker.selected_index(), 0);
    assert_eq!(picker.query(), "");
}

/// Activating clears any previous query and resets selection
#[test]
fn test_picker_activate_clears_state() {
    let mut picker = Picker::new(); // picker with prior state

    // build up some state first
    picker.activate(10);
    picker.select_file_mode();
    picker.push_query('x');
    picker.move_down(5);

    // re-activate at new position
    picker.activate(99);

    assert_eq!(picker.query(), "");
    assert_eq!(picker.selected_index(), 0);
    assert_eq!(picker.trigger_position(), 99);
    assert_eq!(picker.mode(), PickerMode::ChoosingType);
}

/// Deactivating returns picker to Inactive mode
#[test]
fn test_picker_deactivate() {
    let mut picker = Picker::new(); // picker to deactivate

    picker.activate(5);
    picker.select_file_mode();
    picker.push_query('a');

    picker.deactivate();

    assert!(!picker.is_active());
    assert_eq!(picker.mode(), PickerMode::Inactive);
    assert_eq!(picker.query(), "");
    assert_eq!(picker.selected_index(), 0);
}

// ---------------------------------------------------------------------------
// Picker mode selection tests
// ---------------------------------------------------------------------------

/// select_file_mode sets File mode and clears query
#[test]
fn test_picker_select_file_mode() {
    let mut picker = Picker::new(); // picker to switch to file mode

    picker.activate(0);
    picker.push_query('z');
    picker.select_file_mode();

    assert_eq!(picker.mode(), PickerMode::File);
    assert_eq!(picker.query(), "");
    assert_eq!(picker.selected_index(), 0);
}

/// select_folder_mode sets Folder mode and clears query
#[test]
fn test_picker_select_folder_mode() {
    let mut picker = Picker::new(); // picker to switch to folder mode

    picker.activate(0);
    picker.push_query('z');
    picker.select_folder_mode();

    assert_eq!(picker.mode(), PickerMode::Folder);
    assert_eq!(picker.query(), "");
    assert_eq!(picker.selected_index(), 0);
}

// ---------------------------------------------------------------------------
// Picker query manipulation tests
// ---------------------------------------------------------------------------

/// push_query appends characters and resets selection
#[test]
fn test_picker_push_query() {
    let mut picker = Picker::new(); // picker for query testing

    picker.activate(0);
    picker.select_file_mode();

    picker.push_query('h');
    picker.push_query('e');
    picker.push_query('l');

    assert_eq!(picker.query(), "hel");
    assert_eq!(picker.selected_index(), 0);
}

/// pop_query removes the last character
#[test]
fn test_picker_pop_query() {
    let mut picker = Picker::new(); // picker for pop_query testing

    picker.activate(0);
    picker.select_file_mode();
    picker.push_query('a');
    picker.push_query('b');
    picker.push_query('c');

    picker.pop_query();

    assert_eq!(picker.query(), "ab");
}

/// pop_query on empty query does not panic
#[test]
fn test_picker_pop_query_on_empty() {
    let mut picker = Picker::new(); // picker with empty query

    picker.activate(0);
    picker.select_file_mode();

    // should not panic
    picker.pop_query();

    assert_eq!(picker.query(), "");
}

/// clear_query empties the query and resets selection
#[test]
fn test_picker_clear_query() {
    let mut picker = Picker::new(); // picker for clear_query testing

    picker.activate(0);
    picker.select_file_mode();
    picker.push_query('x');
    picker.push_query('y');

    picker.clear_query();

    assert_eq!(picker.query(), "");
    assert_eq!(picker.selected_index(), 0);
}

/// push_query resets selection index to 0
#[test]
fn test_picker_push_query_resets_selection() {
    let mut picker = Picker::new(); // picker to verify selection reset

    picker.activate(0);
    picker.select_file_mode();
    picker.move_down(10);
    picker.move_down(10);

    let idx_before = picker.selected_index(); // index after moving down

    picker.push_query('q');

    assert!(idx_before > 0);
    assert_eq!(picker.selected_index(), 0);
}

// ---------------------------------------------------------------------------
// Picker move_up / move_down tests
// ---------------------------------------------------------------------------

/// move_down increments the selected index within bounds
#[test]
fn test_picker_move_down() {
    let mut picker = Picker::new(); // picker for move_down testing
    let max_items = 5; // upper bound for selection

    picker.move_down(max_items);
    assert_eq!(picker.selected_index(), 1);

    picker.move_down(max_items);
    assert_eq!(picker.selected_index(), 2);
}

/// move_down does not exceed max_items - 1
#[test]
fn test_picker_move_down_clamps_at_max() {
    let mut picker = Picker::new(); // picker for boundary testing
    let max_items = 3; // list has 3 items (indices 0, 1, 2)

    picker.move_down(max_items);
    picker.move_down(max_items);
    picker.move_down(max_items); // should not go beyond 2
    picker.move_down(max_items); // still clamped

    assert_eq!(picker.selected_index(), 2);
}

/// move_down with max_items = 0 does nothing
#[test]
fn test_picker_move_down_zero_items() {
    let mut picker = Picker::new(); // picker with empty list

    picker.move_down(0);

    assert_eq!(picker.selected_index(), 0);
}

/// move_down with max_items = 1 does not move (already at last)
#[test]
fn test_picker_move_down_single_item() {
    let mut picker = Picker::new(); // picker with single-item list

    picker.move_down(1);

    assert_eq!(picker.selected_index(), 0);
}

/// move_up decrements the selected index
#[test]
fn test_picker_move_up() {
    let mut picker = Picker::new(); // picker for move_up testing

    picker.move_down(10);
    picker.move_down(10);
    assert_eq!(picker.selected_index(), 2);

    picker.move_up();
    assert_eq!(picker.selected_index(), 1);
}

/// move_up at index 0 stays at 0
#[test]
fn test_picker_move_up_clamps_at_zero() {
    let mut picker = Picker::new(); // picker already at top

    picker.move_up();

    assert_eq!(picker.selected_index(), 0);
}

/// move_up and move_down round-trip returns to original index
#[test]
fn test_picker_move_round_trip() {
    let mut picker = Picker::new(); // picker for round-trip testing

    picker.move_down(10);
    picker.move_down(10);
    picker.move_down(10);
    picker.move_up();
    picker.move_up();
    picker.move_up();

    assert_eq!(picker.selected_index(), 0);
}

// ---------------------------------------------------------------------------
// Picker::get_type_options tests
// ---------------------------------------------------------------------------

/// get_type_options returns exactly two options for folder and file
#[test]
fn test_picker_get_type_options_count() {
    let picker = Picker::new(); // picker for type options
    let options = picker.get_type_options(); // available type choices

    assert_eq!(options.len(), 2);
}

/// get_type_options contains folder and file labels
#[test]
fn test_picker_get_type_options_content() {
    let picker = Picker::new(); // picker for type options
    let options = picker.get_type_options(); // available type choices

    let has_folder = options.iter().any(|o| o.contains("folder"));
    let has_file = options.iter().any(|o| o.contains("file"));

    assert!(has_folder, "options should contain a folder entry");
    assert!(has_file, "options should contain a file entry");
}

// ---------------------------------------------------------------------------
// Picker::is_active edge cases
// ---------------------------------------------------------------------------

/// is_active returns true for all active modes
#[test]
fn test_picker_is_active_for_all_modes() {
    let mut picker = Picker::new(); // picker for mode testing

    // ChoosingType is active
    picker.activate(0);
    assert!(picker.is_active());

    // File mode is active
    picker.select_file_mode();
    assert!(picker.is_active());

    // Folder mode is active
    picker.select_folder_mode();
    assert!(picker.is_active());

    // Inactive is not active
    picker.deactivate();
    assert!(!picker.is_active());
}

// ---------------------------------------------------------------------------
// Picker Default trait test
// ---------------------------------------------------------------------------

/// Default trait produces same initial state as new()
#[test]
fn test_picker_default_matches_new() {
    let from_new = Picker::new(); // via new()
    let from_default = Picker::default(); // via Default trait

    assert_eq!(from_new.mode(), from_default.mode());
    assert_eq!(from_new.query(), from_default.query());
    assert_eq!(
        from_new.selected_index(),
        from_default.selected_index()
    );
    assert_eq!(
        from_new.trigger_position(),
        from_default.trigger_position()
    );
}
