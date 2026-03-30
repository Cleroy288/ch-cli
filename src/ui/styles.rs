use ratatui::style::{Color, Modifier, Style};

pub mod colors {
	use super::Color;

	pub const ACCENT: Color =
		Color::Rgb(180, 190, 254);
	pub const ACCENT_DIM: Color =
		Color::Rgb(137, 180, 250);
	pub const TEXT_LIGHT: Color =
		Color::Rgb(205, 214, 244);
	pub const TOOL_BLUE: Color =
		Color::Rgb(137, 180, 250);
	pub const STATUS_DIM: Color =
		Color::Rgb(127, 132, 156);
	pub const TITLE: Color = ACCENT;
	pub const INPUT_TEXT: Color = TEXT_LIGHT;
	pub const FILE_REF_BG: Color =
		Color::Rgb(50, 50, 80);
	pub const FOLDER_REF_BG: Color =
		Color::Rgb(45, 50, 65);
	pub const PLACEHOLDER: Color =
		Color::Rgb(88, 91, 112);
	pub const PICKER: Color = TEXT_LIGHT;
	pub const DEBUG: Color = Color::DarkGray;
	pub const MESSAGE_HEADER: Color = ACCENT_DIM;
	pub const SEGMENT_LABEL: Color = Color::Gray;
	pub const PATH_DISPLAY: Color = Color::Gray;
	pub const SEPARATOR: Color =
		Color::Rgb(69, 71, 90);
	pub const BORDER: Color =
		Color::Rgb(49, 50, 68);
	pub const TOOL_NAME: Color = Color::White;
	pub const DIM_TEXT: Color =
		Color::Rgb(108, 112, 134);
	pub const INLINE_CODE_BG: Color =
		Color::Rgb(30, 30, 46);
	pub const SUCCESS: Color =
		Color::Rgb(166, 227, 161);
	pub const USER_LABEL: Color =
		Color::Rgb(249, 226, 175);
	pub const BLOCKQUOTE_BAR: Color = SEPARATOR;
	pub const FOCUS: Color = Color::Yellow;
	pub const JIRA_IN_PROGRESS: Color =
		Color::Rgb(66, 133, 244);
}

pub fn file_reference_style() -> Style {
	Style::default()
		.fg(Color::White)
		.bg(colors::FILE_REF_BG)
		.add_modifier(Modifier::BOLD)
}

pub fn folder_reference_style() -> Style {
	Style::default()
		.fg(Color::White)
		.bg(colors::FOLDER_REF_BG)
		.add_modifier(Modifier::BOLD)
}

pub fn picker_selected_style() -> Style {
	Style::default()
		.fg(Color::Black)
		.bg(colors::ACCENT)
		.add_modifier(Modifier::BOLD)
}

pub fn file_list_selected_style() -> Style {
	Style::default()
		.fg(Color::Black)
		.bg(colors::PICKER)
		.add_modifier(Modifier::BOLD)
}

pub fn show_goodbye_message() {
	use std::io::Write;
	let mut out = std::io::stdout();
	let _ = writeln!(out);
	let _ = writeln!(
		out,
		"  \x1b[38;2;180;190;254m\u{2500}\u{2500}\u{2500} \
		rustean \u{2500}\u{2500}\u{2500}\x1b[0m"
	);
	let _ = writeln!(
		out,
		"  \x1b[38;2;108;112;134m\
		See you next time.\x1b[0m"
	);
	let _ = writeln!(out);
}
