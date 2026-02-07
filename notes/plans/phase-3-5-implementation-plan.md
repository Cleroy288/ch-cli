# Phase 3-5 Implementation Plan

**Date:** 2026-02-03
**Status:** Planning
**Prerequisites:** Phase 1 (context limits, retry logic) and Phase 2 (document type boosting) completed

---

## Summary

| Phase | Focus | Tasks | Files Affected |
|-------|-------|-------|----------------|
| Phase 3 | Enhanced Cross-References | 4 tasks | semantic.rs, graph_walker.rs, block_builder.rs |
| Phase 4 | Stability Improvements | 3 tasks | client.rs, pipeline.rs, lifecycle.rs |
| Phase 5 | Optional Enhancements | 2 tasks | async_client.rs (new), search.rs |

---

## Phase 3: Enhanced Cross-References

**Goal:** Increase from 2 usages to 10+ usages shown, with better context and grouping.

### Task 3.1: Expand ReferenceContext Enum

**File:** `src/indexer/semantic.rs`

**Current state:**
```rust
pub enum ReferenceContext {
    Call,
    Type,
    FieldAccess,
    Import,
    Identifier,
    Unknown,
}
```

**Changes:**
- Add new variants for more specific type usages:
  - `FieldType` - used as field type: `field: MyType`
  - `ReturnType` - used as return type: `-> MyType`
  - `ParameterType` - used as parameter type: `fn foo(x: MyType)`
  - `GenericArg` - used as generic argument: `Vec<MyType>`
  - `TraitBound` - used as trait bound: `T: MyTrait`
  - `ImplTarget` - target of impl: `impl MyTrait for X`

**Unit tests:**
- Test each new context variant can be serialized/deserialized
- Test context detection in sample code patterns

---

### Task 3.2: Add Type-Aware Reference Tracking

**File:** `src/indexer/semantic.rs`

**Add new methods to `SemanticGraph`:**

```rust
/// Find where a type is used in signatures, fields, etc.
pub fn find_type_usages(&self, type_name: &str) -> Vec<SymbolReference>

/// Find all implementations of a trait
pub fn find_trait_implementations(&self, trait_name: &str) -> Vec<Definition>

/// Find all usages grouped by context type
pub fn find_usages_by_context(&self, name: &str) -> FxHashMap<ReferenceContext, Vec<SymbolReference>>
```

**Unit tests:**
- Test `find_type_usages` with struct used in field, parameter, return type
- Test `find_trait_implementations` with multiple impl blocks
- Test `find_usages_by_context` grouping

---

### Task 3.3: Implement UsageCollection with Deduplication

**File:** `src/retrieval/context/graph_walker.rs`

**Add new struct and methods:**

```rust
/// Collection of usages with deduplication
pub struct UsageCollection {
    usages: Vec<UsageInfo>,
    seen: HashSet<(PathBuf, usize)>, // (file, line)
}

impl UsageCollection {
    pub fn new() -> Self
    pub fn add(&mut self, usage: UsageInfo)
    pub fn contains(&self, file: &Path, line: usize) -> bool
    pub fn sort_by_file(&mut self)
    pub fn truncate(&mut self, limit: usize)
    pub fn into_vec(self) -> Vec<UsageInfo>
    pub fn len(&self) -> usize
}
```

**Add to `GraphWalker`:**

```rust
/// Find ALL usages of a symbol across the codebase
pub fn find_all_usages(&self, symbol: &Symbol) -> UsageCollection

/// Extract snippet with context lines
fn extract_snippet(&self, location: &CodeLocation, context_lines: usize) -> Option<String>
```

**Unit tests:**
- Test deduplication (same file+line not added twice)
- Test sorting by file
- Test truncation respects limit
- Test snippet extraction with context lines

---

### Task 3.4: Update XML Output with Grouped Usages

**File:** `src/retrieval/context/block_builder.rs`

**Changes:**
- Modify `build_block()` to use `find_all_usages()`
- Group usages by file in XML output
- Add containing symbol info to each usage

**New XML format:**
```xml
<usages total="12">
  <file path="src/retrieval/agent/pipeline.rs">
    <usage line="145" context="Call" in="retrieve">
      145 |     let graph = SemanticGraph::new();
    </usage>
    <usage line="302" context="Type" in="expand_context">
      302 |     graph: &SemanticGraph,
    </usage>
  </file>
  <file path="src/retrieval/context/mod.rs">
    <usage line="89" context="FieldType" in="ContextExpander">
      89 |     graph: &'a SemanticGraph,
    </usage>
  </file>
</usages>
```

**Unit tests:**
- Test XML output contains grouped usages
- Test total count matches actual usages
- Test containing symbol is correctly identified

---

## Phase 4: Stability Improvements

**Goal:** Daemon uptime from ~30 min to 24+ hours, with graceful degradation.

### Task 4.1: Add Connection Pooling

**File:** `src/retrieval/daemon/client.rs`

**Add new structs:**

```rust
/// Connection pool for daemon communication
pub struct ConnectionPool {
    connections: Mutex<VecDeque<UnixStream>>,
    socket_path: PathBuf,
    max_size: usize,
    timeout: Duration,
}

impl ConnectionPool {
    pub fn new(socket_path: PathBuf, max_size: usize) -> Self
    pub fn get(&self) -> RetrievalResult<PooledConnection>
    fn return_connection(&self, stream: UnixStream)
    fn is_connection_valid(stream: &UnixStream) -> bool
}

/// RAII wrapper for pooled connection
pub struct PooledConnection<'a> {
    stream: Option<UnixStream>,
    pool: &'a ConnectionPool,
}
```

**Modify `DaemonClient`:**
- Add `pool: Option<ConnectionPool>` field
- Use pool in `send_request()` when available
- Add `with_pool()` builder method

**Unit tests:**
- Test connection reuse from pool
- Test pool size limit respected
- Test invalid connection detection
- Test RAII cleanup returns connection to pool

---

### Task 4.2: Add Graceful Degradation

**File:** `src/retrieval/agent/pipeline.rs`

**Add new methods:**

```rust
/// Safe query expansion with timeout
fn expand_query_safe(&self, query: &str) -> RetrievalResult<SearchSpec>

/// Safe reranking with timeout
fn rerank_safe(&self, query: &str, results: &[HybridSearchResult]) -> RetrievalResult<Vec<HybridSearchResult>>

/// Fallback query expansion when daemon fails
fn fallback_query_expansion(&self, query: &str) -> SearchSpec
```

**Modify `retrieve()`:**
- Wrap query expansion in `expand_query_safe()`
- On failure, use `fallback_query_expansion()`
- Wrap reranking in `rerank_safe()`
- On failure, keep RRF-sorted results (already sorted)
- Log degradation events

**Unit tests:**
- Test fallback query expansion produces valid SearchSpec
- Test timeout handling doesn't panic
- Test degradation logs appropriate messages

---

### Task 4.3: Add Health Check & Auto-Recovery

**File:** `src/retrieval/daemon/lifecycle.rs`

**Add new structs and functions:**

```rust
/// Health check result
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub healthy: bool,
    pub latency_ms: u64,
    pub error: Option<String>,
}

/// Check daemon health with latency measurement
pub fn health_check(socket_path: &Path) -> HealthStatus

/// Auto-recover daemon if unhealthy
pub fn ensure_healthy_daemon() -> bool

/// Check and recover before critical operations
pub fn check_and_recover_if_needed() -> bool
```

**Integration:**
- Call `check_and_recover_if_needed()` before retrieval operations
- Add `--health` CLI flag to manually check daemon health
- Add health info to `daemon status` output

**Unit tests:**
- Test health check returns correct status
- Test latency measurement is reasonable
- Test auto-recovery attempts restart

---

## Phase 5: Optional Enhancements

**Goal:** Long-term improvements for performance and maintainability.

### Task 5.1: Migrate to Async Tokio (Optional)

**New file:** `src/retrieval/daemon/async_client.rs`

**Add async client:**

```rust
use tokio::net::UnixStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub struct AsyncDaemonClient {
    socket_path: PathBuf,
    timeout: Duration,
}

impl AsyncDaemonClient {
    pub async fn send_request(&self, request: &DaemonRequest) -> RetrievalResult<DaemonResponse>
    pub async fn embed(&self, texts: Vec<String>) -> RetrievalResult<Vec<Vec<f32>>>
    pub async fn rerank(&self, query: String, documents: Vec<String>) -> RetrievalResult<Vec<f32>>
    pub async fn expand(&self, query: String) -> RetrievalResult<SearchSpec>
}
```

**Migration path:**
1. Add `tokio` dependency with `rt-multi-thread` and `net` features
2. Create async versions of all daemon methods
3. Add feature flag `async-daemon` to switch between sync/async
4. Eventually deprecate sync client

**Considerations:**
- Breaking change for CLI integration
- Requires async runtime in main
- Benefits: better timeout handling, concurrent requests

---

### Task 5.2: Add Tantivy BoostQuery (Optional)

**File:** `src/indexer/search.rs`

**Changes:**
- Add `document_type` field to Tantivy schema
- Index `DocumentType` with each document
- Use `BoostQuery` to apply boosts at query time

```rust
use tantivy::query::BoostQuery;

/// Search with document type boosting at query level
pub fn search_with_boost(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>> {
    // Build base query
    let base_query = self.parse_query(query)?;

    // Create boosted query for source code
    let source_filter = TermQuery::new(
        Term::from_field_text(self.fields.document_type, "SourceCode"),
        IndexRecordOption::Basic,
    );

    let boosted = BooleanQuery::union(vec![
        (Occur::Should, Box::new(BoostQuery::new(base_query.clone(), 1.0))),
        (Occur::Should, Box::new(BoostQuery::new(
            BooleanQuery::intersection(vec![base_query, Box::new(source_filter)]),
            0.5, // Additional boost for source code
        ))),
    ]);

    // Search with boosted query
    self.search_with_query(&boosted, limit)
}
```

**Benefits:**
- Boost applied at search time (before RRF fusion)
- More efficient than post-processing
- Consistent with Tantivy best practices

---

## Implementation Order

```
Phase 3 (Cross-References):
  Task 3.1 → Task 3.2 → Task 3.3 → Task 3.4
  (Sequential - each depends on previous)

Phase 4 (Stability):
  Task 4.1 ─┐
  Task 4.2 ─┼→ Can run in parallel
  Task 4.3 ─┘

Phase 5 (Optional):
  Task 5.1 ─┐
  Task 5.2 ─┴→ Independent, can run in parallel
```

---

## Success Metrics

| Metric | Current | After Phase 3 | After Phase 4 | After Phase 5 |
|--------|---------|---------------|---------------|---------------|
| Usages shown | 2-3 | 10+ | 10+ | 10+ |
| Usage grouping | None | By file | By file | By file |
| Daemon uptime | ~30 min | ~30 min | 24+ hours | 24+ hours |
| Graceful degradation | No | No | Yes | Yes |
| Auto-recovery | No | No | Yes | Yes |
| Async support | No | No | No | Optional |

---

## Test Commands

```bash
# After Phase 3:
./target/release/ch-cli retrieve "where is SemanticGraph used"
# Expected: 10+ usages grouped by file

# After Phase 4:
for i in {1..100}; do
    ./target/release/ch-cli retrieve "BgeEmbedder" > /dev/null
    sleep 1
done
# Expected: 0 failures, auto-recovery if daemon crashes

# Health check:
./target/release/ch-cli daemon health
# Expected: Shows healthy status with latency
```

---

## Dependencies

### Phase 3
- No new dependencies

### Phase 4
- No new dependencies

### Phase 5
- `tokio = { version = "1", features = ["rt-multi-thread", "net", "time", "sync"] }`

---

## Risk Assessment

| Phase | Risk Level | Mitigation |
|-------|------------|------------|
| Phase 3 | Low | Backward compatible, adds functionality |
| Phase 4 | Medium | Connection pooling needs careful testing |
| Phase 5.1 | High | Breaking change, requires async runtime |
| Phase 5.2 | Low | Optional optimization, non-breaking |

---

## Estimated Effort

| Phase | Tasks | Estimated Effort |
|-------|-------|------------------|
| Phase 3 | 4 tasks | 3-4 days |
| Phase 4 | 3 tasks | 2-3 days |
| Phase 5 | 2 tasks | 2-3 days (optional) |

**Total:** 7-10 days for Phases 3-5

---

## Conclusion

- **Phase 3** focuses on showing more usages with better organization
- **Phase 4** ensures daemon reliability for production use
- **Phase 5** provides optional long-term improvements

Recommend implementing Phase 3 and 4 together, as they address the main quality gaps identified in benchmarks. Phase 5 can be deferred until async support is needed.
