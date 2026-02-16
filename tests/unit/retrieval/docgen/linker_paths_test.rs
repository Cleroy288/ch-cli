//! Tests for retrieval::docgen::linker_paths

use std::path::Path;

use rustean::retrieval::docgen::linker_paths::get_module_path;

#[test]
fn test_get_module_path() {
	assert_eq!(
		get_module_path(Path::new(
			"src/retrieval/docgen/entry.rs"
		)),
		"retrieval::docgen::entry"
	);
	assert_eq!(
		get_module_path(Path::new("src/main.rs")),
		"main"
	);
	assert_eq!(
		get_module_path(Path::new("src/lib.rs")),
		"lib"
	);
}
