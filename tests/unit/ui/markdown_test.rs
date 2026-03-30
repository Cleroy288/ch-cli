//! Tests for the custom markdown renderer.

use ratatui::style::{Color, Modifier};

use rustean::ui::markdown::inline;
use rustean::ui::markdown::render_markdown;

/// Helper: extract text from lines
fn lines_text(
	lines: &[ratatui::text::Line<'static>],
) -> Vec<String> {
	lines
		.iter()
		.map(|line| {
			line.spans
				.iter()
				.map(|span| span.content.to_string())
				.collect::<String>()
		})
		.collect()
}

/// Bold text produces bold-styled span
#[test]
fn inline_bold_produces_bold_span() {
	// Arrange / Act
	let spans = inline::parse("hello **world**");

	// Assert
	assert_eq!(spans.len(), 2);
	assert_eq!(spans[0].content.to_string(), "hello ");
	assert_eq!(spans[1].content.to_string(), "world");
	assert!(spans[1]
		.style
		.add_modifier
		.contains(Modifier::BOLD));
}

/// Italic text produces italic-styled span
#[test]
fn inline_italic_produces_italic_span() {
	// Arrange / Act
	let spans = inline::parse("say *hi* now");

	// Assert
	assert_eq!(spans.len(), 3);
	assert_eq!(spans[1].content.to_string(), "hi");
	assert!(spans[1]
		.style
		.add_modifier
		.contains(Modifier::ITALIC));
}

/// Inline code gets code styling
#[test]
fn inline_code_gets_code_style() {
	// Arrange / Act
	let spans = inline::parse("use `Vec<T>` here");

	// Assert
	assert_eq!(spans.len(), 3);
	assert_eq!(spans[1].content.to_string(), "Vec<T>");
	assert_eq!(spans[1].style.fg, Some(Color::White));
}

/// Plain text without markers returns one span
#[test]
fn inline_plain_text_returns_single_span() {
	// Arrange / Act
	let spans = inline::parse("no markers here");

	// Assert
	assert_eq!(spans.len(), 1);
	assert_eq!(
		spans[0].content.to_string(),
		"no markers here"
	);
}

/// Unclosed bold marker treated as plain text
#[test]
fn inline_unclosed_bold_is_plain() {
	// Arrange / Act
	let spans = inline::parse("open **bold");

	// Assert
	let text: String = spans
		.iter()
		.map(|span| span.content.to_string())
		.collect();
	assert_eq!(text, "open **bold");
}

/// Heading strips hash marks from output
#[test]
fn heading_strips_hash_marks() {
	// Arrange / Act
	let lines = render_markdown("### Core Tools");
	let texts = lines_text(&lines);

	// Assert
	assert_eq!(texts.len(), 1);
	assert_eq!(texts[0], "  Core Tools");
}

/// H1 heading has white + bold
#[test]
fn heading_h1_style() {
	// Arrange / Act
	let lines = render_markdown("# Title");

	// Assert
	let span = &lines[0].spans[1];
	assert_eq!(span.style.fg, Some(Color::White));
	assert!(span
		.style
		.add_modifier
		.contains(Modifier::BOLD));
}

/// Table renders header and data rows
#[test]
fn table_renders_with_alignment() {
	// Arrange
	let md = "| Tool  | Purpose    |\n\
	           |-------|------------|\n\
	           | Read  | Read files |\n\
	           | Write | New files  |";

	// Act
	let lines = render_markdown(md);
	let texts = lines_text(&lines);

	// Assert — header, separator, 2 data rows
	assert_eq!(texts.len(), 4);
	assert!(texts[0].contains("Tool"));
	assert!(texts[0].contains("Purpose"));
	assert!(texts[1].contains("\u{2500}")); // ─
	assert!(texts[2].contains("Read"));
	assert!(texts[3].contains("Write"));
}

/// Header row is bold, data rows are not
#[test]
fn table_header_is_bold() {
	// Arrange
	let md = "| A |\n|---|\n| B |";

	// Act
	let lines = render_markdown(md);

	// Assert
	let header_style = lines[0].spans[1].style;
	let data_style = lines[2].spans[1].style;
	assert!(header_style
		.add_modifier
		.contains(Modifier::BOLD));
	assert!(!data_style
		.add_modifier
		.contains(Modifier::BOLD));
}

/// Code block hides raw backticks
#[test]
fn code_block_hides_backtick_fences() {
	// Arrange
	let md = "```rust\nfn main() {}\n```";

	// Act
	let lines = render_markdown(md);
	let texts = lines_text(&lines);

	// Assert — open + code + close + spacing
	assert!(texts.len() >= 3);
	assert!(texts[0].contains("rust"));
	assert!(!texts[0].contains("```"));
	assert!(texts[1].contains("fn main()"));
	assert!(texts[2].contains("\u{2514}"));
}

/// Code block shows language label in accent bold
#[test]
fn code_block_shows_language_label() {
	// Arrange / Act
	let lines =
		render_markdown("```python\npass\n```");
	let texts = lines_text(&lines);

	// Assert — separator with language name
	assert!(texts[0].contains("python"));
	assert!(texts[0].contains("\u{2500}"));
	let label = &lines[0].spans[2];
	assert_eq!(
		label.style.fg,
		Some(Color::Rgb(180, 190, 254)),
	);
	assert!(label
		.style
		.add_modifier
		.contains(Modifier::BOLD));
}

/// Code block content has syntax-highlighted color
#[test]
fn code_block_content_is_highlighted() {
	// Arrange
	let md = "```\nlet x = 1;\n```";

	// Act
	let lines = render_markdown(md);

	// Assert — middle line has gutter + content
	let spans = &lines[1].spans;
	assert!(spans.len() >= 2); // gutter + content
	assert!(spans[1].content.contains("\u{2502}"));
	// Gutter has DarkGray fg
	assert!(spans[1].style.fg.is_some());
}

/// Bullet list renders with bullet marker
#[test]
fn bullet_list_renders_with_marker() {
	// Arrange
	let md = "- First item\n- Second item";

	// Act
	let lines = render_markdown(md);
	let texts = lines_text(&lines);

	// Assert
	assert_eq!(texts.len(), 2);
	assert!(texts[0].contains("\u{2022}")); // •
	assert!(texts[0].contains("First item"));
	assert!(texts[1].contains("Second item"));
}

/// Numbered list preserves numbers
#[test]
fn numbered_list_preserves_numbers() {
	// Arrange
	let md = "1. Alpha\n2. Beta\n3. Gamma";

	// Act
	let lines = render_markdown(md);
	let texts = lines_text(&lines);

	// Assert
	assert_eq!(texts.len(), 3);
	assert!(texts[0].contains("1."));
	assert!(texts[0].contains("Alpha"));
	assert!(texts[1].contains("2."));
	assert!(texts[2].contains("3."));
}

/// List items support inline formatting
#[test]
fn list_item_supports_inline_bold() {
	// Arrange / Act
	let lines = render_markdown("- **bold** item");

	// Assert — has bold span in list item
	let has_bold = lines[0].spans.iter().any(|span| {
		span.style.add_modifier.contains(Modifier::BOLD)
	});
	assert!(has_bold);
}

/// Blockquote renders with left bar
#[test]
fn blockquote_renders_with_bar() {
	// Arrange / Act
	let lines = render_markdown("> Quoted text");
	let texts = lines_text(&lines);

	// Assert
	assert_eq!(texts.len(), 1);
	assert!(texts[0].contains("\u{2502}")); // │
	assert!(texts[0].contains("Quoted text"));
}

/// Blockquote text is italic
#[test]
fn blockquote_text_is_italic() {
	// Arrange / Act
	let lines = render_markdown("> Note here");

	// Assert — quoted text span has italic
	let has_italic = lines[0].spans.iter().any(|span| {
		span.content.contains("Note")
			&& span
				.style
				.add_modifier
				.contains(Modifier::ITALIC)
	});
	assert!(has_italic);
}

/// Horizontal rules are stripped (not rendered)
#[test]
fn horizontal_rule_is_stripped() {
	// Arrange / Act
	let lines = render_markdown("---");

	// Assert — rule produces no output
	assert!(lines.is_empty());
}

/// Star rules are also stripped
#[test]
fn star_rule_also_stripped() {
	// Arrange / Act
	let lines = render_markdown("***");

	// Assert — rule produces no output
	assert!(lines.is_empty());
}

/// Mixed markdown renders all block types
#[test]
fn mixed_markdown_renders_all_types() {
	// Arrange
	let md = "### Heading\n\n\
	           Some **bold** text.\n\n\
	           | A | B |\n|---|---|\n| x | y |\n\n\
	           ```\ncode\n```\n\n\
	           - list item\n\n\
	           > quote\n\n\
	           ---";

	// Act
	let lines = render_markdown(md);
	let texts = lines_text(&lines);
	let joined = texts.join("\n");

	// Assert
	assert!(joined.contains("Heading"));
	assert!(joined.contains("bold"));
	assert!(joined.contains("code"));
	assert!(joined.contains("\u{2022}")); // bullet
	assert!(joined.contains("\u{2502}")); // quote bar
}

/// Empty input returns empty output
#[test]
fn empty_input_returns_empty() {
	// Arrange / Act
	let lines = render_markdown("");

	// Assert
	assert!(lines.is_empty());
}
