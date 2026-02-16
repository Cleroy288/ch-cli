//! Unit tests for ui::layout — migrated from inline tests

use ratatui::layout::Rect;

use rustean::domain::{
    INPUT_BOX_HEIGHT, MAX_PICKER_HEIGHT, PICKER_WIDTH,
    TITLE_BOX_HEIGHT,
};
use rustean::ui::layout::{
    calculate_file_list_area, calculate_help_text_area,
    calculate_type_chooser_area, get_main_layout_constraints,
};

/// Test calculate_type_chooser_area positions below input
#[test]
fn test_calculate_type_chooser_area() {
    let input_area = Rect::new(10, 5, 80, 3);
    let picker_area = calculate_type_chooser_area(input_area);

    assert_eq!(picker_area.x, input_area.x);
    assert_eq!(picker_area.y, input_area.y + input_area.height);
    assert_eq!(picker_area.height, 6);
    assert!(picker_area.width <= PICKER_WIDTH);
}

/// Test calculate_type_chooser_area respects width
#[test]
fn test_calculate_type_chooser_area_width_limit() {
    let narrow_input = Rect::new(0, 0, 20, 3);
    let picker_area = calculate_type_chooser_area(narrow_input);

    assert_eq!(picker_area.width, 20); // Smaller than PICKER_WIDTH
}

/// Test calculate_file_list_area positions below input
#[test]
fn test_calculate_file_list_area() {
    let input_area = Rect::new(10, 5, 80, 3);
    let bottom_area = Rect::new(0, 8, 100, 20);
    let picker_area =
        calculate_file_list_area(input_area, bottom_area);

    assert_eq!(picker_area.x, input_area.x);
    assert_eq!(picker_area.y, input_area.y + input_area.height);
    assert_eq!(picker_area.width, input_area.width);
    assert!(picker_area.height > 0);
}

/// Test calculate_file_list_area respects max height
#[test]
fn test_calculate_file_list_area_max_height() {
    let input_area = Rect::new(0, 0, 80, 3);
    let large_bottom = Rect::new(0, 3, 100, 100);
    let picker_area =
        calculate_file_list_area(input_area, large_bottom);

    assert!(picker_area.height <= MAX_PICKER_HEIGHT + 2);
}

/// Test calculate_help_text_area positions below picker
#[test]
fn test_calculate_help_text_area() {
    let picker_area = Rect::new(10, 10, 60, 15);
    let help_area = calculate_help_text_area(picker_area);

    assert_eq!(help_area.x, picker_area.x + 2);
    assert_eq!(
        help_area.y,
        picker_area.y + picker_area.height
    );
    assert_eq!(help_area.height, 1);
    assert!(help_area.width <= picker_area.width);
}

/// Test get_main_layout_constraints returns valid values
#[test]
fn test_get_main_layout_constraints() {
    let (title_height, input_height) =
        get_main_layout_constraints();

    assert_eq!(title_height, TITLE_BOX_HEIGHT);
    assert_eq!(input_height, INPUT_BOX_HEIGHT);
    assert!(title_height > 0);
    assert!(input_height > 0);
}
