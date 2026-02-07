//! Tests for IndexManager indexing operations.

use std::fs;
use std::io::Write;

use tempfile::TempDir;

use ch_cli::indexer::manager::IndexManager;
use ch_cli::indexer::SymbolKind;

fn create_test_project() -> TempDir {
	let temp_dir = TempDir::new().expect("Failed to create temp dir");

	// Create a simple Rust file
	let src_dir = temp_dir.path().join("src");
	fs::create_dir_all(&src_dir).expect("Failed to create src dir");

	let lib_rs = src_dir.join("lib.rs");
	let mut file = fs::File::create(&lib_rs)
		.expect("Failed to create lib.rs");
	writeln!(
		file,
		r#"
pub struct MyStruct {{
    pub field: i32,
}}

impl MyStruct {{
    pub fn new() -> Self {{
        Self {{ field: 0 }}
    }}

    pub fn get_field(&self) -> i32 {{
        self.field
    }}
}}

pub fn helper_function() -> i32 {{
    42
}}

pub enum Status {{
    Active,
    Inactive,
}}
"#
	)
	.expect("Failed to write lib.rs");

	// Create another file
	let main_rs = src_dir.join("main.rs");
	let mut file = fs::File::create(&main_rs)
		.expect("Failed to create main.rs");
	writeln!(
		file,
		r#"
mod lib;

fn main() {{
    println!("Hello");
}}
"#
	)
	.expect("Failed to write main.rs");

	temp_dir
}

#[test]
fn test_index_project_basic() {
	let temp_dir = create_test_project();

	let manager = IndexManager::new();
	let result = manager.index_project(temp_dir.path()).unwrap();

	assert_eq!(result.stats.files_found, 2);
	assert_eq!(result.stats.files_parsed, 2);
	assert_eq!(result.stats.files_failed, 0);
	assert!(result.stats.symbols_found > 0);
	assert!(result.semantic_graph.is_none()); // Not enabled
}

#[test]
fn test_index_project_with_semantic_analysis() {
	let temp_dir = create_test_project();

	let manager = IndexManager::new().with_semantic_analysis();
	let result = manager.index_project(temp_dir.path()).unwrap();

	assert!(result.semantic_graph.is_some());

	let graph = result.semantic_graph.as_ref().unwrap();
	let stats = graph.stats();

	assert!(stats.total_definitions > 0);
	assert!(stats.unique_symbols > 0);
	assert_eq!(stats.files_analyzed, 2);
}

#[test]
fn test_semantic_graph_finds_struct() {
	let temp_dir = create_test_project();

	let manager = IndexManager::new().with_semantic_analysis();
	let result = manager.index_project(temp_dir.path()).unwrap();

	let graph = result.semantic_graph.as_ref().unwrap();

	// Find the struct we defined (may also find impl block reference)
	let defs = graph.find_definitions("MyStruct");
	assert!(!defs.is_empty());
	// At least one should be a Struct
	assert!(
		defs.iter().any(|d| d.symbol.kind == SymbolKind::Struct)
	);
}

#[test]
fn test_semantic_graph_finds_methods() {
	let temp_dir = create_test_project();

	let manager = IndexManager::new().with_semantic_analysis();
	let result = manager.index_project(temp_dir.path()).unwrap();

	let graph = result.semantic_graph.as_ref().unwrap();

	// Find the "new" method
	let defs = graph.find_definitions("new");
	assert!(!defs.is_empty());

	// Find the "get_field" method
	let defs = graph.find_definitions("get_field");
	assert!(!defs.is_empty());
}

#[test]
fn test_semantic_graph_finds_enum() {
	let temp_dir = create_test_project();

	let manager = IndexManager::new().with_semantic_analysis();
	let result = manager.index_project(temp_dir.path()).unwrap();

	let graph = result.semantic_graph.as_ref().unwrap();

	// Find the enum
	let defs = graph.find_definitions("Status");
	assert_eq!(defs.len(), 1);
	assert_eq!(defs[0].symbol.kind, SymbolKind::Enum);

	// Find enum variants
	let active = graph.find_definitions("Active");
	assert!(!active.is_empty());

	let inactive = graph.find_definitions("Inactive");
	assert!(!inactive.is_empty());
}

#[test]
fn test_semantic_graph_find_by_kind() {
	let temp_dir = create_test_project();

	let manager = IndexManager::new().with_semantic_analysis();
	let result = manager.index_project(temp_dir.path()).unwrap();

	let graph = result.semantic_graph.as_ref().unwrap();

	// Find all structs
	let structs = graph.find_by_kind(SymbolKind::Struct);
	assert!(
		structs.iter().any(|d| d.symbol.name == "MyStruct")
	);

	// Find all enums
	let enums = graph.find_by_kind(SymbolKind::Enum);
	assert!(enums.iter().any(|d| d.symbol.name == "Status"));

	// Find all functions
	let functions = graph.find_by_kind(SymbolKind::Function);
	assert!(
		functions
			.iter()
			.any(|d| d.symbol.name == "helper_function")
	);
}

#[test]
fn test_index_result_debug() {
	let temp_dir = create_test_project();

	let manager = IndexManager::new().with_semantic_analysis();
	let result = manager.index_project(temp_dir.path()).unwrap();

	// Test that Debug trait works
	let debug_str = format!("{:?}", result);
	assert!(debug_str.contains("IndexResult"));
	assert!(debug_str.contains("semantic_graph"));
}

#[test]
fn test_incremental_indexing() {
	let temp_dir = create_test_project();

	// First index with persistence
	let manager = IndexManager::new().with_persistence();
	let result1 = manager.index_project(temp_dir.path()).unwrap();
	assert!(!result1.incremental); // First run is not incremental

	// Second index should be incremental with no changes
	let result2 = manager.index_project(temp_dir.path()).unwrap();
	assert!(result2.incremental);
	assert!(result2.changes.is_some());
	let changes = result2.changes.as_ref().unwrap();
	assert!(!changes.has_changes()); // No changes

	// Clean up
	IndexManager::clear_index(temp_dir.path()).unwrap();
}

#[test]
fn test_reference_extraction() {
	let temp_dir = create_test_project();

	let manager = IndexManager::new()
		.with_semantic_analysis()
		.with_reference_extraction();
	let result = manager.index_project(temp_dir.path()).unwrap();

	// Should have extracted some references
	assert!(!result.references.is_empty());

	// Semantic graph should have references
	let graph = result.semantic_graph.as_ref().unwrap();
	let stats = graph.stats();
	assert!(stats.total_references > 0);
}

#[test]
fn test_index_stats() {
	let temp_dir = create_test_project();

	// Initially no index
	assert!(!IndexManager::has_index(temp_dir.path()));

	// Create index with persistence
	let manager = IndexManager::new().with_persistence();
	let _ = manager.index_project(temp_dir.path()).unwrap();

	// Now should have index
	assert!(IndexManager::has_index(temp_dir.path()));

	// Get stats
	let stats =
		IndexManager::get_index_stats(temp_dir.path()).unwrap();
	assert!(stats.is_some());
	let stats = stats.unwrap();
	assert_eq!(stats.file_count, 2);
	assert!(stats.symbol_count > 0);

	// Clean up
	IndexManager::clear_index(temp_dir.path()).unwrap();
	assert!(!IndexManager::has_index(temp_dir.path()));
}
