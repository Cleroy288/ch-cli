//! Unit tests for ui::styles — migrated from inline tests

use ratatui::style::{Color, Modifier, Style};

use ch_cli::ui::styles::colors;
use ch_cli::ui::styles::{
    file_list_selected_style, file_reference_style,
    folder_reference_style, picker_selected_style,
};

/// Test file_reference_style returns correct colors
#[test]
fn test_file_reference_style() {
    let style = file_reference_style();

    assert_ne!(style, Style::default());
    assert_eq!(style.fg, Some(Color::White));
    assert_eq!(style.bg, Some(colors::FILE_REF_BG));
    assert!(style.add_modifier.contains(Modifier::BOLD));
}

/// Test folder_reference_style returns correct colors
#[test]
fn test_folder_reference_style() {
    let style = folder_reference_style();

    assert_ne!(style, Style::default());
    assert_eq!(style.fg, Some(Color::White));
    assert_eq!(style.bg, Some(colors::FOLDER_REF_BG));
    assert!(style.add_modifier.contains(Modifier::BOLD));
}

/// Test picker_selected_style returns correct colors
#[test]
fn test_picker_selected_style() {
    let style = picker_selected_style();

    assert_ne!(style, Style::default());
    assert_eq!(style.fg, Some(Color::Black));
    assert_eq!(style.bg, Some(colors::TITLE));
    assert!(style.add_modifier.contains(Modifier::BOLD));
}

/// Test file_list_selected_style returns correct colors
#[test]
fn test_file_list_selected_style() {
    let style = file_list_selected_style();

    assert_ne!(style, Style::default());
    assert_eq!(style.fg, Some(Color::Black));
    assert_eq!(style.bg, Some(colors::PICKER));
    assert!(style.add_modifier.contains(Modifier::BOLD));
}
