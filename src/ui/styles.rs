use ratatui::style::{Color, Modifier, Style};

/// Color constants for consistent UI theming
///
/// Rust-inspired dark orange palette with
/// monochrome accents for a tech aesthetic.
pub mod colors {
	use super::Color;

	/// Rust orange — primary brand color
	pub const RUST_ORANGE: Color =
		Color::Rgb(183, 65, 14);

	/// Darker orange for secondary elements
	pub const DARK_ORANGE: Color =
		Color::Rgb(140, 50, 10);

	/// Muted amber for highlights
	pub const AMBER: Color =
		Color::Rgb(200, 120, 40);

	/// Primary accent for titles
	pub const TITLE: Color = RUST_ORANGE;

	/// Color for normal input text
	pub const INPUT_TEXT: Color = AMBER;

	/// Background color for file references
	pub const FILE_REF_BG: Color = DARK_ORANGE;

	/// Background color for folder references
	pub const FOLDER_REF_BG: Color =
		Color::Rgb(80, 80, 80);

	/// Color for inactive/placeholder text
	pub const PLACEHOLDER: Color = Color::DarkGray;

	/// Color for picker UI elements
	pub const PICKER: Color = AMBER;

	/// Color for debug panel
	pub const DEBUG: Color = Color::DarkGray;

	/// Color for message headers
	pub const MESSAGE_HEADER: Color = AMBER;

	/// Color for segment labels
	pub const SEGMENT_LABEL: Color = Color::Gray;

	/// Color for full paths in debug view
	pub const PATH_DISPLAY: Color = Color::Gray;

	/// Color for arrows and separators
	pub const SEPARATOR: Color = Color::DarkGray;

	/// Border color for panels
	pub const BORDER: Color = Color::Rgb(60, 60, 60);

	/// Subtle text for secondary info
	pub const DIM_TEXT: Color =
		Color::Rgb(100, 100, 100);
}

/// Style for file references (bold on dark orange bg)
pub fn file_reference_style() -> Style {
	Style::default()
		.fg(Color::White)
		.bg(colors::FILE_REF_BG)
		.add_modifier(Modifier::BOLD)
}

/// Style for folder references (bold on gray bg)
pub fn folder_reference_style() -> Style {
	Style::default()
		.fg(Color::White)
		.bg(colors::FOLDER_REF_BG)
		.add_modifier(Modifier::BOLD)
}

/// Style for selected picker items
pub fn picker_selected_style() -> Style {
	Style::default()
		.fg(Color::Black)
		.bg(colors::RUST_ORANGE)
		.add_modifier(Modifier::BOLD)
}

/// Style for selected file list items
pub fn file_list_selected_style() -> Style {
	Style::default()
		.fg(Color::Black)
		.bg(colors::PICKER)
		.add_modifier(Modifier::BOLD)
}

/// Display a goodbye message after terminal restore
pub fn show_goodbye_message() {
	use std::io::Write;
	let mut out = std::io::stdout();
	let _ = writeln!(out);
	let _ = writeln!(out);
	let _ = writeln!(
		out,
		"  \x1b[38;2;183;65;14m\u{2500}\u{2500}\u{2500} \
		rustean \u{2500}\u{2500}\u{2500}\x1b[0m"
	);
	let _ = writeln!(
		out,
		"  \x1b[38;2;100;100;100m\
		See you next time.\x1b[0m"
	);
	let _ = writeln!(out);
}
