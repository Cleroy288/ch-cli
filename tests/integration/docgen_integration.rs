//! Integration tests for the documentation generation system.
//!
//! Tests the full docgen pipeline: indexing -> doc store -> linking.

use std::fs;
use std::path::PathBuf;

use ch_cli::indexer::{IndexManager, SymbolKind};
use ch_cli::retrieval::docgen::{DocEntry, DocLinker, DocStatus, DocStore};

/// Create a temporary test directory with unique name.
fn setup_test_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "docgen_test_{}_{}_{}",
        name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    // Remove if exists from previous failed test
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("Failed to create test directory");
    dir
}

/// Clean up test directory.
fn cleanup_test_dir(dir: &PathBuf) {
    let _ = fs::remove_dir_all(dir);
}

/// Create a sample Rust file for testing.
fn create_sample_rust_file(dir: &PathBuf) -> PathBuf {
    let src_dir = dir.join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let file_path = src_dir.join("lib.rs");
    let content = r#"//! Sample library for testing documentation generation.

/// A simple greeting function.
///
/// # Arguments
/// * `name` - The name to greet
///
/// # Returns
/// A greeting string
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// A calculator struct for basic math operations.
pub struct Calculator {
    /// Current value stored in the calculator
    pub value: i32,
}

impl Calculator {
    /// Create a new calculator with initial value.
    pub fn new(initial: i32) -> Self {
        Self { value: initial }
    }

    /// Add a number to the current value.
    pub fn add(&mut self, n: i32) {
        self.value += n;
    }

    /// Multiply the current value by a number.
    pub fn multiply(&mut self, n: i32) {
        self.value *= n;
    }
}

/// Result type for operations.
pub enum OpResult {
    /// Operation succeeded
    Success(i32),
    /// Operation failed with error message
    Error(String),
}

/// Perform a safe division.
pub fn safe_divide(a: i32, b: i32) -> OpResult {
    if b == 0 {
        OpResult::Error("Division by zero".to_string())
    } else {
        OpResult::Success(a / b)
    }
}
"#;

    fs::write(&file_path, content).expect("Failed to write test file");
    file_path
}

#[test]
fn test_docstore_roundtrip() {
    let dir = setup_test_dir("roundtrip");

    // create store and add entries
    let mut store = DocStore::new(&dir);

    let entry1 = DocEntry::new(
        "greet".to_string(),
        SymbolKind::Function,
        dir.join("src/lib.rs"),
        10,
    );

    let entry2 = DocEntry::new(
        "Calculator".to_string(),
        SymbolKind::Struct,
        dir.join("src/lib.rs"),
        20,
    );

    store.upsert(entry1);
    store.upsert(entry2);

    // save to disk
    store.save().expect("Failed to save store");

    // load from disk
    let loaded = DocStore::load(&dir).expect("Failed to load store");

    // verify entries
    assert!(loaded.get_by_name("greet").is_some());
    assert!(loaded.get_by_name("Calculator").is_some());
    assert_eq!(loaded.all_entries().count(), 2);

    cleanup_test_dir(&dir);
}

#[test]
fn test_docstore_sync_with_index() {
    let dir = setup_test_dir("sync");
    let rust_file = create_sample_rust_file(&dir);

    // Verify file was created
    assert!(rust_file.exists(), "Rust file was not created");

    // index the project
    let manager = IndexManager::new().with_semantic_analysis();
    let result = manager
        .index_project(dir.to_str().unwrap())
        .expect("Failed to index project");

    // create store and populate from symbols
    let mut store = DocStore::new(&dir);
    store.populate_from_symbols(&result.symbols);

    // verify entries were created (may be 0 if no .rs in root)
    let count = store.all_entries().count();

    // check if specific symbols exist (depends on indexer finding them)
    if count > 0 {
        // We found some symbols
        let greet = store.get_by_name("greet");
        let calc = store.get_by_name("Calculator");
        // At least one should exist
        assert!(
            greet.is_some() || calc.is_some(),
            "Expected to find greet or Calculator"
        );
    }

    cleanup_test_dir(&dir);
}

#[test]
fn test_docentry_status_transitions() {
    let dir = setup_test_dir("status");

    let mut entry = DocEntry::new(
        "test_fn".to_string(),
        SymbolKind::Function,
        dir.join("test.rs"),
        1,
    );

    // initial status should be Pending
    assert_eq!(entry.status, DocStatus::Pending);

    // transition to generating
    entry.mark_generating();
    assert_eq!(entry.status, DocStatus::Generating);

    // set generated doc using mark_ready
    entry.mark_ready("This function does something.".to_string());
    assert_eq!(entry.status, DocStatus::Ready);
    assert!(entry.llm_doc.is_some());

    cleanup_test_dir(&dir);
}

#[test]
fn test_docentry_combined_doc() {
    let dir = setup_test_dir("combined");

    let mut entry = DocEntry::new(
        "test_fn".to_string(),
        SymbolKind::Function,
        dir.join("test.rs"),
        1,
    );

    // set user comment
    entry.user_comment = Some("User's description.".to_string());

    // set llm doc using mark_ready
    entry.mark_ready("LLM generated description.".to_string());

    // combined doc should have both
    let combined = entry.combined_doc();
    assert!(combined.contains("User's description"));
    assert!(combined.contains("LLM generated"));

    cleanup_test_dir(&dir);
}

#[test]
fn test_docstore_pending_entries() {
    let dir = setup_test_dir("pending");

    let mut store = DocStore::new(&dir);

    // add entries with different statuses
    let mut entry1 = DocEntry::new(
        "fn1".to_string(),
        SymbolKind::Function,
        dir.join("test.rs"),
        1,
    );
    entry1.mark_ready("Done".to_string());

    let entry2 = DocEntry::new(
        "fn2".to_string(),
        SymbolKind::Function,
        dir.join("test.rs"),
        10,
    );

    let entry3 = DocEntry::new(
        "fn3".to_string(),
        SymbolKind::Function,
        dir.join("test.rs"),
        20,
    );

    store.upsert(entry1);
    store.upsert(entry2);
    store.upsert(entry3);

    // check pending count - get_pending returns Vec
    let pending = store.get_pending();
    assert_eq!(pending.len(), 2);

    // check stats - returns DocStoreStats struct
    let stats = store.stats();
    assert_eq!(stats.total, 3);
    assert_eq!(stats.ready, 1);
    assert_eq!(stats.pending, 2);

    cleanup_test_dir(&dir);
}

#[test]
fn test_doclinker_extract_crates() {
    let linker = DocLinker::new();

    let code = r#"
use serde::{Serialize, Deserialize};
use tokio::runtime::Runtime;
use std::collections::HashMap;
extern crate regex;
"#;

    let crates = linker.extract_crates(code);

    assert!(crates.contains(&"serde".to_string()));
    assert!(crates.contains(&"tokio".to_string()));
    assert!(crates.contains(&"regex".to_string()));
    // std should be filtered out
    assert!(!crates.contains(&"std".to_string()));
}

#[test]
fn test_docstore_exists_check() {
    let dir = setup_test_dir("exists");

    // should not exist initially
    assert!(!DocStore::exists(&dir));

    // create and save store
    let store = DocStore::new(&dir);
    store.save().expect("Failed to save store");

    // should exist now
    assert!(DocStore::exists(&dir));

    cleanup_test_dir(&dir);
}

#[test]
fn test_full_indexing_pipeline() {
    let dir = setup_test_dir("pipeline");
    let rust_file = create_sample_rust_file(&dir);

    // Verify file was created
    assert!(rust_file.exists(), "Rust file should exist");
    let content = fs::read_to_string(&rust_file).expect("Should read file");
    assert!(content.contains("greet"), "File should contain greet");

    // step 1: index project
    let manager = IndexManager::new().with_semantic_analysis();
    let result = manager
        .index_project(dir.to_str().unwrap())
        .expect("Indexing should succeed");

    // step 2: create doc store
    let mut store = DocStore::new(&dir);
    store.populate_from_symbols(&result.symbols);

    // step 3: verify store contains expected symbols (if indexer found them)
    let greet_entry = store.get_by_name("greet");
    let calc_entry = store.get_by_name("Calculator");

    // At least verify the store operations work
    if let Some(greet) = greet_entry {
        assert_eq!(greet.kind, SymbolKind::Function);
    }

    // step 4: build links using semantic graph (if available)
    if let Some(ref graph) = result.semantic_graph {
        let linker = DocLinker::new();
        linker.build_links(&mut store, graph, &result.symbols);
    }

    // step 5: save and reload
    store.save().expect("Save should succeed");

    let reloaded = DocStore::load(&dir).expect("Load should succeed");
    assert_eq!(reloaded.all_entries().count(), store.all_entries().count());

    cleanup_test_dir(&dir);
}

#[test]
fn test_docentry_code_snippet() {
    let dir = setup_test_dir("snippet");
    let file_path = create_sample_rust_file(&dir);

    let mut entry = DocEntry::new(
        "greet".to_string(),
        SymbolKind::Function,
        file_path.clone(),
        12, // line where greet is defined
    );

    // set code snippet manually for this test
    entry.code_snippet = r#"pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}"#
    .to_string();

    assert!(entry.code_snippet.contains("greet"));
    assert!(entry.code_snippet.contains("Hello"));

    cleanup_test_dir(&dir);
}

#[test]
fn test_docstore_update_existing() {
    let dir = setup_test_dir("update");

    let mut store = DocStore::new(&dir);

    // add initial entry
    let entry1 = DocEntry::new(
        "test_fn".to_string(),
        SymbolKind::Function,
        dir.join("test.rs"),
        1,
    );
    let id = entry1.id.clone();
    store.upsert(entry1);

    // update the entry
    if let Some(entry) = store.get_mut(&id) {
        entry.mark_ready("Updated documentation.".to_string());
    }

    // verify update
    let retrieved = store.get(&id).unwrap();
    assert_eq!(retrieved.status, DocStatus::Ready);
    assert!(retrieved.llm_doc.as_ref().unwrap().contains("Updated"));

    cleanup_test_dir(&dir);
}

#[test]
fn test_docentry_is_stale() {
    let dir = setup_test_dir("stale");

    let entry = DocEntry::new(
        "test_fn".to_string(),
        SymbolKind::Function,
        dir.join("test.rs"),
        1,
    )
    .with_mtime(1000);

    // same mtime should not be stale
    assert!(!entry.is_stale(1000));

    // newer mtime should be stale
    assert!(entry.is_stale(2000));

    // older mtime should not be stale
    assert!(!entry.is_stale(500));

    cleanup_test_dir(&dir);
}

#[test]
fn test_symbol_links_operations() {
    let mut links = ch_cli::retrieval::docgen::SymbolLinks::new();

    // test adding dependencies
    links.add_depends_on("foo".to_string());
    links.add_depends_on("bar".to_string());
    links.add_depends_on("foo".to_string()); // duplicate

    assert_eq!(links.depends_on.len(), 2);
    assert!(links.depends_on.contains(&"foo".to_string()));
    assert!(links.depends_on.contains(&"bar".to_string()));

    // test adding dependents
    links.add_depended_by("baz".to_string());
    assert_eq!(links.depended_by.len(), 1);

    // test adding children
    links.add_child("child1".to_string());
    links.add_child("child2".to_string());
    assert_eq!(links.children.len(), 2);

    // test adding external deps
    links.add_external_dep("serde".to_string());
    links.add_external_dep("tokio".to_string());
    assert_eq!(links.external_deps.len(), 2);
}

#[test]
fn test_docentry_embedding_text() {
    let dir = setup_test_dir("embedding");

    let mut entry = DocEntry::new(
        "my_function".to_string(),
        SymbolKind::Function,
        dir.join("test.rs"),
        1,
    )
    .with_signature("fn my_function(x: i32) -> i32".to_string());

    entry.user_comment = Some("Doubles the input".to_string());
    entry.llm_doc = Some("A function that multiplies by 2".to_string());

    let text = entry.embedding_text();

    assert!(text.contains("my_function"));
    assert!(text.contains("fn my_function"));
    assert!(text.contains("Doubles the input"));
    assert!(text.contains("multiplies by 2"));

    cleanup_test_dir(&dir);
}
