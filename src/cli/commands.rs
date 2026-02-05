//! Command implementations for the CLI.

use std::path::Path;
use std::time::Instant;

use crate::indexer::{IndexError, IndexManager, SearchIndex, SemanticGraph, SymbolKind};
use crate::retrieval::agent::{PipelineConfig, RetrievalPipeline};
use crate::retrieval::daemon::{
    daemon_status, start_daemon, stop_daemon, DaemonClient, ModelDaemon,
};
use crate::retrieval::docgen::DocStore;
use crate::retrieval::hybrid::HybridSearch;
use crate::retrieval::query::{detect_structure_query, find_module_structure, list_source_directories, StructureQuery};
use crate::retrieval::{ContextConfig, ContextExpander, RetrievalConfig};

/// Result type for CLI commands
pub type CommandResult = Result<(), CommandError>;

/// Errors that can occur during command execution
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Index error: {0}")]
    IndexError(String),

    #[error("Index manager error: {0}")]
    IndexManagerError(#[from] IndexError),

    #[error("Search error: {0}")]
    SearchError(#[from] crate::indexer::SearchError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Symbol not found: {0}")]
    SymbolNotFound(String),

    #[error("Invalid symbol kind: {0}")]
    InvalidKind(String),

    #[error("Retrieval error: {0}")]
    RetrievalError(#[from] crate::retrieval::RetrievalError),
}

/// Execute the `index` command
pub fn index_command(path: &str, semantic: bool, verbose: bool) -> CommandResult {
    let start = Instant::now();

    println!("🔍 Indexing project at: {}", path);

    let manager = if semantic {
        println!("   Semantic analysis: enabled");
        IndexManager::new().with_semantic_analysis()
    } else {
        IndexManager::new()
    };

    // Add progress callback if verbose
    let manager = if verbose {
        manager.on_progress(|current, total, file| {
            println!("   [{}/{}] {}", current, total, file.display());
        })
    } else {
        manager
    };

    let result = manager.index_project(path)?;

    println!("\n📊 Index Statistics:");
    println!("   Files discovered: {}", result.stats.files_found);
    println!("   Files parsed:     {}", result.stats.files_parsed);
    println!("   Files failed:     {}", result.stats.files_failed);
    println!("   Symbols found:    {}", result.stats.symbols_found);
    println!("   Duration:         {} ms", result.stats.duration_ms);

    if let Some(ref graph) = result.semantic_graph {
        let stats = graph.stats();
        println!("\n🧠 Semantic Analysis:");
        println!("   Definitions:      {}", stats.total_definitions);
        println!("   Unique symbols:   {}", stats.unique_symbols);
    }

    // Build search index
    println!("\n📚 Building search index...");
    let search_start = Instant::now();
    let search_index = SearchIndex::in_memory()
        .map_err(|e| CommandError::IndexError(e.to_string()))?;
    search_index
        .index_symbols(&result.symbols)
        .map_err(|e| CommandError::IndexError(e.to_string()))?;
    println!(
        "   Indexed {} symbols in {} ms",
        result.symbols.len(),
        search_start.elapsed().as_millis()
    );

    println!(
        "\n✅ Indexing complete in {} ms",
        start.elapsed().as_millis()
    );

    Ok(())
}

/// Execute the `search` command
pub fn search_command(
    query: &str,
    limit: usize,
    fuzzy: bool,
    kind: Option<&str>,
    semantic: bool,
    context: bool,
    rerank: bool,
) -> CommandResult {
    // Check for structure query first (e.g., "modules in retrieval")
    if let Some(structure_query) = detect_structure_query(query) {
        return handle_structure_query(&structure_query);
    }

    // First, index the project (with semantic analysis for context expansion)
    let manager = if context {
        IndexManager::new().with_semantic_analysis()
    } else {
        IndexManager::new()
    };
    let result = manager.index_project(".")?;

    if semantic {
        // Use hybrid search (keyword + semantic)
        return search_semantic(query, limit, &result.symbols, context, rerank, &result.semantic_graph);
    }

    // Build search index
    let search_index = SearchIndex::in_memory()
        .map_err(|e| CommandError::IndexError(e.to_string()))?;
    search_index
        .index_symbols(&result.symbols)
        .map_err(|e| CommandError::IndexError(e.to_string()))?;

    // Perform search
    let hits = if let Some(kind_str) = kind {
        let symbol_kind = parse_symbol_kind(kind_str)?;
        search_index.search_by_kind(symbol_kind, limit)?
    } else if fuzzy {
        search_index.fuzzy_search(query, 2, limit)?
    } else {
        search_index.search(query, limit)?
    };

    if hits.is_empty() {
        println!("No results found for '{}'", query);
        return Ok(());
    }

    println!("Search results for '{}':\n", query);
    for (i, hit) in hits.iter().enumerate() {
        let file = hit
            .symbol
            .location
            .file
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("?");
        println!(
            "  {}. {} {} ({}:{})",
            i + 1,
            hit.symbol.kind,
            hit.symbol.name,
            file,
            hit.symbol.location.line
        );
        if let Some(ref sig) = hit.symbol.signature {
            println!("     {}", sig);
        }
    }

    Ok(())
}

/// Handle a module structure query
fn handle_structure_query(query: &StructureQuery) -> CommandResult {
    let base_path = std::path::Path::new("."); // current directory
    let modules = find_module_structure(base_path, &query.target); // find modules

    if modules.is_empty() {
        println!("No module structure found for '{}'", query.target);
        println!("\nAvailable directories under src/:");

        // List available directories for suggestions
        let dirs = list_source_directories(base_path); // get src subdirs
        for dir in dirs {
            println!("  - {}", dir);
        }
        return Ok(());
    }

    println!("Module structure for '{}':\n", query.target);

    for module in modules {
        // Show file path with icon
        println!("📁 {}", module.path.display());

        // Show submodules with visibility
        if !module.submodules.is_empty() {
            for sub in &module.submodules {
                let vis = if sub.is_public { "pub mod" } else { "mod" }; // visibility
                if let Some(ref doc) = sub.doc {
                    println!("   {} {}; // {}", vis, sub.name, doc);
                } else {
                    println!("   {} {};", vis, sub.name);
                }
            }
        }

        // Show re-export count (summarized to keep output clean)
        if !module.reexports.is_empty() {
            println!("   // {} re-exports", module.reexports.len());
        }

        println!(); // blank line between modules
    }

    Ok(())
}

/// Perform semantic (hybrid) search
fn search_semantic(
    query: &str,
    limit: usize,
    symbols: &[crate::indexer::Symbol],
    context: bool,
    rerank: bool,
    semantic_graph: &Option<SemanticGraph>,
) -> CommandResult {
    println!("Building hybrid search index...");

    // Create hybrid search
    let mut hybrid = HybridSearch::new()?;

    // Index symbols (this also generates embeddings via daemon)
    println!("Generating embeddings for {} symbols...", symbols.len());
    hybrid.index_symbols(symbols)?;

    // Perform hybrid search - get more results if reranking
    let fetch_limit = if rerank { limit * 3 } else { limit };
    println!("Searching...\n");
    let mut results = hybrid.search(query, fetch_limit)?;

    if results.is_empty() {
        println!("No results found for '{}'", query);
        return Ok(());
    }

    // Rerank if requested
    if rerank {
        println!("Reranking {} results...\n", results.len());
        results = rerank_results(query, results, limit)?;
    }

    // If context expansion requested, output XML format
    if context {
        if let Some(ref graph) = semantic_graph {
            return output_with_context(query, &results, graph);
        } else {
            println!("Warning: Context expansion requires semantic analysis.");
            println!("Run with semantic analysis enabled to get context.\n");
        }
    }

    let title = if rerank {
        "Reranked search results"
    } else {
        "Hybrid search results"
    };
    println!("{} for '{}':\n", title, query);

    for (i, result) in results.iter().enumerate() {
        let file = result
            .symbol
            .location
            .file
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("?");

        // show RRF score and source ranks
        let kw_rank = result
            .keyword_rank
            .map(|r| format!("K:{}", r))
            .unwrap_or_else(|| "-".to_string());
        let sem_rank = result
            .semantic_rank
            .map(|r| format!("S:{}", r))
            .unwrap_or_else(|| "-".to_string());

        // show rerank score if available
        let rerank_info = if let Some(rs) = result.rerank_score {
            format!(" R:{:.3}", rs)
        } else {
            String::new()
        };

        println!(
            "  {}. {} {} ({}:{}) [RRF:{:.4} {} {}{}]",
            i + 1,
            result.symbol.kind,
            result.symbol.name,
            file,
            result.symbol.location.line,
            result.rrf_score,
            kw_rank,
            sem_rank,
            rerank_info
        );

        if let Some(ref sig) = result.symbol.signature {
            println!("     {}", sig);
        }
    }

    Ok(())
}

/// Rerank search results using cross-encoder
fn rerank_results(
    query: &str,
    mut results: Vec<crate::retrieval::hybrid::HybridSearchResult>,
    limit: usize,
) -> Result<Vec<crate::retrieval::hybrid::HybridSearchResult>, CommandError> {
    // create documents for reranking (symbol name + signature)
    let documents: Vec<String> = results
        .iter()
        .map(|r| {
            format!(
                "{} {} {}",
                r.symbol.kind,
                r.symbol.name,
                r.symbol.signature.as_deref().unwrap_or("")
            )
        })
        .collect();

    // call daemon for reranking
    let client = DaemonClient::new();
    let scores = client.rerank(query.to_string(), documents)?;

    // assign rerank scores
    for (result, score) in results.iter_mut().zip(scores.iter()) {
        result.rerank_score = Some(*score);
    }

    // sort by rerank score (descending)
    results.sort_by(|a, b| {
        b.rerank_score
            .partial_cmp(&a.rerank_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // truncate to limit
    results.truncate(limit);

    Ok(results)
}

/// Output search results with expanded context as XML
fn output_with_context(
    query: &str,
    results: &[crate::retrieval::hybrid::HybridSearchResult],
    graph: &SemanticGraph,
) -> CommandResult {
    // extract symbols from results
    let symbols: Vec<_> = results.iter().map(|r| r.symbol.clone()).collect();

    // create context expander
    let config = ContextConfig::default();
    let expander = ContextExpander::with_config(graph, config, 8000);

    // expand and format as XML
    let xml = expander.expand_to_xml(&symbols);

    println!("Context-expanded results for '{}':\n", query);
    println!("{}", xml);

    Ok(())
}

/// Execute the `goto` command - find symbol definition
pub fn goto_command(symbol: &str) -> CommandResult {
    // Index with semantic analysis for definition tracking
    let manager = IndexManager::new().with_semantic_analysis();
    let result = manager.index_project(".")?;

    let graph = result
        .semantic_graph
        .ok_or_else(|| CommandError::IndexError("Semantic graph not available".into()))?;

    let definitions = graph.find_definitions(symbol);

    if definitions.is_empty() {
        return Err(CommandError::SymbolNotFound(symbol.to_string()));
    }

    println!("📍 Definition(s) for '{}':\n", symbol);
    for def in definitions {
        println!(
            "  {} {} - {}:{}:{}",
            def.symbol.kind,
            def.symbol.name,
            def.symbol.location.file.display(),
            def.symbol.location.line,
            def.symbol.location.column
        );
        if let Some(ref sig) = def.symbol.signature {
            println!("     {}", sig);
        }
    }

    Ok(())
}

/// Execute the `refs` command - find all references to a symbol
pub fn refs_command(symbol: &str, include_definition: bool) -> CommandResult {
    // Index with semantic analysis for reference tracking
    let manager = IndexManager::new().with_semantic_analysis();
    let result = manager.index_project(".")?;

    let graph = result
        .semantic_graph
        .ok_or_else(|| CommandError::IndexError("Semantic graph not available".into()))?;

    let usages = graph.find_all_usages(symbol);

    if usages.total() == 0 {
        return Err(CommandError::SymbolNotFound(symbol.to_string()));
    }

    if include_definition && !usages.definitions.is_empty() {
        println!("📍 Definition(s):\n");
        for def in &usages.definitions {
            println!(
                "  {} - {}:{}",
                symbol,
                def.file.display(),
                def.line
            );
        }
        println!();
    }

    if !usages.references.is_empty() {
        println!("🔗 References ({}):\n", usages.references.len());
        for reference in &usages.references {
            println!(
                "  {}:{}",
                reference.file.display(),
                reference.line
            );
        }
    } else {
        println!("No references found for '{}'", symbol);
    }

    Ok(())
}

/// Execute the `symbols` command - list symbols in a file or project
pub fn symbols_command(file: Option<&str>, kind: Option<&str>) -> CommandResult {
    let manager = IndexManager::new().with_semantic_analysis();
    let result = manager.index_project(".")?;

    let graph = result
        .semantic_graph
        .ok_or_else(|| CommandError::IndexError("Semantic graph not available".into()))?;

    // Filter by kind if specified
    let symbols: Vec<_> = if let Some(kind_str) = kind {
        let symbol_kind = parse_symbol_kind(kind_str)?;
        graph.find_by_kind(symbol_kind)
    } else {
        // Get all definitions
        graph
            .all_symbol_names()
            .into_iter()
            .flat_map(|name| graph.find_definitions(name))
            .collect()
    };

    // Filter by file if specified
    let symbols: Vec<_> = if let Some(file_path) = file {
        let path = Path::new(file_path);
        symbols
            .into_iter()
            .filter(|def| def.symbol.location.file.ends_with(path))
            .collect()
    } else {
        symbols
    };

    if symbols.is_empty() {
        println!("No symbols found");
        return Ok(());
    }

    let title = match (file, kind) {
        (Some(f), Some(k)) => format!("Symbols in '{}' of kind '{}'", f, k),
        (Some(f), None) => format!("Symbols in '{}'", f),
        (None, Some(k)) => format!("All '{}' symbols", k),
        (None, None) => "All symbols".to_string(),
    };

    println!("📋 {}:\n", title);
    for def in symbols {
        let file_name = def
            .symbol
            .location
            .file
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("?");
        println!(
            "  {} {} ({}:{})",
            def.symbol.kind, def.symbol.name, file_name, def.symbol.location.line
        );
    }

    Ok(())
}

/// Execute the `stats` command - show index statistics
pub fn stats_command() -> CommandResult {
    let start = Instant::now();

    let manager = IndexManager::new().with_semantic_analysis();
    let result = manager.index_project(".")?;

    println!("📊 Index Statistics:\n");
    println!("  Files discovered:  {}", result.stats.files_found);
    println!("  Files parsed:      {}", result.stats.files_parsed);
    println!("  Files failed:      {}", result.stats.files_failed);
    println!("  Symbols found:     {}", result.stats.symbols_found);
    println!("  Indexing time:     {} ms", result.stats.duration_ms);

    if let Some(ref graph) = result.semantic_graph {
        let stats = graph.stats();
        println!("\n🧠 Semantic Analysis:\n");
        println!("  Total definitions: {}", stats.total_definitions);
        println!("  Unique symbols:    {}", stats.unique_symbols);
        println!("  Files analyzed:    {}", stats.files_analyzed);
        println!("  Total references:  {}", stats.total_references);

        // Show symbol breakdown by kind
        println!("\n📋 Symbol breakdown:");
        let kinds = [
            SymbolKind::Function,
            SymbolKind::Struct,
            SymbolKind::Enum,
            SymbolKind::Trait,
            SymbolKind::Impl,
            SymbolKind::Method,
            SymbolKind::Constant,
            SymbolKind::Static,
            SymbolKind::Module,
        ];
        for kind in kinds {
            let count = graph.find_by_kind(kind).len();
            if count > 0 {
                println!("  {:12}: {}", format!("{}", kind), count);
            }
        }
    }

    println!(
        "\n⏱️  Stats generated in {} ms",
        start.elapsed().as_millis()
    );

    Ok(())
}

/// Parse a symbol kind string into a SymbolKind enum
fn parse_symbol_kind(kind_str: &str) -> Result<SymbolKind, CommandError> {
    match kind_str.to_lowercase().as_str() {
        "function" | "fn" | "func" => Ok(SymbolKind::Function),
        "struct" => Ok(SymbolKind::Struct),
        "enum" => Ok(SymbolKind::Enum),
        "trait" => Ok(SymbolKind::Trait),
        "impl" => Ok(SymbolKind::Impl),
        "method" => Ok(SymbolKind::Method),
        "const" | "constant" => Ok(SymbolKind::Constant),
        "static" => Ok(SymbolKind::Static),
        "type" | "typealias" => Ok(SymbolKind::TypeAlias),
        "module" | "mod" => Ok(SymbolKind::Module),
        "macro" => Ok(SymbolKind::Macro),
        "field" => Ok(SymbolKind::Field),
        "variant" => Ok(SymbolKind::EnumVariant),
        _ => Err(CommandError::InvalidKind(kind_str.to_string())),
    }
}

// ============================================================================
// Daemon Commands
// ============================================================================

/// Execute the `daemon start` command
pub fn daemon_start_command() -> CommandResult {
    let config = RetrievalConfig::default();

    // check if already running
    let status = daemon_status(&config.socket_path);
    if status.running {
        println!("Daemon is already running (PID: {})", status.pid.unwrap_or(0));
        return Ok(());
    }

    println!("Starting model daemon...");
    start_daemon(&config.socket_path)?;

    // wait for daemon to be ready
    std::thread::sleep(std::time::Duration::from_millis(500));

    let status = daemon_status(&config.socket_path);
    if status.running {
        println!("Daemon started (PID: {})", status.pid.unwrap_or(0));
        println!("Models are loading in the background...");
    } else {
        println!("Warning: Daemon may not have started correctly");
    }

    Ok(())
}

/// Execute the `daemon stop` command
pub fn daemon_stop_command() -> CommandResult {
    let config = RetrievalConfig::default();

    let status = daemon_status(&config.socket_path);
    if !status.running {
        println!("Daemon is not running");
        return Ok(());
    }

    println!("Stopping daemon (PID: {})...", status.pid.unwrap_or(0));
    stop_daemon(&config.socket_path)?;
    println!("Daemon stopped");

    Ok(())
}

/// Execute the `daemon status` command
pub fn daemon_status_command() -> CommandResult {
    let config = RetrievalConfig::default();

    // get basic status
    let status = daemon_status(&config.socket_path);

    if !status.running {
        println!("Daemon Status: Not running");
        println!("Socket path:   {:?}", config.socket_path);
        return Ok(());
    }

    // get detailed status from daemon
    let client = DaemonClient::new();
    match client.status() {
        Ok(detailed) => {
            println!("Daemon Status: Running");
            println!("PID:           {}", detailed.pid.unwrap_or(0));
            println!(
                "Device:        {} ({})",
                detailed.device.device_type, detailed.device.device_name
            );
            if let Some(mem) = detailed.device.memory_mb {
                println!("GPU Memory:    {} MB", mem);
            }
            println!("Uptime:        {} seconds", detailed.uptime_secs);
            println!("Socket:        {:?}", config.socket_path);
            println!("\nLoaded Models:");
            for model in &detailed.loaded_models {
                println!("  - {}", model);
            }
            if detailed.loaded_models.is_empty() {
                println!("  (none)");
            }
        }
        Err(e) => {
            println!("Daemon Status: Running (but unreachable)");
            println!("PID:           {}", status.pid.unwrap_or(0));
            println!("Error:         {}", e);
        }
    }

    Ok(())
}

/// Execute the `daemon restart` command
pub fn daemon_restart_command() -> CommandResult {
    daemon_stop_command()?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    daemon_start_command()
}

/// Execute the `daemon run` command (foreground mode)
pub fn daemon_run_command(socket: Option<&str>) -> CommandResult {
    let mut daemon = match socket {
        Some(path) => ModelDaemon::with_socket_path(path),
        None => ModelDaemon::new(),
    };

    daemon.run()?;
    Ok(())
}

/// Execute the `embed` command
pub fn embed_command(path: &str, _force: bool) -> CommandResult {
    let start = Instant::now();

    println!("Indexing project at: {}", path);

    // first index the project
    let manager = IndexManager::new().with_semantic_analysis();
    let result = manager.index_project(path)?;

    println!("Found {} symbols", result.symbols.len());

    // ensure daemon is running
    let client = DaemonClient::new();
    println!("Connecting to daemon...");

    // ping to check if daemon is ready (auto-starts if needed)
    match client.ping() {
        Ok(true) => println!("Daemon is ready"),
        Ok(false) => {
            println!("Daemon started, waiting for models to load...");
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
        Err(e) => {
            println!("Warning: Could not connect to daemon: {}", e);
            println!("Run 'ch-cli daemon start' first and wait for models to load");
            return Ok(());
        }
    }

    // generate embeddings for symbols
    println!("Generating embeddings...");
    let texts: Vec<String> = result
        .symbols
        .iter()
        .map(|s| {
            format!(
                "{} {} {}",
                s.kind,
                s.name,
                s.signature.as_deref().unwrap_or("")
            )
        })
        .collect();

    // batch by 32 to avoid overwhelming the daemon
    let batch_size = 32;
    let mut total_embedded = 0;

    for (i, batch) in texts.chunks(batch_size).enumerate() {
        match client.embed(batch.to_vec()) {
            Ok(embeddings) => {
                total_embedded += embeddings.len();
                println!(
                    "  Batch {}: embedded {} symbols ({}/{})",
                    i + 1,
                    embeddings.len(),
                    total_embedded,
                    texts.len()
                );
            }
            Err(e) => {
                println!("  Batch {}: error - {}", i + 1, e);
            }
        }
    }

    println!(
        "\nEmbedding complete: {} symbols in {} ms",
        total_embedded,
        start.elapsed().as_millis()
    );

    Ok(())
}

// ============================================================================
// Retrieve Command (Agentic Pipeline)
// ============================================================================

/// Execute the `retrieve` command - run the full agentic retrieval pipeline
/// Uses persistent caching for fast warm starts (<500ms vs ~4.5s cold start)
pub fn retrieve_command(
    query: &str,
    limit: usize,
    max_tokens: usize,
    no_expand: bool,
    no_rerank: bool,
    no_context: bool,
    xml: bool,
    structured: bool,
    threshold: f32,
    min_results: usize,
) -> CommandResult {
    let start = Instant::now();

    // Configure the pipeline with persistence enabled for fast warm starts
    let config = PipelineConfig {
        max_results: limit,
        max_tokens,
        expand_query: !no_expand,
        tiered_expansion: true,
        fast_path_threshold: 0.7,
        importance_threshold: 0.5,
        semantic_search: true,
        rerank: !no_rerank,
        expand_context: !no_context,
        project_path: ".".to_string(),
        enable_persistence: true,
        use_daemon_cache: true,
        include_usage_counts: true,
        use_doc_context: true,
        use_triple_pipeline: structured,
        max_code_results: 10,
        max_doc_results: 5,
        max_notes_results: 3,
        max_lines_per_file: 500,
        rrf_score_threshold: threshold,
        min_results_per_type: min_results,
    };

    if !xml && !structured {
        println!("Initializing retrieval pipeline...");
    }
    let mut pipeline = RetrievalPipeline::with_config(config);

    // Use structured output for triple pipeline
    if structured {
        return retrieve_structured_output(&mut pipeline, query, xml, start);
    }

    // Run the legacy pipeline
    let output = pipeline.retrieve(query)?;

    // Output results
    if xml {
        println!("{}", output.xml_output);
    } else {
        println!("\n{}", "=".repeat(60));
        println!("RETRIEVAL RESULTS");
        println!("{}", "=".repeat(60));

        println!("\nQuery: {}", output.query);
        println!("Intent: {:?}", output.search_spec.intent);

        if !output.search_spec.symbol_names.is_empty() {
            println!("Symbols: {}", output.search_spec.symbol_names.join(", "));
        }

        println!("\nResults: {} found", output.result_count);
        println!("Tokens: ~{}", output.token_count);

        if output.has_more {
            println!("\n(More results available - increase --limit to see more)");
        }

        println!("\n{}", "-".repeat(60));
        println!("CONTEXT OUTPUT");
        println!("{}", "-".repeat(60));
        println!("{}", output.xml_output);
    }

    if !xml {
        println!(
            "\nRetrieval complete in {} ms",
            start.elapsed().as_millis()
        );
    }

    Ok(())
}

/// Run structured retrieval with separate code, doc, notes pipelines
fn retrieve_structured_output(
    pipeline: &mut RetrievalPipeline,
    query: &str,
    xml: bool,
    start: Instant,
) -> CommandResult {
    let output = pipeline.retrieve_structured(query)?;

    if xml {
        // XML output for LLM consumption
        println!("{}", output.to_xml());
    } else {
        // JSON output (default for structured)
        match output.to_json() {
            Ok(json) => println!("{}", json),
            Err(e) => return Err(CommandError::IndexError(format!("JSON serialization error: {}", e))),
        }
        eprintln!(
            "\nRetrieval: {} code, {} doc, {} notes in {}ms",
            output.code_context.len(),
            output.doc_context.len(),
            output.notes_context.len(),
            start.elapsed().as_millis()
        );
    }

    Ok(())
}

// ============================================================================
// Documentation Commands
// ============================================================================

/// Execute the `docs generate` command - start background doc generation
pub fn docs_generate_command(force: bool) -> CommandResult {
    let project_path = "."; // current directory
    let client = DaemonClient::new(); // daemon client for IPC

    println!("Starting documentation generation...");

    // send request to daemon
    match client.start_doc_gen(project_path.to_string(), force) {
        Ok(_) => {
            println!("Documentation generation started in background.");
            println!("Use 'ch-cli docs status' to check progress.");
        }
        Err(e) => {
            println!("Error starting doc generation: {}", e);
            println!("Make sure the daemon is running: ch-cli daemon start");
        }
    }

    Ok(())
}

/// Execute the `docs status` command - show generation progress
pub fn docs_status_command() -> CommandResult {
    let project_path = "."; // current directory
    let client = DaemonClient::new(); // daemon client for IPC

    match client.doc_gen_status(project_path.to_string()) {
        Ok(status) => {
            println!("📚 Documentation Status:\n");
            println!("  Total entries:    {}", status.total);
            println!("  Completed:        {}", status.completed);
            println!("  Pending:          {}", status.pending);

            // show progress bar
            if status.total > 0 {
                let pct = (status.completed as f64 / status.total as f64) * 100.0;
                let filled = (pct / 5.0) as usize; // 20 char bar
                let empty = 20 - filled;
                println!(
                    "\n  Progress: [{}{}] {:.1}%",
                    "█".repeat(filled),
                    "░".repeat(empty),
                    pct
                );
            }

            if status.is_ready {
                println!("\n✅ Documentation is ready!");
            } else {
                println!("\n⏳ Generation in progress...");
            }
        }
        Err(e) => {
            // fallback to local store check
            if DocStore::exists(Path::new(project_path)) {
                match DocStore::load(Path::new(project_path)) {
                    Ok(store) => {
                        let entries: Vec<_> = store.all_entries().collect();
                        let completed = entries.iter().filter(|e| e.llm_doc.is_some()).count();

                        println!("📚 Documentation Status (local):\n");
                        println!("  Total entries:    {}", entries.len());
                        println!("  Completed:        {}", completed);
                        println!("  Pending:          {}", entries.len() - completed);
                    }
                    Err(load_err) => {
                        println!("No documentation found.");
                        println!("Run 'ch-cli docs generate' to start.");
                        println!("(Load error: {})", load_err);
                    }
                }
            } else {
                println!("No documentation found.");
                println!("Run 'ch-cli docs generate' to start.");
                println!("(Daemon error: {})", e);
            }
        }
    }

    Ok(())
}

/// Execute the `docs show` command - display doc for a symbol
pub fn docs_show_command(symbol: &str) -> CommandResult {
    let project_path = "."; // current directory
    let client = DaemonClient::new(); // daemon client for IPC

    match client.get_doc(project_path.to_string(), symbol.to_string()) {
        Ok(Some(doc)) => {
            println!("📖 Documentation for '{}'\n", doc.name);
            println!("  Kind:     {}", doc.kind);
            println!("  File:     {}:{}", doc.file_path, doc.line);

            if let Some(sig) = &doc.signature {
                println!("\n  Signature:");
                println!("    {}", sig);
            }

            if let Some(comment) = &doc.user_comment {
                println!("\n  User Comment:");
                for line in comment.lines() {
                    println!("    {}", line);
                }
            }

            if let Some(llm_doc) = &doc.llm_doc {
                println!("\n  Generated Documentation:");
                for line in llm_doc.lines() {
                    println!("    {}", line);
                }
            }

            if !doc.depends_on.is_empty() {
                println!("\n  Depends On:");
                for dep in &doc.depends_on {
                    println!("    - {}", dep);
                }
            }

            if !doc.depended_by.is_empty() {
                println!("\n  Used By:");
                for dep in &doc.depended_by {
                    println!("    - {}", dep);
                }
            }

            if !doc.external_deps.is_empty() {
                println!("\n  External Crates:");
                for dep in &doc.external_deps {
                    println!("    - {}", dep);
                }
            }

            println!("\n  Status: {}", doc.status);
        }
        Ok(None) => {
            println!("No documentation found for '{}'", symbol);
            println!("Run 'ch-cli docs generate' to generate docs.");
        }
        Err(e) => {
            // fallback to local store
            if DocStore::exists(Path::new(project_path)) {
                match DocStore::load(Path::new(project_path)) {
                    Ok(store) => {
                        if let Some(entry) = store.get_by_name(symbol) {
                            println!("📖 Documentation for '{}' (local)\n", entry.name);
                            println!("  Kind:     {}", entry.kind);
                            println!("  File:     {}:{}", entry.file_path.display(), entry.line);

                            if let Some(ref llm_doc) = entry.llm_doc {
                                println!("\n  Generated Documentation:");
                                for line in llm_doc.lines() {
                                    println!("    {}", line);
                                }
                            } else {
                                println!("\n  (No documentation generated yet)");
                            }
                        } else {
                            println!("Symbol '{}' not found.", symbol);
                        }
                    }
                    Err(load_err) => {
                        println!("No documentation found for '{}'", symbol);
                        println!("(Load error: {})", load_err);
                    }
                }
            } else {
                println!("No documentation found for '{}'", symbol);
                println!("(Daemon error: {})", e);
            }
        }
    }

    Ok(())
}

/// Execute the `docs search` command - search documentation
pub fn docs_search_command(query: &str, limit: usize) -> CommandResult {
    let project_path = "."; // current directory
    let client = DaemonClient::new(); // daemon client for IPC

    match client.search_docs(project_path.to_string(), query.to_string(), limit) {
        Ok(results) => {
            if results.is_empty() {
                println!("No documentation found for '{}'", query);
                return Ok(());
            }

            println!("📚 Documentation search results for '{}':\n", query);

            for (i, doc) in results.iter().enumerate() {
                println!(
                    "  {}. {} {} ({}:{})",
                    i + 1,
                    doc.kind,
                    doc.name,
                    doc.file_path,
                    doc.line
                );

                // show brief excerpt from llm_doc
                if let Some(ref llm_doc) = doc.llm_doc {
                    let excerpt: String = llm_doc.chars().take(100).collect();
                    let suffix = if llm_doc.len() > 100 { "..." } else { "" };
                    println!("     {}{}", excerpt.replace('\n', " "), suffix);
                }
                println!();
            }
        }
        Err(e) => {
            println!("Search failed: {}", e);
            println!("Make sure the daemon is running: ch-cli daemon start");
        }
    }

    Ok(())
}
