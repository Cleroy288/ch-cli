use ratatui::layout::Rect;

use crate::domain::{
    INPUT_BOX_HEIGHT, MAX_PICKER_HEIGHT, PICKER_WIDTH,
    TITLE_BOX_HEIGHT,
};

/// Calculate the area for the type chooser picker overlay.
///
/// Positions the picker right below the input box with a fixed width.
pub fn calculate_type_chooser_area(input_area: Rect) -> Rect {
    Rect {
        x: input_area.x,
        y: input_area.y + input_area.height,
        width: input_area.width.min(PICKER_WIDTH),
        // Fixed height for type chooser
        // (2 options + borders + title)
        height: 6,
    }
}

/// Calculate the area for the file/folder list picker overlay.
///
/// Positions the picker below the input box, using most of the bottom area.
/// Respects the maximum picker height constant.
pub fn calculate_file_list_area(input_area: Rect, bottom_area: Rect) -> Rect {
    let max_height = bottom_area
        .height
        .saturating_sub(2)
        .min(MAX_PICKER_HEIGHT);
    Rect {
        x: input_area.x,
        y: input_area.y + input_area.height,
        width: input_area.width,
        height: max_height + 2, // +2 for borders
    }
}

/// Calculate the area for help text below the picker.
pub fn calculate_help_text_area(picker_area: Rect) -> Rect {
    Rect {
        x: picker_area.x + 2,
        y: picker_area.y + picker_area.height,
        width: picker_area.width.saturating_sub(4),
        height: 1,
    }
}

/// Get the main UI layout constraints.
///
/// Returns a tuple of (title_height, input_height) using domain constants.
pub fn get_main_layout_constraints() -> (u16, u16) {
    (TITLE_BOX_HEIGHT, INPUT_BOX_HEIGHT)
}

