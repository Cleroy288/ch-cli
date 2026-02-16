use rustean::retrieval::query::structure_search::{
	find_module_structure, list_source_directories,
};
use std::fs;

/// Test list_source_directories finds subdirectories
#[test]
fn test_list_source_directories() {
	let tmp_dir = std::env::temp_dir()
		.join("ch_test_list_src_dirs");
	let src_dir = tmp_dir.join("src");

	let _ = fs::remove_dir_all(&tmp_dir);
	fs::create_dir_all(src_dir.join("alpha")).unwrap();
	fs::create_dir_all(src_dir.join("beta")).unwrap();

	let dirs = list_source_directories(&tmp_dir);

	assert!(dirs.contains(&"alpha".to_string()));
	assert!(dirs.contains(&"beta".to_string()));
	assert_eq!(dirs[0], "alpha");
	assert_eq!(dirs[1], "beta");

	let _ = fs::remove_dir_all(&tmp_dir);
}

/// Test list_source_directories returns empty for
/// missing src/
#[test]
fn test_list_source_directories_no_src() {
	let tmp_dir =
		std::env::temp_dir().join("ch_test_no_src");

	let _ = fs::remove_dir_all(&tmp_dir);
	fs::create_dir_all(&tmp_dir).unwrap();

	let dirs = list_source_directories(&tmp_dir);

	assert!(dirs.is_empty());

	let _ = fs::remove_dir_all(&tmp_dir);
}

/// Test find_module_structure parses mod.rs
#[test]
fn test_find_module_structure() {
	let tmp_dir =
		std::env::temp_dir().join("ch_test_find_mod");
	let mod_dir = tmp_dir.join("src").join("mymod");

	let _ = fs::remove_dir_all(&tmp_dir);
	fs::create_dir_all(&mod_dir).unwrap();
	fs::write(
		mod_dir.join("mod.rs"),
		"/// Parser utilities\npub mod parser;\n\
		 mod internal;\npub use parser::Parser;\n",
	)
	.unwrap();

	let results =
		find_module_structure(&tmp_dir, "mymod");

	assert!(!results.is_empty());
	let info = &results[0];
	assert!(info.path.ends_with("mod.rs"));
	assert_eq!(info.submodules.len(), 2);

	let pub_mod = info
		.submodules
		.iter()
		.find(|s| s.name == "parser")
		.unwrap();
	assert!(pub_mod.is_public);
	assert!(pub_mod.doc.is_some());

	let priv_mod = info
		.submodules
		.iter()
		.find(|s| s.name == "internal")
		.unwrap();
	assert!(!priv_mod.is_public);

	assert_eq!(info.reexports.len(), 1);
	assert!(info.reexports[0].contains("Parser"));

	let _ = fs::remove_dir_all(&tmp_dir);
}
