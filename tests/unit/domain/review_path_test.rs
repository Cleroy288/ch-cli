//! Tests for domain::review_path -- file path
//! inference from preceding text.

use rustean::domain::review::BlockIdx;
use rustean::domain::review_path::infer_file_path;
use rustean::domain::review_path
	::extract_review_blocks;

/// infer_file_path: single-line format cases
#[test]
fn infer_file_path_single_line_cases() {
	let cases: &[(&str, Option<&str>)] = &[
		(
			"\u{250C}\u{2500} src/main.rs",
			Some("src/main.rs"),
		),
		("### src/lib.rs", Some("src/lib.rs")),
		(
			"file `src/app/mod.rs` here",
			Some("src/app/mod.rs"),
		),
		(
			"Modify src/handler/auth.rs",
			Some("src/handler/auth.rs"),
		),
		("This has no file path", None),
		("", None),
		(
			"Update **src/main.rs** below",
			Some("src/main.rs"),
		),
		(
			"[MODIFIED] src/handler/auth.rs",
			Some("src/handler/auth.rs"),
		),
	];
	for (input, expected) in cases {
		let result = infer_file_path(input);
		let expected_owned =
			expected.map(|s| s.to_string());
		assert_eq!(
			result, expected_owned,
			"infer_file_path({input:?})",
		);
	}
}

#[test]
fn finds_path_through_explanation_lines() {
	let text = "\
\u{250C}\u{2500} src/main.rs\n\
\u{2502}\n\
\u{251C}\u{2500} What    : Added handler\n\
\u{251C}\u{2500} Why     : Needed for auth\n\
\u{2502}\n\
\u{2514}\u{2500} Diff";
	let result = infer_file_path(text);
	assert_eq!(
		result, Some("src/main.rs".to_string()),
	);
}

#[test]
fn finds_path_with_indented_box_drawing() {
	let text = "\
  \u{250C}\u{2500} src/app/mod.rs\n\
  \u{2502}\n\
  \u{2514}\u{2500} Diff";
	let result = infer_file_path(text);
	assert_eq!(
		result,
		Some("src/app/mod.rs".to_string()),
	);
}

#[test]
fn no_code_blocks_returns_empty() {
	let resp = "Just some text, no code.";
	let blocks = extract_review_blocks(resp);
	assert!(blocks.is_empty());
}

#[test]
fn single_code_block_extracted() {
	let resp = "Here is code:\n\
		```rust\nfn main() {}\n```\nDone.";
	let blocks = extract_review_blocks(resp);
	assert_eq!(blocks.len(), 1);
	assert_eq!(blocks[0].lang, "rust");
	assert_eq!(blocks[0].edited, "fn main() {}");
}

#[test]
fn block_with_box_drawing_format() {
	let resp = "\
\u{250C}\u{2500} src/main.rs\n\
\u{2502}\n\
\u{251C}\u{2500} What : Added main\n\
\u{2502}\n\
\u{2514}\u{2500} Diff\n\
```rust\nfn main() {}\n```";
	let blocks = extract_review_blocks(resp);
	assert_eq!(blocks.len(), 1);
	assert_eq!(
		blocks[0].file_path,
		Some("src/main.rs".to_string()),
	);
}

#[test]
fn multiple_blocks_with_paths() {
	let resp = "\u{250C}\u{2500} src/a.rs\n\
		```rust\nlet a = 1;\n```\n\
		\u{250C}\u{2500} src/b.rs\n\
		```rust\nlet b = 2;\n```";
	let blocks = extract_review_blocks(resp);
	assert_eq!(blocks.len(), 2);
	assert_eq!(
		blocks[0].file_path,
		Some("src/a.rs".to_string()),
	);
	assert_eq!(
		blocks[1].file_path,
		Some("src/b.rs".to_string()),
	);
}

#[test]
fn block_without_path_has_none() {
	let resp = "No path here\n\
		```js\nconsole.log('hi')\n```";
	let blocks = extract_review_blocks(resp);
	assert_eq!(blocks.len(), 1);
	assert_eq!(blocks[0].file_path, None);
}

#[test]
fn block_index_increments() {
	let resp = "```rs\na\n```\n```rs\nb\n```";
	let blocks = extract_review_blocks(resp);
	assert_eq!(blocks[0].index, BlockIdx(0));
	assert_eq!(blocks[1].index, BlockIdx(1));
}
