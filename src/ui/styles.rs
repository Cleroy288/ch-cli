use ratatui::style::{Color, Modifier, Style};

/// Color constants for consistent UI theming
pub mod colors {
    use super::Color;

    /// Primary accent color for titles and type chooser
    pub const TITLE: Color = Color::Cyan;

    /// Color for normal input text
    pub const INPUT_TEXT: Color = Color::Yellow;

    /// Background color for file references
    pub const FILE_REF_BG: Color = Color::Green;

    /// Background color for folder references
    pub const FOLDER_REF_BG: Color = Color::Cyan;

    /// Color for inactive/placeholder text
    pub const PLACEHOLDER: Color = Color::DarkGray;

    /// Color for picker UI elements
    pub const PICKER: Color = Color::Green;

    /// Color for debug panel
    pub const DEBUG: Color = Color::Magenta;

    /// Color for message headers
    pub const MESSAGE_HEADER: Color = Color::Yellow;

    /// Color for segment labels (Text, File, Folder)
    pub const SEGMENT_LABEL: Color = Color::Gray;

    /// Color for full paths in debug view
    pub const PATH_DISPLAY: Color = Color::Gray;

    /// Color for arrows and separators
    pub const SEPARATOR: Color = Color::DarkGray;
}

/// Create a style for file references (white text on green background, bold)
pub fn file_reference_style() -> Style {
    Style::default()
        .fg(Color::White)
        .bg(colors::FILE_REF_BG)
        .add_modifier(Modifier::BOLD)
}

/// Create a style for folder references (white text on cyan background, bold)
pub fn folder_reference_style() -> Style {
    Style::default()
        .fg(Color::White)
        .bg(colors::FOLDER_REF_BG)
        .add_modifier(Modifier::BOLD)
}

/// Create a style for selected picker items (black text on cyan background, bold)
pub fn picker_selected_style() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(colors::TITLE)
        .add_modifier(Modifier::BOLD)
}

/// Create a style for selected file list items (black text on green background, bold)
pub fn file_list_selected_style() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(colors::PICKER)
        .add_modifier(Modifier::BOLD)
}

/// Display a goodbye message after the terminal is restored.
///
/// This is the only non-pure function in this module, as it performs I/O.
/// It's kept here for convenience since it's UI-related.
pub fn show_goodbye_message() {
    println!("\n");
    println!("╔═══════════════════════════════════════════╗");
    println!("║                                           ║");
    println!("║          Thanks for using tcah!           ║");
    println!("║                                           ║");
    println!("║            See you next time!             ║");
    println!("║                                           ║");
    println!("╚═══════════════════════════════════════════╝");
    println!("\n");
}
