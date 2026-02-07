# ch-cli Improvement Plan: Matching Augment MCP Quality

**Date:** 2026-02-03
**Goal:** Fix the 3 main issues identified in benchmark:
1. Return code instead of notes/docs for conceptual queries
2. Find all usages (increase cross-file context depth)
3. Improve daemon stability

---

## Table of Contents

1. [Problem 1: Code vs Docs Priority](#problem-1-code-vs-docs-priority)
2. [Problem 2: Cross-File Usage Depth](#problem-2-cross-file-usage-depth)
3. [Problem 3: Daemon Stability](#problem-3-daemon-stability)
4. [Implementation Order](#implementation-order)
5. [Success Metrics](#success-metrics)

---

## Problem 1: Code vs Docs Priority

### Current Behavior
- Query "How does retrieval pipeline work?" returns benchmark notes first
- Documentation matches keywords better than actual code
- Fields ranked higher than functions (shorter names = higher BM25)

### Root Cause Analysis

**Location:** `src/retrieval/hybrid/mod.rs` and `src/indexer/search.rs`

Current scoring is pure RRF fusion without document type awareness:
```rust
// Current: All documents treated equally
let rrf = fusion::rrf_score(ranked.rank, self.config.rrf_k) * keyword_weight;
```

No distinction between:
- Source code (`.rs` files)
- Documentation (`.md` in `doc/`)
- Notes (`.md` in `notes/`)
- Benchmark files (`.md` in `benchmarks/`)

### Solution: Document Type Boosting

#### Research Findings

From [GitHub Docs search](https://github.blog/engineering/architecture-optimization/how-github-docs-new-search-works/):
> A match on the title has a slightly higher boost than a match on the content. Constants: `BOOST_TITLE = 4.0`, `BOOST_HEADINGS = 3.0`, `BOOST_CONTENT = 1.0`

From [Tantivy BoostQuery](https://docs.rs/tantivy/latest/tantivy/query/struct.BoostQuery.html):
> The score of each document is the score of the underlying query multiplied by the boost factor.

From [Google Cloud Search](https://developers.google.com/cloud-search/docs/guides/improve-search-quality):
> Items from a data source with HIGH source importance receive a ranking boost compared to items from a data source with a DEFAULT or a LOW source importance.

#### Implementation Plan

##### Step 1.1: Add Document Type Classification

**File:** `src/indexer/symbols.rs` (new enum)

```rust
/// Document type for scoring purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
    /// Source code (.rs, .py, .js, etc.)
    SourceCode,
    /// Official documentation (doc/*.md)
    Documentation,
    /// Implementation notes (notes/*.md)
    Notes,
    /// Benchmark/test notes (notes/benchmarks/*.md)
    Benchmark,
    /// Test files (*_test.rs, tests/*.rs)
    Test,
}

impl DocumentType {
    /// Classify a file path into a document type
    pub fn from_path(path: &Path) -> Self {
        let path_str = path.to_string_lossy();
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        // Check benchmarks first (most specific)
        if path_str.contains("/benchmarks/") {
            return Self::Benchmark;
        }

        // Check notes
        if path_str.contains("/notes/") {
            return Self::Notes;
        }

        // Check documentation
        if path_str.contains("/doc/") && extension == "md" {
            return Self::Documentation;
        }

        // Check test files
        if path_str.contains("/tests/") || path_str.ends_with("_test.rs") {
            return Self::Test;
        }

        // Check source code extensions
        match extension {
            "rs" | "py" | "js" | "ts" | "go" | "java" | "c" | "cpp" | "h" => Self::SourceCode,
            "md" | "txt" => Self::Documentation,
            _ => Self::SourceCode, // default to source
        }
    }

    /// Get the boost factor for this document type
    pub fn boost_factor(&self) -> f32 {
        match self {
            Self::SourceCode => 1.5,      // Highest priority
            Self::Documentation => 1.0,   // Normal priority
            Self::Test => 0.9,            // Slightly lower
            Self::Notes => 0.7,           // Lower priority
            Self::Benchmark => 0.3,       // Much lower priority
        }
    }
}
```

##### Step 1.2: Add Symbol Kind Boosting

**File:** `src/indexer/symbols.rs` (add method to SymbolKind)

```rust
impl SymbolKind {
    /// Get boost factor for ranking
    /// Functions/methods ranked higher than fields/constants
    pub fn boost_factor(&self) -> f32 {
        match self {
            // High priority: actual code structures
            SymbolKind::Function => 1.4,
            SymbolKind::Method => 1.4,
            SymbolKind::Struct => 1.3,
            SymbolKind::Enum => 1.3,
            SymbolKind::Trait => 1.3,
            SymbolKind::Impl => 1.2,
            SymbolKind::Module => 1.1,

            // Medium priority
            SymbolKind::Constant => 0.9,
            SymbolKind::Static => 0.9,
            SymbolKind::TypeAlias => 0.9,

            // Lower priority
            SymbolKind::Field => 0.7,
            SymbolKind::Variant => 0.8,
            SymbolKind::Macro => 1.0,

            // Documentation symbols
            SymbolKind::DocumentChunk => 0.6,
        }
    }
}
```

##### Step 1.3: Apply Boosts in Hybrid Search

**File:** `src/retrieval/hybrid/mod.rs`

```rust
/// Fuse keyword and semantic results with document type boosting
fn fuse_with_boosts(
    &self,
    keyword_results: Vec<RankedItem<SearchHit>>,
    semantic_results: Vec<RankedItem<VectorSearchResult>>,
    keyword_weight: f32,
    semantic_weight: f32,
) -> Vec<HybridSearchResult> {
    use std::collections::HashMap;

    let mut results_by_key: HashMap<String, HybridSearchResult> = HashMap::new();

    // Process keyword results with document type and kind boosts
    for ranked in keyword_results {
        let key = format!(
            "{}:{}",
            ranked.item.symbol.name, ranked.item.symbol.location.line
        );

        // Calculate combined boost
        let doc_type = DocumentType::from_path(&ranked.item.symbol.location.file);
        let doc_boost = doc_type.boost_factor();
        let kind_boost = ranked.item.symbol.kind.boost_factor();
        let combined_boost = doc_boost * kind_boost;

        // Apply boost to RRF score
        let rrf = fusion::rrf_score(ranked.rank, self.config.rrf_k)
            * keyword_weight
            * combined_boost;

        results_by_key
            .entry(key.clone())
            .and_modify(|r| {
                r.rrf_score += rrf;
                r.keyword_rank = Some(ranked.rank);
                r.keyword_score = Some(ranked.score);
            })
            .or_insert(HybridSearchResult {
                symbol: ranked.item.symbol,
                rrf_score: rrf,
                keyword_rank: Some(ranked.rank),
                semantic_rank: None,
                keyword_score: Some(ranked.score),
                semantic_distance: None,
                rerank_score: None,
            });
    }

    // Same for semantic results...
    // [similar code with boosts applied]

    let mut results: Vec<_> = results_by_key.into_values().collect();
    results.sort_by(|a, b| b.rrf_score.partial_cmp(&a.rrf_score).unwrap());
    results
}
```

##### Step 1.4: Add Query-Aware Boosting

**File:** `src/retrieval/hybrid/adaptive.rs`

For conceptual queries like "how does X work", boost source code even more:

```rust
/// Compute adaptive weights based on query characteristics
pub fn compute_weights_with_doc_boost(query: &str) -> (AdaptiveWeights, f32) {
    let query_type = classify_query(query);
    let base_weights = weights_for_type(query_type);

    // For conceptual queries, increase source code boost
    let source_code_boost = match query_type {
        QueryType::Conceptual => 2.0,  // Strong boost for source code
        QueryType::Usage => 1.5,       // Moderate boost
        _ => 1.0,                      // Normal
    };

    (base_weights, source_code_boost)
}
```

##### Step 1.5: Alternative - Use Tantivy BoostQuery

**File:** `src/indexer/search.rs`

Wrap queries with BoostQuery based on document type:

```rust
use tantivy::query::BoostQuery;

/// Search with document type boosting applied at query level
pub fn search_with_boost(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>> {
    let reader = self.reader()?;
    let searcher = reader.searcher();

    // Parse base query
    let query_parser = QueryParser::for_index(
        &self.index,
        vec![self.fields.symbol_name, self.fields.content],
    );
    let base_query = query_parser.parse_query(query)?;

    // Create boost queries for different document types
    // This requires indexing document_type as a field
    let source_filter = /* filter for source code */;
    let boosted_query = BoostQuery::new(
        Box::new(BooleanQuery::intersection(vec![
            base_query.clone(),
            source_filter,
        ])),
        1.5, // boost factor
    );

    // Search and collect
    let top_docs = searcher.search(&boosted_query, &TopDocs::with_limit(limit))?;
    // ...
}
```

---

## Problem 2: Cross-File Usage Depth

### Current Behavior
- SemanticGraph shows 2 usages
- Augment shows 8+ usages across multiple files
- `max_callers` and `max_callees` limited to 5

### Root Cause Analysis

**Location:** `src/retrieval/context/mod.rs`

```rust
impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            max_callers: 5,      // Too low!
            max_callees: 5,      // Too low!
            context_lines_before: 3,
            context_lines_after: 10,
            include_parent: true,
            include_related_types: true,
        }
    }
}
```

**Location:** `src/retrieval/context/block_builder.rs`

```rust
fn collect_validated_usages(&self, symbol: &Symbol) -> Vec<UsageInfo> {
    let refs = self.graph.validated_references(symbol);
    let max_usages = 20; // Hardcoded limit
    // ...
}
```

**Location:** `src/retrieval/context/graph_walker.rs`

The callee detection only looks within 50 lines:
```rust
if ref_line >= sym_line && ref_line <= sym_line + 50 {
    // ... find callee
}
```

### Solution: Expanded Cross-Reference Collection

#### Research Findings

From [Sourcegraph](https://sourcegraph.com/docs/code-search/features):
> Personalized ranking with boosted results from repos the user recently contributed to.

From the existing `cross-reference-improvement-plan.md`:
> Proposed flow: Query → Expand → Hybrid Search → Rerank → Collect Usages → Format XML

#### Implementation Plan

##### Step 2.1: Increase Default Limits

**File:** `src/retrieval/context/mod.rs`

```rust
impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            max_callers: 15,           // Increased from 5
            max_callees: 15,           // Increased from 5
            max_usages_per_symbol: 30, // NEW: explicit limit
            context_lines_before: 3,
            context_lines_after: 15,   // Increased from 10
            include_parent: true,
            include_related_types: true,
            include_transitive: false, // NEW: for future expansion
        }
    }
}

/// Extended config for deep context expansion
pub struct ContextConfig {
    pub max_callers: usize,
    pub max_callees: usize,
    pub max_usages_per_symbol: usize,  // NEW
    pub context_lines_before: usize,
    pub context_lines_after: usize,
    pub include_parent: bool,
    pub include_related_types: bool,
    pub include_transitive: bool,       // NEW
}
```

##### Step 2.2: Add Comprehensive Usage Collection

**File:** `src/retrieval/context/graph_walker.rs`

```rust
impl<'a> GraphWalker<'a> {
    /// Find ALL usages of a symbol across the codebase
    /// Returns usages grouped by file for better organization
    pub fn find_all_usages(&self, symbol: &Symbol) -> UsageCollection {
        let mut usages = UsageCollection::new();

        // 1. Find direct references by name
        let refs = self.graph.find_references(&symbol.name);
        for reference in refs {
            // Validate this reference actually points to our symbol
            if self.graph.validate_reference(&reference, symbol) {
                usages.add(UsageInfo {
                    file: reference.location.file.clone(),
                    line: reference.location.line,
                    context: reference.context.clone(),
                    snippet: self.extract_snippet(&reference.location, 3),
                    containing_symbol: self.find_containing_symbol(
                        &reference.location.file,
                        reference.location.line,
                    ),
                });
            }
        }

        // 2. Find type usages (for structs, enums, traits)
        if matches!(symbol.kind, SymbolKind::Struct | SymbolKind::Enum | SymbolKind::Trait) {
            let type_refs = self.graph.find_type_usages(&symbol.name);
            for type_ref in type_refs {
                if !usages.contains(&type_ref.location) {
                    usages.add(UsageInfo {
                        file: type_ref.location.file.clone(),
                        line: type_ref.location.line,
                        context: ReferenceContext::Type,
                        snippet: self.extract_snippet(&type_ref.location, 3),
                        containing_symbol: self.find_containing_symbol(
                            &type_ref.location.file,
                            type_ref.location.line,
                        ),
                    });
                }
            }
        }

        // 3. Find impl block usages (for traits)
        if symbol.kind == SymbolKind::Trait {
            let impl_refs = self.find_trait_implementations(&symbol.name);
            for impl_ref in impl_refs {
                usages.add(impl_ref);
            }
        }

        // Sort by file path for grouped display
        usages.sort_by_file();

        // Apply limit
        usages.truncate(self.config.max_usages_per_symbol);

        usages
    }

    /// Extract snippet with context lines
    fn extract_snippet(&self, location: &CodeLocation, context_lines: usize) -> Option<String> {
        let file_content = std::fs::read_to_string(&location.file).ok()?;
        let lines: Vec<&str> = file_content.lines().collect();

        let start = location.line.saturating_sub(context_lines);
        let end = (location.line + context_lines).min(lines.len());

        let snippet: String = lines[start..end]
            .iter()
            .enumerate()
            .map(|(i, line)| format!("{:>4} | {}", start + i + 1, line))
            .collect::<Vec<_>>()
            .join("\n");

        Some(snippet)
    }
}

/// Collection of usages with deduplication
pub struct UsageCollection {
    usages: Vec<UsageInfo>,
    seen: HashSet<(PathBuf, usize)>, // (file, line)
}

impl UsageCollection {
    pub fn new() -> Self {
        Self {
            usages: Vec::new(),
            seen: HashSet::new(),
        }
    }

    pub fn add(&mut self, usage: UsageInfo) {
        let key = (usage.file.clone(), usage.line);
        if !self.seen.contains(&key) {
            self.seen.insert(key);
            self.usages.push(usage);
        }
    }

    pub fn contains(&self, location: &CodeLocation) -> bool {
        self.seen.contains(&(location.file.clone(), location.line))
    }

    pub fn sort_by_file(&mut self) {
        self.usages.sort_by(|a, b| {
            a.file.cmp(&b.file).then(a.line.cmp(&b.line))
        });
    }

    pub fn truncate(&mut self, limit: usize) {
        self.usages.truncate(limit);
    }

    pub fn into_vec(self) -> Vec<UsageInfo> {
        self.usages
    }
}
```

##### Step 2.3: Add Type-Aware Reference Tracking

**File:** `src/indexer/semantic.rs`

```rust
impl SemanticGraph {
    /// Find where a type is used in signatures, fields, etc.
    pub fn find_type_usages(&self, type_name: &str) -> Vec<SymbolReference> {
        self.references_by_name
            .get(type_name)
            .map(|refs| {
                refs.iter()
                    .filter(|r| matches!(
                        r.context,
                        ReferenceContext::Type |
                        ReferenceContext::FieldType |
                        ReferenceContext::ReturnType |
                        ReferenceContext::ParameterType
                    ))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Find all implementations of a trait
    pub fn find_trait_implementations(&self, trait_name: &str) -> Vec<Definition> {
        self.definitions_by_name
            .values()
            .flatten()
            .filter(|def| {
                def.symbol.kind == SymbolKind::Impl &&
                def.symbol.signature
                    .as_ref()
                    .map(|s| s.contains(trait_name))
                    .unwrap_or(false)
            })
            .cloned()
            .collect()
    }
}
```

##### Step 2.4: Expand Reference Context Types

**File:** `src/indexer/semantic.rs`

```rust
/// Context in which a reference appears
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceContext {
    // Existing
    Call,
    Type,
    Import,

    // NEW: More specific contexts
    FieldType,        // Used as field type: `field: MyType`
    ReturnType,       // Used as return type: `-> MyType`
    ParameterType,    // Used as parameter type: `fn foo(x: MyType)`
    GenericArg,       // Used as generic argument: `Vec<MyType>`
    TraitBound,       // Used as trait bound: `T: MyTrait`
    ImplTarget,       // Target of impl: `impl MyTrait for X`
    MacroUse,         // Used in macro invocation
    Attribute,        // Used in attribute: `#[derive(MyDerive)]`
    TestAssertion,    // Used in test assertion
}
```

##### Step 2.5: Update XML Output Format

**File:** `src/retrieval/context/mod.rs`

```rust
impl ContextualBlock {
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<context-block>\n");

        // Symbol info
        xml.push_str(&format!(
            "  <symbol kind=\"{}\" name=\"{}\" file=\"{}\" line=\"{}\" usages=\"{}\">\n",
            self.symbol.kind,
            self.symbol.name,
            self.symbol.location.file.display(),
            self.symbol.location.line,
            self.usage_count
        ));

        // ... code snippet ...

        // Enhanced usages section with grouping by file
        if !self.usages.is_empty() {
            xml.push_str(&format!("  <usages total=\"{}\">\n", self.usages.len()));

            // Group by file for readability
            let mut by_file: HashMap<&Path, Vec<&UsageInfo>> = HashMap::new();
            for usage in &self.usages {
                by_file.entry(&usage.file).or_default().push(usage);
            }

            for (file, file_usages) in by_file {
                xml.push_str(&format!("    <file path=\"{}\">\n", file.display()));
                for usage in file_usages {
                    xml.push_str(&format!(
                        "      <usage line=\"{}\" context=\"{:?}\"",
                        usage.line,
                        usage.context
                    ));
                    if let Some(ref containing) = usage.containing_symbol {
                        xml.push_str(&format!(" in=\"{}\"", containing));
                    }
                    xml.push_str(">\n");
                    if let Some(ref snippet) = usage.snippet {
                        xml.push_str(&format!("        {}\n", escape_xml(snippet)));
                    }
                    xml.push_str("      </usage>\n");
                }
                xml.push_str("    </file>\n");
            }

            xml.push_str("  </usages>\n");
        }

        xml.push_str("</context-block>\n");
        xml
    }
}
```

---

## Problem 3: Daemon Stability

### Current Behavior
- Error: `Resource temporarily unavailable (os error 35)` after ~30 minutes
- Socket communication fails during reranking
- Query crashes instead of graceful degradation

### Root Cause Analysis

**Location:** `src/retrieval/daemon/client.rs`

Current implementation has no retry logic:
```rust
fn send_request(&self, request: &DaemonRequest) -> RetrievalResult<DaemonResponse> {
    let mut stream = self.connect()?;  // No retry on failure
    // ...
    reader.read_line(&mut line)?;  // No retry on read failure
    // ...
}
```

**Location:** `src/retrieval/daemon/server.rs`

Uses synchronous, blocking I/O which can cause issues:
```rust
loop {
    match listener.accept() {
        Ok((stream, _)) => {
            stream.set_nonblocking(false)?;  // Blocking mode
            if let Err(e) = self.handle_client(stream) {
                eprintln!("[daemon] client error: {}", e);
            }
        }
        // ...
    }
}
```

### Solution: Robust Socket Communication

#### Research Findings

From [Tokio documentation](https://docs.rs/tokio/latest/tokio/net/struct.UnixStream.html):
> The recommendation is to rely on the "shutdown()-followed-by-read()-eof technique"

From [Building Robust Servers](https://matthewtejo.substack.com/p/building-robust-server-with-async):
> Having one big thread pool handle everything means the blast radius is the entire application. A recommended approach is to start with three thread pools.

From [Named Pipe Retry Logic](https://docs.rs/tokio/latest/tokio/net/windows/named_pipe/struct.ClientOptions.html):
```rust
let client = loop {
    match ClientOptions::new().open(PIPE_NAME) {
        Ok(client) => break client,
        Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY as i32) => (),
        Err(e) => return Err(e),
    }
    time::sleep(Duration::from_millis(50)).await;
};
```

#### Implementation Plan

##### Step 3.1: Add Retry Logic to Client

**File:** `src/retrieval/daemon/client.rs`

```rust
/// Configuration for retry behavior
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: usize,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Multiplier for exponential backoff
    pub backoff_multiplier: f32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(2),
            backoff_multiplier: 2.0,
        }
    }
}

impl DaemonClient {
    /// Send a request with retry logic
    fn send_request_with_retry(&self, request: &DaemonRequest) -> RetrievalResult<DaemonResponse> {
        let config = RetryConfig::default();
        let mut delay = config.initial_delay;
        let mut last_error = None;

        for attempt in 0..=config.max_retries {
            match self.send_request_once(request) {
                Ok(response) => return Ok(response),
                Err(e) => {
                    // Check if error is retryable
                    if !Self::is_retryable_error(&e) {
                        return Err(e);
                    }

                    last_error = Some(e);

                    if attempt < config.max_retries {
                        eprintln!(
                            "[daemon-client] Retry {}/{} after {:?}",
                            attempt + 1,
                            config.max_retries,
                            delay
                        );
                        std::thread::sleep(delay);

                        // Exponential backoff
                        delay = Duration::from_millis(
                            (delay.as_millis() as f32 * config.backoff_multiplier) as u64
                        ).min(config.max_delay);
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            RetrievalError::DaemonCommunication("max retries exceeded".to_string())
        }))
    }

    /// Check if an error is transient and worth retrying
    fn is_retryable_error(error: &RetrievalError) -> bool {
        match error {
            RetrievalError::Io(io_err) => {
                matches!(
                    io_err.kind(),
                    std::io::ErrorKind::WouldBlock |
                    std::io::ErrorKind::TimedOut |
                    std::io::ErrorKind::Interrupted |
                    std::io::ErrorKind::ConnectionReset |
                    std::io::ErrorKind::BrokenPipe
                )
            }
            RetrievalError::DaemonCommunication(msg) => {
                msg.contains("temporarily unavailable") ||
                msg.contains("Resource temporarily unavailable") ||
                msg.contains("connection reset")
            }
            _ => false,
        }
    }

    /// Single attempt to send request
    fn send_request_once(&self, request: &DaemonRequest) -> RetrievalResult<DaemonResponse> {
        let mut stream = self.connect()?;

        // Set socket options for reliability
        stream.set_read_timeout(Some(self.timeout))?;
        stream.set_write_timeout(Some(self.timeout))?;

        // Serialize and send
        let bytes = serialize_request(request)
            .map_err(|e| RetrievalError::DaemonCommunication(format!("serialize: {}", e)))?;

        stream.write_all(&bytes)?;
        stream.flush()?;

        // Read response with explicit error handling
        let mut reader = BufReader::new(&stream);
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => {
                return Err(RetrievalError::DaemonCommunication(
                    "daemon closed connection".to_string()
                ));
            }
            Ok(_) => {}
            Err(e) => {
                return Err(RetrievalError::Io(e));
            }
        }

        deserialize_response(line.trim().as_bytes())
            .map_err(|e| RetrievalError::DaemonCommunication(format!("deserialize: {}", e)))
    }
}
```

##### Step 3.2: Add Connection Pooling

**File:** `src/retrieval/daemon/client.rs`

```rust
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

/// Connection pool for daemon communication
pub struct ConnectionPool {
    /// Available connections
    connections: Mutex<VecDeque<UnixStream>>,
    /// Socket path
    socket_path: PathBuf,
    /// Maximum pool size
    max_size: usize,
    /// Connection timeout
    timeout: Duration,
}

impl ConnectionPool {
    pub fn new(socket_path: PathBuf, max_size: usize) -> Self {
        Self {
            connections: Mutex::new(VecDeque::new()),
            socket_path,
            max_size,
            timeout: Duration::from_secs(30),
        }
    }

    /// Get a connection from the pool or create new one
    pub fn get(&self) -> RetrievalResult<PooledConnection> {
        // Try to get existing connection
        if let Some(stream) = self.connections.lock().unwrap().pop_front() {
            // Verify connection is still valid with ping
            if Self::is_connection_valid(&stream) {
                return Ok(PooledConnection {
                    stream: Some(stream),
                    pool: self,
                });
            }
            // Connection invalid, will create new one
        }

        // Create new connection
        let stream = UnixStream::connect(&self.socket_path)
            .map_err(|e| RetrievalError::DaemonNotRunning(e.to_string()))?;
        stream.set_read_timeout(Some(self.timeout)).ok();
        stream.set_write_timeout(Some(self.timeout)).ok();

        Ok(PooledConnection {
            stream: Some(stream),
            pool: self,
        })
    }

    /// Return a connection to the pool
    fn return_connection(&self, stream: UnixStream) {
        let mut connections = self.connections.lock().unwrap();
        if connections.len() < self.max_size {
            connections.push_back(stream);
        }
        // If pool is full, connection is dropped
    }

    /// Check if connection is still valid
    fn is_connection_valid(stream: &UnixStream) -> bool {
        // Try to set non-blocking temporarily
        if stream.set_nonblocking(true).is_err() {
            return false;
        }

        // Try a zero-byte read to check connection state
        let mut buf = [0u8; 0];
        let valid = match stream.peek(&mut buf) {
            Ok(_) => true,
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => true,
            Err(_) => false,
        };

        // Restore blocking mode
        let _ = stream.set_nonblocking(false);
        valid
    }
}

/// RAII wrapper for pooled connection
pub struct PooledConnection<'a> {
    stream: Option<UnixStream>,
    pool: &'a ConnectionPool,
}

impl<'a> Drop for PooledConnection<'a> {
    fn drop(&mut self) {
        if let Some(stream) = self.stream.take() {
            self.pool.return_connection(stream);
        }
    }
}

impl<'a> std::ops::Deref for PooledConnection<'a> {
    type Target = UnixStream;
    fn deref(&self) -> &Self::Target {
        self.stream.as_ref().unwrap()
    }
}

impl<'a> std::ops::DerefMut for PooledConnection<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.stream.as_mut().unwrap()
    }
}
```

##### Step 3.3: Add Graceful Degradation

**File:** `src/retrieval/agent/pipeline.rs`

```rust
impl RetrievalPipeline {
    /// Retrieve with graceful degradation on daemon errors
    pub fn retrieve(&mut self, query: &str) -> RetrievalResult<RetrievalOutput> {
        // Initialize if needed
        self.initialize()?;

        // Step 1: Query Expansion (with fallback)
        let search_spec = match self.expand_query_safe(query) {
            Ok(spec) => spec,
            Err(e) => {
                eprintln!("[pipeline] Query expansion failed, using fallback: {}", e);
                self.fallback_query_expansion(query)
            }
        };

        // Step 2: Hybrid Search
        let fetch_limit = if self.config.rerank {
            self.config.max_results * 3
        } else {
            self.config.max_results
        };

        let mut results = self.search(&search_spec, fetch_limit)?;
        eprintln!("[pipeline] Found {} candidates", results.len());

        // Step 3: Reranking (with fallback)
        if self.config.rerank && !results.is_empty() {
            match self.rerank_safe(query, &results) {
                Ok(reranked) => {
                    results = reranked;
                }
                Err(e) => {
                    eprintln!("[pipeline] Reranking failed, using RRF scores: {}", e);
                    // Keep results sorted by RRF score (already sorted)
                }
            }
            results.truncate(self.config.max_results);
        }

        // Step 4: Context Expansion
        let xml_output = self.expand_context(&results)?;

        Ok(RetrievalOutput {
            query: query.to_string(),
            search_spec,
            xml_output,
            result_count: results.len(),
            token_count: xml_output.len() / 4,
            has_more: results.len() >= self.config.max_results,
        })
    }

    /// Safe query expansion with timeout
    fn expand_query_safe(&self, query: &str) -> RetrievalResult<SearchSpec> {
        // Use a timeout to prevent hanging
        let (tx, rx) = std::sync::mpsc::channel();
        let daemon = self.daemon.clone();
        let query = query.to_string();

        std::thread::spawn(move || {
            let result = daemon.expand(query);
            let _ = tx.send(result);
        });

        // Wait with timeout
        match rx.recv_timeout(Duration::from_secs(10)) {
            Ok(result) => result,
            Err(_) => Err(RetrievalError::DaemonCommunication(
                "query expansion timed out".to_string()
            )),
        }
    }

    /// Safe reranking with timeout
    fn rerank_safe(
        &self,
        query: &str,
        results: &[HybridSearchResult],
    ) -> RetrievalResult<Vec<HybridSearchResult>> {
        // Similar timeout pattern...
        self.rerank(query, results.to_vec())
    }

    /// Fallback query expansion when daemon fails
    fn fallback_query_expansion(&self, query: &str) -> SearchSpec {
        use crate::retrieval::query::FastPathParser;

        let parser = FastPathParser::new();
        let fast_result = parser.extract_symbols(query);

        SearchSpec {
            original_query: query.to_string(),
            symbol_names: fast_result.symbols,
            intent: match fast_result.intent {
                crate::retrieval::query::FastPathIntent::Symbol =>
                    crate::retrieval::daemon::protocol::QueryIntent::Search,
                crate::retrieval::query::FastPathIntent::Definition =>
                    crate::retrieval::daemon::protocol::QueryIntent::FindDefinition,
                crate::retrieval::query::FastPathIntent::Usage =>
                    crate::retrieval::daemon::protocol::QueryIntent::FindUsages,
                crate::retrieval::query::FastPathIntent::Conceptual =>
                    crate::retrieval::daemon::protocol::QueryIntent::Understand,
                crate::retrieval::query::FastPathIntent::Debug =>
                    crate::retrieval::daemon::protocol::QueryIntent::Debug,
            },
            file_filters: Vec::new(),
            context_hints: Vec::new(),
        }
    }
}
```

##### Step 3.4: Add Health Check & Auto-Recovery

**File:** `src/retrieval/daemon/lifecycle.rs`

```rust
/// Health check result
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub healthy: bool,
    pub latency_ms: u64,
    pub error: Option<String>,
}

/// Check daemon health with latency measurement
pub fn health_check(socket_path: &Path) -> HealthStatus {
    let start = std::time::Instant::now();

    let client = DaemonClient::with_socket_path(socket_path);
    match client.ping() {
        Ok(true) => HealthStatus {
            healthy: true,
            latency_ms: start.elapsed().as_millis() as u64,
            error: None,
        },
        Ok(false) => HealthStatus {
            healthy: false,
            latency_ms: start.elapsed().as_millis() as u64,
            error: Some("ping returned false".to_string()),
        },
        Err(e) => HealthStatus {
            healthy: false,
            latency_ms: start.elapsed().as_millis() as u64,
            error: Some(e.to_string()),
        },
    }
}

/// Auto-recover daemon if unhealthy
pub fn ensure_healthy_daemon() -> bool {
    let config = RetrievalConfig::default();
    let socket_path = &config.socket_path;

    let health = health_check(socket_path);

    if health.healthy {
        return true;
    }

    eprintln!("[daemon] Unhealthy: {:?}", health.error);
    eprintln!("[daemon] Attempting recovery...");

    // Stop existing daemon
    if let Err(e) = stop_daemon(socket_path) {
        eprintln!("[daemon] Stop failed: {}", e);
    }

    // Wait for cleanup
    std::thread::sleep(Duration::from_millis(500));

    // Start fresh
    if let Err(e) = start_daemon(socket_path) {
        eprintln!("[daemon] Start failed: {}", e);
        return false;
    }

    // Wait for startup
    ensure_daemon_ready()
}
```

##### Step 3.5: Optional - Migrate to Async Tokio

For long-term stability, consider migrating to async Tokio:

**File:** `src/retrieval/daemon/async_client.rs` (new file)

```rust
use tokio::net::UnixStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub struct AsyncDaemonClient {
    socket_path: PathBuf,
    timeout: Duration,
}

impl AsyncDaemonClient {
    pub async fn send_request(&self, request: &DaemonRequest) -> RetrievalResult<DaemonResponse> {
        let stream = UnixStream::connect(&self.socket_path).await
            .map_err(|e| RetrievalError::DaemonNotRunning(e.to_string()))?;

        let (reader, mut writer) = stream.into_split();

        // Write request
        let bytes = serialize_request(request)
            .map_err(|e| RetrievalError::DaemonCommunication(format!("serialize: {}", e)))?;

        writer.write_all(&bytes).await?;
        writer.flush().await?;

        // Read response with timeout
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        tokio::time::timeout(self.timeout, reader.read_line(&mut line))
            .await
            .map_err(|_| RetrievalError::DaemonCommunication("timeout".to_string()))??;

        deserialize_response(line.trim().as_bytes())
            .map_err(|e| RetrievalError::DaemonCommunication(format!("deserialize: {}", e)))
    }
}
```

---

## Implementation Order

### Phase 1: Quick Wins (1-2 days)

1. **Increase context limits** (Step 2.1)
   - Change `max_callers: 5` → `15`
   - Change `max_callees: 5` → `15`
   - Change `max_usages: 20` → `30`
   - **Immediate impact, minimal code change**

2. **Add retry logic** (Step 3.1)
   - Implement `send_request_with_retry()`
   - Add `is_retryable_error()` check
   - **Fixes the crash issue**

### Phase 2: Document Type Boosting (2-3 days)

3. **Add DocumentType enum** (Step 1.1)
4. **Add SymbolKind boost factors** (Step 1.2)
5. **Apply boosts in fusion** (Step 1.3)

### Phase 3: Enhanced Cross-References (3-4 days)

6. **Expand ReferenceContext enum** (Step 2.4)
7. **Implement comprehensive usage collection** (Step 2.2)
8. **Add type-aware reference tracking** (Step 2.3)
9. **Update XML output format** (Step 2.5)

### Phase 4: Stability Improvements (2-3 days)

10. **Add connection pooling** (Step 3.2)
11. **Add graceful degradation** (Step 3.3)
12. **Add health check & auto-recovery** (Step 3.4)

### Phase 5: Optional Enhancements

13. **Migrate to async Tokio** (Step 3.5)
14. **Add Tantivy BoostQuery** (Step 1.5)

---

## Success Metrics

### Before vs After Targets

| Metric | Current | Target |
|--------|---------|--------|
| Query "pipeline" returns code first | ❌ No (notes first) | ✅ Yes |
| Query "RRF" returns function first | ❌ No (field first) | ✅ Yes |
| Usages shown for SemanticGraph | 2 | 10+ |
| Daemon stability (uptime) | ~30 min | 24+ hours |
| Retry success rate | 0% | 95%+ |

### Benchmark Queries to Retest

```bash
# After implementation, run these queries:
./target/release/ch-cli retrieve "How does the retrieval pipeline work?"
# Expected: pipeline.rs first, not benchmark notes

./target/release/ch-cli retrieve "RRF fusion algorithm"
# Expected: rrf_score() function first, not rrf_score field

./target/release/ch-cli retrieve "where is SemanticGraph used"
# Expected: 10+ usages across multiple files

# Stability test:
for i in {1..100}; do
    ./target/release/ch-cli retrieve "BgeEmbedder" > /dev/null
    sleep 1
done
# Expected: 0 failures
```

---

## References

### Research Sources

- [Hybrid Search & Reranking Best Practices](https://superlinked.com/vectorhub/articles/optimizing-rag-with-hybrid-search-reranking)
- [Qdrant Reranking Tutorial](https://qdrant.tech/documentation/advanced-tutorials/reranking-hybrid-search/)
- [GitHub Docs Search Architecture](https://github.blog/engineering/architecture-optimization/how-github-docs-new-search-works/)
- [Tantivy BoostQuery](https://docs.rs/tantivy/latest/tantivy/query/struct.BoostQuery.html)
- [Tokio Unix Socket Handling](https://docs.rs/tokio/latest/tokio/net/struct.UnixStream.html)
- [Building Robust Async Servers](https://matthewtejo.substack.com/p/building-robust-server-with-async)

### Internal Documentation

- `notes/plans/cross-reference-improvement-plan.md`
- `notes/implementation/daemon-socket-timing-fix.md`
- `doc/retrieval/hybrid-search.md`
- `doc/retrieval/context-expansion.md`
