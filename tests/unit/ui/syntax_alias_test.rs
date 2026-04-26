//! Tests for syntax highlighting aliases.

use ratatui::style::Color;

use rustean::ui::markdown::render_markdown;

/// TypeScript code block has colored keywords
#[test]
fn typescript_block_has_colored_keywords() {
	// Arrange
	let md =
		"```typescript\nfunction greet() {}\n```";

	// Act
	let lines = render_markdown(md);

	// Assert — code line (index 1) has RGB colors
	let code_spans = &lines[1].spans;
	let has_rgb = code_spans.iter().any(|span| {
		matches!(span.style.fg, Some(Color::Rgb(..)))
	});
	assert!(has_rgb, "expected RGB-colored spans");

	// "function" keyword should differ from default
	let colors: Vec<_> = code_spans
		.iter()
		.filter_map(|span| span.style.fg)
		.collect();
	let unique: std::collections::HashSet<_> =
		colors.iter().collect();
	assert!(
		unique.len() > 1,
		"expected multiple colors, got {:?}",
		colors,
	);
}

/// "ts" fence tag also gets JavaScript highlighting
#[test]
fn ts_fence_gets_js_highlighting() {
	// Arrange
	let md = "```ts\nconst x = 42;\n```";

	// Act
	let lines = render_markdown(md);

	// Assert — multiple distinct colors
	let code_spans = &lines[1].spans;
	let colors: Vec<_> = code_spans
		.iter()
		.filter_map(|span| span.style.fg)
		.collect();
	let unique: std::collections::HashSet<_> =
		colors.iter().collect();
	assert!(
		unique.len() > 1,
		"expected multiple colors for ts, got {:?}",
		colors,
	);
}

/// PHP uses "PHP Source" syntax (no `<?php` needed)
#[test]
fn php_code_has_colored_keywords() {
	// Arrange
	let md = "```php\nclass User {\n    private $name;\n}\n```";

	// Act
	let lines = render_markdown(md);

	// Assert — keywords get distinct colors
	let code_spans = &lines[1].spans;
	let colors: Vec<_> = code_spans
		.iter()
		.filter_map(|span| span.style.fg)
		.collect();
	let unique: std::collections::HashSet<_> =
		colors.iter().collect();
	assert!(
		unique.len() > 1,
		"expected multiple colors for php, got {:?}",
		colors,
	);
}

/// Rust code still works (no alias needed)
#[test]
fn rust_code_has_colored_keywords() {
	// Arrange
	let md = "```rust\nfn main() {}\n```";

	// Act
	let lines = render_markdown(md);

	// Assert — has distinct colors
	let code_spans = &lines[1].spans;
	let colors: Vec<_> = code_spans
		.iter()
		.filter_map(|span| span.style.fg)
		.collect();
	let unique: std::collections::HashSet<_> =
		colors.iter().collect();
	assert!(
		unique.len() > 1,
		"expected multiple colors for rust",
	);
}
