//! Demo: Index the rustean project with semantic analysis
//!
//! Run with: cargo run --example index_demo

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

use rustean::indexer::{IndexManager, SearchIndex, SymbolKind, Visibility};

fn main() {
    println!("🔍 Semantic Code Indexer Demo - Phase 4\n");
    println!("Indexing rustean project with semantic analysis...\n");

    // Create index manager with progress tracking and semantic analysis enabled
    let files_processed = Arc::new(AtomicUsize::new(0));
    let files_processed_clone = files_processed.clone();

    let manager =
        IndexManager::new()
            .with_semantic_analysis()
            .on_progress(move |current, total, path| {
                files_processed_clone.store(current, Ordering::SeqCst);
                if current % 10 == 0 || current == total {
                    println!("  [{}/{}] Parsing: {}", current, total, path.display());
                }
            });

    // Index the project
    let result = manager.index_project(".").expect("Failed to index project");

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Parsing Statistics:");
    println!("   Files discovered: {}", result.stats.files_found);
    println!("   Files parsed:     {}", result.stats.files_parsed);
    println!("   Files failed:     {}", result.stats.files_failed);
    println!("   Symbols found:    {}", result.stats.symbols_found);
    println!("   Parse duration:   {} ms", result.stats.duration_ms);

    // Create search index and add symbols
    println!("\n📚 Building Tantivy search index...");
    let index_start = Instant::now();
    let search_index = SearchIndex::in_memory().expect("Failed to create search index");
    search_index
        .index_symbols(&result.symbols)
        .expect("Failed to index symbols");
    let index_duration = index_start.elapsed();
    println!(
        "   Indexed {} symbols in {} ms",
        result.symbols.len(),
        index_duration.as_millis()
    );
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Demo: Full-text search
    println!("🔎 Search Demo: \"handle\"");
    let search_start = Instant::now();
    let hits = search_index.search("handle", 5).expect("Search failed");
    let search_duration = search_start.elapsed();
    println!("   Found {} results in {:?}", hits.len(), search_duration);
    for hit in &hits {
        let file = hit
            .symbol
            .location
            .file
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("?");
        println!(
            "   • {} {} ({}, line {}) [score: {:.2}]",
            hit.symbol.kind, hit.symbol.name, file, hit.symbol.location.line, hit.score
        );
    }

    // Demo: Fuzzy search (handles typos)
    println!("\n🔎 Fuzzy Search Demo: \"proces\" (typo for \"process\")");
    let hits = search_index
        .fuzzy_search("proces", 2, 5)
        .expect("Fuzzy search failed");
    println!("   Found {} results", hits.len());
    for hit in &hits {
        let file = hit
            .symbol
            .location
            .file
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("?");
        println!(
            "   • {} {} ({}, line {}) [score: {:.2}]",
            hit.symbol.kind, hit.symbol.name, file, hit.symbol.location.line, hit.score
        );
    }

    // Demo: Search by kind
    println!("\n🔎 Search by Kind: Structs");
    let hits = search_index
        .search_by_kind(SymbolKind::Struct, 10)
        .expect("Search by kind failed");
    println!("   Found {} structs", hits.len());
    for hit in &hits {
        let file = hit
            .symbol
            .location
            .file
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("?");
        let vis = if hit.symbol.visibility == Visibility::Public {
            "pub "
        } else {
            ""
        };
        println!(
            "   • {}{} ({}, line {})",
            vis, hit.symbol.name, file, hit.symbol.location.line
        );
    }

    // Demo: Semantic Analysis
    if let Some(ref graph) = result.semantic_graph {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🧠 Semantic Analysis Demo:\n");

        // Show stats
        let stats = graph.stats();
        println!("   Total definitions: {}", stats.total_definitions);
        println!("   Unique symbols:    {}", stats.unique_symbols);
        println!("   Files analyzed:    {}", stats.files_analyzed);

        // Find all definitions of a common symbol
        println!("\n🔎 Find Definitions: \"new\"");
        let defs = graph.find_definitions("new");
        println!("   Found {} definitions of 'new':", defs.len());
        for def in defs.iter().take(5) {
            let file = def
                .symbol
                .location
                .file
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("?");
            println!(
                "   • {} {} in {} (line {})",
                def.symbol.kind, def.fqn, file, def.symbol.location.line
            );
        }
        if defs.len() > 5 {
            println!("   ... and {} more", defs.len() - 5);
        }

        // Find all structs by kind
        println!("\n🔎 Find All Structs (by kind):");
        let structs = graph.find_by_kind(SymbolKind::Struct);
        println!("   Found {} struct definitions:", structs.len());
        for def in structs.iter().take(5) {
            let file = def
                .symbol
                .location
                .file
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("?");
            println!(
                "   • {} in {} (line {})",
                def.symbol.name, file, def.symbol.location.line
            );
        }
        if structs.len() > 5 {
            println!("   ... and {} more", structs.len() - 5);
        }

        // Find all traits
        println!("\n🔎 Find All Traits:");
        let traits = graph.find_by_kind(SymbolKind::Trait);
        println!("   Found {} trait definitions:", traits.len());
        for def in traits.iter().take(5) {
            let file = def
                .symbol
                .location
                .file
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("?");
            println!(
                "   • {} in {} (line {})",
                def.symbol.name, file, def.symbol.location.line
            );
        }
    }

    // Show files with errors
    let errors: Vec<_> = result
        .file_results
        .iter()
        .filter(|r| r.error.is_some())
        .collect();
    if !errors.is_empty() {
        println!("\n⚠️  Files with errors:");
        for file_result in errors {
            println!(
                "   {} - {:?}",
                file_result.path.display(),
                file_result.error
            );
        }
    }

    println!("\n🎉 Phase 4 Complete: Semantic analysis with definition tracking!");
}
