//! Tests for retrieval::docgen::linker

use ch_cli::retrieval::docgen::DocLinker;

#[test]
fn test_extract_crates() {
	let linker = DocLinker::new();

	let code = r#"
use serde::{Serialize, Deserialize};
use std::path::Path;
use tokio::runtime;
extern crate regex;
"#;

	let crates = linker.extract_crates(code);
	assert!(crates.contains(&"serde".to_string()));
	assert!(crates.contains(&"tokio".to_string()));
	assert!(crates.contains(&"regex".to_string()));
	assert!(!crates.contains(&"std".to_string()));
}
