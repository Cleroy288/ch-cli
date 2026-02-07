//! Unit tests for trigram index

use std::path::{Path, PathBuf};

use ch_cli::indexer::trigram::TrigramIndex;

#[test]
fn test_extract_trigrams() {
    let trigrams = TrigramIndex::extract_trigrams("hello");

    assert_eq!(trigrams.len(), 3); // "hel", "ell", "llo"
    assert_eq!(trigrams[0], [b'h', b'e', b'l']);
    assert_eq!(trigrams[1], [b'e', b'l', b'l']);
    assert_eq!(trigrams[2], [b'l', b'l', b'o']);
}

#[test]
fn test_extract_trigrams_short() {
    assert!(TrigramIndex::extract_trigrams("ab").is_empty());
    assert!(TrigramIndex::extract_trigrams("").is_empty());
}

#[test]
fn test_index_and_search() {
    let mut index = TrigramIndex::new(); // test index

    index.index_file(Path::new("a.rs"), "fn hello_world() {}");
    index.index_file(Path::new("b.rs"), "fn goodbye() {}");
    index.index_file(Path::new("c.rs"), "struct Hello { world: i32 }");

    // search for "hello" - should find a.rs
    let candidates = index.candidate_files("hello");
    assert!(candidates.contains(&PathBuf::from("a.rs")));

    // search for "world" - should find a.rs and c.rs
    let candidates = index.candidate_files("world");
    assert!(candidates.contains(&PathBuf::from("a.rs")));
    assert!(candidates.contains(&PathBuf::from("c.rs")));

    // search for "xyz" - should find nothing
    let candidates = index.candidate_files("xyz123");
    assert!(candidates.is_empty());
}

#[test]
fn test_stats() {
    let mut index = TrigramIndex::new(); // test index

    index.index_file(Path::new("a.rs"), "hello world");

    let stats = index.stats();
    assert_eq!(stats.file_count, 1);
    assert!(stats.trigram_count > 0);
}
