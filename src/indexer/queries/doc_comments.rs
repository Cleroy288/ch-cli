/// Query for extracting doc comments (separate query for efficiency).
pub const RUST_DOC_COMMENTS_QUERY: &str = r#"
; Line doc comments
(line_comment) @doc.line

; Block doc comments
(block_comment) @doc.block
"#;
