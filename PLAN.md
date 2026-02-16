# Search Quality Improvements Plan

**Date:** 2026-02-05
**Status:** ✅ Completed
**Previous:** CodeRabbit Fixes (Completed)
**Estimated Effort:** 2-3 hours

---

## Summary

Three improvements to close gaps between rustean and Augment MCP:

| # | Improvement | Impact | Effort | Files |
|---|-------------|--------|--------|-------|
| 1 | Intent detection in basic search | HIGH | LOW | `commands.rs` |
| 2 | Add `--full` flag for file content | HIGH | LOW | `mod.rs`, `commands.rs`, `main.rs` |
| 3 | Stronger definition boosting | MEDIUM | LOW | `symbols.rs` |

---

## Improvement 1: Intent Detection in Basic Search

### Problem
Basic `rustean search` doesn't use intent detection. Queries like "BgeEmbedder struct" return documentation files instead of the actual struct definition.

### Current Flow
```
search_command(query)
  → SearchIndex.search(query)
  → Returns raw Tantivy hits (docs rank higher due to keyword density)
```

### Target Flow
```
search_command(query)
  → fallback_parse(query)              # detect intent
  → SearchIndex.search(query)          # get candidates
  → apply_intent_boosts(hits, intent)  # boost by intent
  → sort by boosted score
  → display results
```

### Changes

**File: `src/cli/commands.rs`**

Add imports at top:
```rust
use crate::retrieval::query::fallback_parse;
use crate::retrieval::daemon::protocol::QueryIntent;
use crate::indexer::DocumentType;
```

Modify `search_command()` (after line 146, where hits are returned):
```rust
// Detect intent from query
let spec = fallback_parse(query);
let intent = spec.intent;

// Apply intent-aware boosts
let mut boosted_hits: Vec<(SearchHit, f32)> = hits
    .into_iter()
    .map(|hit| {
        let doc_type = DocumentType::from_path(&hit.symbol.location.file);
        let doc_boost = doc_type.boost_factor_for_intent(&intent);
        let kind_boost = hit.symbol.kind.boost_factor_for_intent(&intent);
        let boosted_score = hit.score * doc_boost * kind_boost;
        (hit, boosted_score)
    })
    .collect();

// Sort by boosted score
boosted_hits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

// Extract hits
let hits: Vec<SearchHit> = boosted_hits.into_iter().map(|(h, _)| h).collect();
```

---

## Improvement 2: Add `--full` Flag

### Problem
Search only shows `file:line` pointers. Users want full file content like Augment MCP.

### Changes

**File: `src/cli/mod.rs`** (line ~69, in `Commands::Search`)

Add new flag:
```rust
/// Show full file content for results
#[arg(long)]
full: bool,
```

**File: `src/cli/commands.rs`**

Update signature:
```rust
pub fn search_command(
    query: &str,
    limit: usize,
    fuzzy: bool,
    kind: Option<&str>,
    semantic: bool,
    context: bool,
    rerank: bool,
    full: bool,  // NEW
) -> CommandResult {
```

Add display logic after results (around line 175):
```rust
if full && !hits.is_empty() {
    println!("\n--- Full Content ---\n");
    for hit in &hits {
        let path = &hit.symbol.location.file;
        println!("=== {} (line {}) ===", path.display(), hit.symbol.location.line);

        match std::fs::read_to_string(path) {
            Ok(content) => {
                let lines: Vec<&str> = content.lines().collect();
                let start = hit.symbol.location.line.saturating_sub(5);
                let end = (hit.symbol.location.line + 195).min(lines.len());

                for (i, line) in lines[start..end].iter().enumerate() {
                    let line_num = start + i + 1;
                    let marker = if line_num == hit.symbol.location.line { ">" } else { " " };
                    println!("{}{:4} | {}", marker, line_num, line);
                }
                if end < lines.len() {
                    println!("      ... ({} more lines)", lines.len() - end);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
        println!();
    }
}
```

**File: `src/main.rs`**

Update call to pass `full` parameter.

---

## Improvement 3: Stronger Definition Boosting

### Problem
Current 1.1x boost for definitions is too weak. Docs still rank above definitions.

### Changes

**File: `src/indexer/symbols.rs`** (line ~163)

Replace `apply_find_definition_boost`:
```rust
fn apply_find_definition_boost(&self, base_boost: f32) -> f32 {
    match self {
        // Type definitions - strong boost (was 1.1x)
        SymbolKind::Struct => base_boost * 2.0,
        SymbolKind::Enum => base_boost * 2.0,
        SymbolKind::Trait => base_boost * 2.0,
        SymbolKind::TypeAlias => base_boost * 1.8,

        // Function definitions - moderate boost
        SymbolKind::Function => base_boost * 1.5,
        SymbolKind::Method => base_boost * 1.3,

        // Impl blocks are usages - reduce
        SymbolKind::Impl => base_boost * 0.5,

        // Documentation - heavily reduce
        SymbolKind::DocumentChunk => base_boost * 0.2,

        _ => base_boost,
    }
}
```

---

## Implementation Order

1. **Improvement 3** (5 min) - One function change
2. **Improvement 1** (30 min) - Logic changes only
3. **Improvement 2** (30 min) - CLI signature changes

---

## Files Changed

| File | Changes |
|------|---------|
| `src/indexer/symbols.rs` | Increase definition boost 1.1x → 2.0x |
| `src/cli/mod.rs` | Add `--full` flag |
| `src/cli/commands.rs` | Add intent detection + full display |
| `src/main.rs` | Pass `full` parameter |

---

## Expected Results

| Query | Before | After |
|-------|--------|-------|
| "BgeEmbedder struct" | 10 doc files | struct #1 |
| "how does pipeline work" | doc headers | source #1-3 |
| "RetrievalError definition" | docs #1-4 | enum #1 |
| "BgeEmbedder" --full | N/A | full content |

---

## Testing

```bash
# Unit tests
cargo test --lib

# Manual tests
rustean search "BgeEmbedder struct"           # struct at #1
rustean search "how does pipeline work"        # source code
rustean search "RetrievalError definition"     # enum at #1
rustean search "BgeEmbedder" --full            # full content
```

---
---

# Previous Plan: Relevance-Based Filtering (Option B)

**Date:** 2026-02-04
**Status:** ✅ Completed
**Previous:** Structured Pipeline (Completed)
**Estimated Effort:** 4-6 hours

---

## Problem

Current structured pipeline returns **fixed limits** (10 code + 5 doc + 3 notes) regardless of relevance, resulting in noise when queries don't have enough relevant results.

## Solution: Min/Max with Threshold

Return only relevant results while guaranteeing minimum coverage:

```
Per content type:
- Minimum: 1 result (if any exist in index)
- Maximum: current limits (10 code, 5 doc, 3 notes)
- Filter: RRF score > threshold
```

---

## Research Summary

### Sources

1. [Elasticsearch Weighted RRF](https://www.elastic.co/search-labs/blog/weighted-reciprocal-rank-fusion-rrf) - Weighted RRF best practices
2. [Azure Hybrid Search Scoring](https://learn.microsoft.com/en-us/azure/search/hybrid-search-ranking) - RRF k parameter tuning
3. [OpenSearch RRF](https://opensearch.org/blog/introducing-reciprocal-rank-fusion-hybrid-search/) - RRF fundamentals
4. [Milvus RRF Ranker](https://milvus.io/docs/rrf-ranker.md) - RRF implementation patterns
5. [BM25 Thresholds](https://www.paradedb.com/learn/search-concepts/bm25) - Score distribution considerations

### Key Findings

| Finding | Source | Implication |
|---------|--------|-------------|
| RRF formula: `1/(k+rank)` with k≈60 | Elasticsearch, Azure | Rank 1 → 0.0164, Rank 50 → 0.0091 |
| Scores are **relative**, not absolute | ParadeDB | Must calibrate threshold per dataset |
| Documents in both lists get boosted | OpenSearch | Dual-hit score ≈ 0.033 (rank 1+1) |
| BM25 threshold ~1.5 for filtering | Research | Can use as pre-filter before RRF |
| Weighted RRF allows signal prioritization | Elasticsearch | Can weight code vs doc vs notes |

### RRF Score Distribution (k=60)

| Scenario | RRF Score |
|----------|-----------|
| Rank 1 only (one list) | 0.0164 |
| Rank 1 + Rank 1 (both lists) | 0.0328 |
| Rank 10 only | 0.0143 |
| Rank 10 + Rank 10 | 0.0286 |
| Rank 50 only | 0.0091 |
| Rank 50 + Rank 50 | 0.0182 |

**Recommended threshold: 0.015** (filters out rank >40 single-list hits, keeps dual-list hits)

---

## Architecture

```
Query → Query Expansion → Parallel Hybrid Search
                              ↓
         ┌──────────────────────────────────────┐
         │         Parallel Execution           │
         ├──────────┬──────────┬───────────────┤
         │   Code   │   Doc    │    Notes      │
         │ Pipeline │ Pipeline │   Pipeline    │
         │          │          │               │
         │ RRF Fuse │ RRF Fuse │   RRF Fuse    │
         │    ↓     │    ↓     │      ↓        │
         │ FILTER   │ FILTER   │   FILTER      │
         │ score>θ  │ score>θ  │   score>θ     │
         │ min=1    │ min=1    │   min=1       │
         │ max=10   │ max=5    │   max=3       │
         └──────────┴──────────┴───────────────┘
                              ↓
              StructuredOutput (only relevant results)
```

---

## Implementation Tasks

### Task 1: Add Relevance Config to HybridSearchConfig
**File:** `src/retrieval/hybrid/mod.rs`
**Effort:** 30 min

```rust
/// Configuration for hybrid search
#[derive(Debug, Clone)]
pub struct HybridSearchConfig {
    // ... existing fields ...

    /// Minimum RRF score threshold (default: 0.015)
    /// Results below this are filtered (unless min not met)
    pub rrf_score_threshold: f32,

    /// Minimum results per content type (default: 1)
    /// Guarantees at least N results if available
    pub min_results_per_type: usize,
}

impl Default for HybridSearchConfig {
    fn default() -> Self {
        Self {
            keyword_weight: 1.0,
            semantic_weight: 1.0,
            candidates_per_source: 50,
            rrf_k: 60.0,
            rrf_score_threshold: 0.015,  // NEW
            min_results_per_type: 1,      // NEW
        }
    }
}
```

---

### Task 2: Add filter_by_relevance to TripleHybridSearch
**File:** `src/retrieval/hybrid/triple.rs`
**Effort:** 1 hour

```rust
/// Filter results by relevance score with min/max guarantees
fn filter_by_relevance(
    &self,
    results: Vec<HybridSearchResult>,
    max_limit: usize,
) -> Vec<HybridSearchResult> {
    let threshold = self.config.rrf_score_threshold;
    let min_results = self.config.min_results_per_type;

    // Results are already sorted by RRF score (descending)
    let mut filtered: Vec<HybridSearchResult> = Vec::new();

    for (i, result) in results.into_iter().enumerate() {
        // Always include if we haven't met minimum
        if filtered.len() < min_results {
            filtered.push(result);
            continue;
        }

        // Stop if we hit max limit
        if filtered.len() >= max_limit {
            break;
        }

        // Include only if above threshold
        if result.rrf_score >= threshold {
            filtered.push(result);
        }
    }

    filtered
}
```

Update `fuse_pipeline_results`:

```rust
fn fuse_pipeline_results(
    &self,
    keyword_hits: &[SearchHit],
    semantic_results: &[VectorSearchResult],
    limit: usize,
) -> Vec<HybridSearchResult> {
    // ... existing RRF fusion logic ...

    // Sort by RRF score
    let mut results: Vec<HybridSearchResult> = scores.into_values().collect();
    results.sort_by(|a, b| b.rrf_score.partial_cmp(&a.rrf_score).unwrap());

    // NEW: Apply relevance filtering instead of simple truncate
    self.filter_by_relevance(results, limit)
}
```

---

### Task 3: Add threshold to PipelineConfig
**File:** `src/retrieval/agent/pipeline.rs`
**Effort:** 30 min

```rust
pub struct PipelineConfig {
    // ... existing fields ...

    /// RRF score threshold for filtering (default: 0.015)
    pub rrf_score_threshold: f32,

    /// Minimum results per content type (default: 1)
    pub min_results_per_type: usize,
}
```

---

### Task 4: Add CLI flag for threshold
**File:** `src/cli/mod.rs`
**Effort:** 30 min

```rust
#[derive(Parser)]
pub struct RetrieveArgs {
    // ... existing args ...

    /// RRF score threshold for relevance filtering (default: 0.015)
    #[arg(long, default_value = "0.015")]
    pub threshold: f32,

    /// Minimum results per content type (default: 1)
    #[arg(long, default_value = "1")]
    pub min_results: usize,
}
```

---

### Task 5: Add debug output showing filtered count
**File:** `src/retrieval/hybrid/triple.rs`
**Effort:** 30 min

```rust
fn filter_by_relevance(
    &self,
    results: Vec<HybridSearchResult>,
    max_limit: usize,
    content_type: &str,  // for logging
) -> Vec<HybridSearchResult> {
    let total = results.len();
    let filtered = /* ... filtering logic ... */;

    let below_threshold = total - filtered.len();
    if below_threshold > 0 {
        eprintln!(
            "[pipeline] {}: {} results, {} filtered (score < {})",
            content_type,
            filtered.len(),
            below_threshold,
            self.config.rrf_score_threshold
        );
    }

    filtered
}
```

---

### Task 6: Unit tests
**File:** `src/retrieval/hybrid/triple.rs`
**Effort:** 1 hour

```rust
#[cfg(test)]
mod filter_tests {
    use super::*;

    /// Test: Results above threshold are kept
    #[test]
    fn test_filter_keeps_high_scores() {
        let results = vec![
            mock_result(0.025), // above threshold
            mock_result(0.020), // above threshold
            mock_result(0.010), // below threshold
        ];

        let config = HybridSearchConfig {
            rrf_score_threshold: 0.015,
            min_results_per_type: 1,
            ..Default::default()
        };

        let filtered = filter_by_relevance(&results, 10, &config);
        assert_eq!(filtered.len(), 2); // only high scores kept
    }

    /// Test: Minimum guarantee honored even with low scores
    #[test]
    fn test_filter_respects_minimum() {
        let results = vec![
            mock_result(0.010), // below threshold
            mock_result(0.005), // below threshold
        ];

        let config = HybridSearchConfig {
            rrf_score_threshold: 0.015,
            min_results_per_type: 1,
            ..Default::default()
        };

        let filtered = filter_by_relevance(&results, 10, &config);
        assert_eq!(filtered.len(), 1); // min=1 honored
    }

    /// Test: Max limit still enforced
    #[test]
    fn test_filter_respects_maximum() {
        let results = vec![
            mock_result(0.030),
            mock_result(0.025),
            mock_result(0.020),
        ];

        let config = HybridSearchConfig {
            rrf_score_threshold: 0.015,
            min_results_per_type: 1,
            ..Default::default()
        };

        let filtered = filter_by_relevance(&results, 2, &config);
        assert_eq!(filtered.len(), 2); // max=2 enforced
    }

    /// Test: Empty input returns empty
    #[test]
    fn test_filter_empty_input() {
        let results: Vec<HybridSearchResult> = vec![];
        let config = HybridSearchConfig::default();

        let filtered = filter_by_relevance(&results, 10, &config);
        assert!(filtered.is_empty());
    }
}
```

---

## Configuration Recommendations

| Parameter | Default | Tuning Guidance |
|-----------|---------|-----------------|
| `rrf_score_threshold` | 0.015 | Lower (0.01) for more results, higher (0.02) for stricter |
| `min_results_per_type` | 1 | Set to 0 to allow empty sections |
| `max_code_results` | 10 | Unchanged |
| `max_doc_results` | 5 | Unchanged |
| `max_notes_results` | 3 | Unchanged |

---

## Expected Outcomes

### Before (Fixed Limits)
```
Query: "HybridSearch struct"
Code: 10 results (many irrelevant)
Docs: 5 results (some irrelevant)
Notes: 3 results (2 irrelevant)
```

### After (Relevance Filtering)
```
Query: "HybridSearch struct"
Code: 3 results (all relevant, score > 0.015)
Docs: 2 results (both relevant)
Notes: 1 result (minimum guarantee)
```

---

## File Summary

| File | Action | Effort |
|------|--------|--------|
| `src/retrieval/hybrid/mod.rs` | Add threshold config fields | 30 min |
| `src/retrieval/hybrid/triple.rs` | Add filter_by_relevance | 1 hour |
| `src/retrieval/agent/pipeline.rs` | Add threshold to PipelineConfig | 30 min |
| `src/cli/mod.rs` | Add CLI flags | 30 min |
| `src/cli/commands.rs` | Wire up new flags | 30 min |
| Tests | Unit tests for filtering | 1 hour |
| **Total** | | **~4 hours** |

---

## Success Criteria

1. ✅ Results with RRF score < threshold are filtered out
2. ✅ Minimum 1 result per type (if available) is guaranteed
3. ✅ Maximum limits are still enforced
4. ✅ Debug output shows filtering stats
5. ✅ CLI flags allow tuning threshold
6. ✅ All existing tests pass (245 tests)
7. ✅ New filtering tests pass (5 tests)

## Final Test Results (2026-02-04)

| Query | Results | Filtered |
|-------|---------|----------|
| "how does daemon work" | 6 code, 5 doc, 3 notes | 12 code + 7 doc |
| "HybridSearch struct" | 5 code, 5 doc, 2 notes | 21 doc |
| "query expansion documentation" | 8 code, 5 doc, 3 notes | 18 code + 35 doc + 18 notes |

**Noise reduction: 12-71 low-relevance results filtered per query**

---

## Future Enhancements (Not in scope)

- Adaptive threshold based on score distribution
- Per-content-type thresholds
- Confidence levels in output (high/medium/low)
- Weighted content types (boost code over notes)

---
---

# Previous Plan: Structured Pipeline (Completed)

**Date:** 2026-02-04
**Status:** ✅ Completed
**Estimated Effort:** 2-3 days

---

## User Decisions

| Question | Decision |
|----------|----------|
| Default limits | 10 code + 5 docs + 3 notes |
| Reranking | **No** - start clean without reranking |
| Doc types | **3 pipelines** - Code, Doc, Notes (separate) |
| Full file content | **Yes** - like Augment MCP |

---

## Summary

Split the unified retrieval pipeline into **three parallel pipelines**:
1. **Code Pipeline** - SourceCode + Test files (`.rs`, `.py`, `.js`, etc.)
2. **Doc Pipeline** - Documentation files (`doc/*.md`)
3. **Notes Pipeline** - Notes + Benchmarks (`notes/*.md`, `notes/benchmarks/*.md`)

Results merged into structured output with **full file content** for LLM consumption.

---

## Proposed Architecture

```
                           User Query
                               │
            ┌──────────────────┼──────────────────┐
            │                  │                  │
            ▼                  ▼                  ▼
   ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
   │  CODE PIPELINE  │ │  DOC PIPELINE   │ │ NOTES PIPELINE  │
   │  (parallel)     │ │  (parallel)     │ │  (parallel)     │
   ├─────────────────┤ ├─────────────────┤ ├─────────────────┤
   │ CodeSearchIndex │ │ DocSearchIndex  │ │ NotesSearchIndex│
   │ CodeVectorStore │ │ DocVectorStore  │ │ NotesVectorStore│
   │ RRF Fusion      │ │ RRF Fusion      │ │ RRF Fusion      │
   │ Top 10 results  │ │ Top 5 results   │ │ Top 3 results   │
   └────────┬────────┘ └────────┬────────┘ └────────┬────────┘
            │                   │                   │
            └───────────────────┼───────────────────┘
                                │
                       ┌────────▼────────┐
                       │   STRUCTURED    │
                       │   OUTPUT        │
                       │ {               │
                       │   code: [...],  │  ← Full file content
                       │   docs: [...],  │
                       │   notes: [...]  │
                       │ }               │
                       └─────────────────┘
```

---

## Content Type Classification

| ContentType | DocumentType Sources | Files |
|-------------|---------------------|-------|
| **Code** | SourceCode, Test | `src/**/*.rs`, `tests/**/*.rs` |
| **Doc** | Documentation | `doc/**/*.md` |
| **Notes** | Notes, Benchmark | `notes/**/*.md`, `notes/benchmarks/**/*.md` |

---

## Implementation Steps

### Phase 1: Create Triple Index Infrastructure

#### Task 1.1: Create ContentType Enum
**File:** `src/indexer/symbols.rs`
**Effort:** 30 min

```rust
/// High-level content classification for separate indexing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentType {
    /// Code files (SourceCode + Test)
    Code,
    /// Documentation files (doc/*.md)
    Doc,
    /// Notes and benchmarks (notes/*.md)
    Notes,
}

impl ContentType {
    /// Classify from DocumentType
    pub fn from_document_type(doc_type: &DocumentType) -> Self {
        match doc_type {
            DocumentType::SourceCode | DocumentType::Test => Self::Code,
            DocumentType::Documentation => Self::Doc,
            DocumentType::Notes | DocumentType::Benchmark => Self::Notes,
        }
    }

    /// Classify from file path
    pub fn from_path(path: &Path) -> Self {
        Self::from_document_type(&DocumentType::from_path(path))
    }

    /// Get all variants for iteration
    pub fn all() -> [Self; 3] {
        [Self::Code, Self::Doc, Self::Notes]
    }
}
```

**Test:** Unit test for classification logic

---

#### Task 1.2: Create TripleSearchIndex
**File:** `src/indexer/triple_search.rs` (new file)
**Effort:** 2.5 hours

```rust
/// Triple search index with separate code, doc, and notes indexes
pub struct TripleSearchIndex {
    /// Index for code symbols (SourceCode + Test)
    code_index: SearchIndex,
    /// Index for documentation (doc/*.md)
    doc_index: SearchIndex,
    /// Index for notes (notes/*.md)
    notes_index: SearchIndex,
}

impl TripleSearchIndex {
    /// Create new triple index (in-memory)
    pub fn in_memory() -> SearchResult<Self>;

    /// Create persistent triple index
    pub fn open_or_create(base_path: &Path) -> SearchResult<Self>;

    /// Index symbols, routing to appropriate index
    pub fn index_symbols(&self, symbols: &[Symbol]) -> SearchResult<TripleIndexStats>;

    /// Search code index only
    pub fn search_code(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>>;

    /// Search doc index only
    pub fn search_docs(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>>;

    /// Search notes index only
    pub fn search_notes(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>>;

    /// Parallel search all three indexes
    pub fn search_parallel(
        &self,
        query: &str,
        code_limit: usize,
        doc_limit: usize,
        notes_limit: usize,
    ) -> SearchResult<TripleSearchResults>;
}

/// Stats from triple indexing
pub struct TripleIndexStats {
    pub code_count: usize,
    pub doc_count: usize,
    pub notes_count: usize,
}

/// Results from triple search
pub struct TripleSearchResults {
    pub code_results: Vec<SearchHit>,
    pub doc_results: Vec<SearchHit>,
    pub notes_results: Vec<SearchHit>,
}
```

**Storage structure:**
```
.rustean-index/
├── code/
│   └── tantivy/     # Code keyword index
├── docs/
│   └── tantivy/     # Doc keyword index
├── notes/
│   └── tantivy/     # Notes keyword index
└── ...
```

---

#### Task 1.3: Create TripleVectorStore
**File:** `src/retrieval/hybrid/triple_vector_store.rs` (new file)
**Effort:** 2.5 hours

```rust
/// Triple vector store with separate code, doc, and notes stores
pub struct TripleVectorStore {
    /// Vector store for code embeddings
    code_store: VectorStore,
    /// Vector store for doc embeddings
    doc_store: VectorStore,
    /// Vector store for notes embeddings
    notes_store: VectorStore,
}

impl TripleVectorStore {
    /// Create new triple store
    pub fn new() -> Self;

    /// Create with persistence
    pub fn with_path(base_path: &Path) -> RetrievalResult<Self>;

    /// Insert point, routing to appropriate store
    pub fn insert(&mut self, point: VectorPoint, content_type: ContentType);

    /// Build all three HNSW indexes
    pub fn build_indexes(&mut self) -> RetrievalResult<()>;

    /// Search code vectors only
    pub fn search_code(&self, query: &[f32], k: usize) -> Vec<SearchResult>;

    /// Search doc vectors only
    pub fn search_docs(&self, query: &[f32], k: usize) -> Vec<SearchResult>;

    /// Search notes vectors only
    pub fn search_notes(&self, query: &[f32], k: usize) -> Vec<SearchResult>;

    /// Parallel search all three stores
    pub fn search_parallel(
        &self,
        query: &[f32],
        code_k: usize,
        doc_k: usize,
        notes_k: usize,
    ) -> TripleVectorResults;
}

/// Results from triple vector search
pub struct TripleVectorResults {
    pub code_results: Vec<SearchResult>,
    pub doc_results: Vec<SearchResult>,
    pub notes_results: Vec<SearchResult>,
}
```

**Storage structure:**
```
.rustean-index/
├── code/
│   └── vectors.json   # Code embeddings
├── docs/
│   └── vectors.json   # Doc embeddings
├── notes/
│   └── vectors.json   # Notes embeddings
└── ...
```

---

### Phase 2: Create Triple Hybrid Search

#### Task 2.1: Create TripleHybridSearch
**File:** `src/retrieval/hybrid/triple.rs` (new file)
**Effort:** 3 hours

```rust
/// Triple hybrid search with separate code, doc, and notes pipelines
pub struct TripleHybridSearch {
    /// Triple keyword index
    keyword_index: TripleSearchIndex,
    /// Triple vector store
    vector_store: TripleVectorStore,
    /// Daemon client for embeddings
    daemon_client: DaemonClient,
    /// Configuration
    config: HybridConfig,
}

impl TripleHybridSearch {
    /// Index symbols into all three pipelines
    pub fn index_symbols(&mut self, symbols: &[Symbol]) -> RetrievalResult<TripleIndexStats>;

    /// Parallel hybrid search (no reranking)
    pub fn search(
        &self,
        query: &str,
        code_limit: usize,
        doc_limit: usize,
        notes_limit: usize,
    ) -> RetrievalResult<TripleHybridResults>;

    /// Search with SearchSpec
    pub fn search_with_spec(
        &self,
        spec: &SearchSpec,
        code_limit: usize,
        doc_limit: usize,
        notes_limit: usize,
    ) -> RetrievalResult<TripleHybridResults>;
}

/// Results from triple hybrid search
pub struct TripleHybridResults {
    pub code_results: Vec<HybridSearchResult>,
    pub doc_results: Vec<HybridSearchResult>,
    pub notes_results: Vec<HybridSearchResult>,
}
```

**Key implementation - parallel search with rayon:**
```rust
pub fn search(
    &self,
    query: &str,
    code_limit: usize,
    doc_limit: usize,
    notes_limit: usize,
) -> RetrievalResult<TripleHybridResults> {
    // Get query embedding once (shared)
    let query_embedding = self.daemon_client.embed(vec![query.to_string()])?
        .into_iter().next()
        .ok_or_else(|| RetrievalError::Embedding("no embedding".to_string()))?;

    // Parallel search all three pipelines
    let ((code_results, doc_results), notes_results) = rayon::join(
        || rayon::join(
            || self.search_code_internal(query, &query_embedding, code_limit),
            || self.search_docs_internal(query, &query_embedding, doc_limit),
        ),
        || self.search_notes_internal(query, &query_embedding, notes_limit),
    );

    Ok(TripleHybridResults {
        code_results: code_results?,
        doc_results: doc_results?,
        notes_results: notes_results?,
    })
}
```

---

### Phase 3: Update Pipeline (No Reranking)

#### Task 3.1: Update RetrievalPipeline
**File:** `src/retrieval/agent/pipeline.rs`
**Effort:** 2 hours

```rust
/// Output from triple retrieval
pub struct TripleRetrievalOutput {
    /// Original query
    pub query: String,
    /// Search specification used
    pub search_spec: SearchSpec,
    /// Code search results (guaranteed top-10)
    pub code_results: Vec<HybridSearchResult>,
    /// Doc search results (guaranteed top-5)
    pub doc_results: Vec<HybridSearchResult>,
    /// Notes search results (guaranteed top-3)
    pub notes_results: Vec<HybridSearchResult>,
    /// Formatted output (JSON or XML)
    pub formatted_output: String,
    /// Token count estimate
    pub token_count: usize,
}
```

**Updated retrieve() flow (no reranking):**
```rust
pub fn retrieve(&mut self, query: &str) -> RetrievalResult<TripleRetrievalOutput> {
    // 1. Query expansion
    let search_spec = self.expand_query(query)?;

    // 2. Triple hybrid search (parallel, no rerank)
    let results = self.triple_hybrid.search_with_spec(
        &search_spec,
        self.config.max_code_results,   // 10
        self.config.max_doc_results,    // 5
        self.config.max_notes_results,  // 3
    )?;

    // 3. Read full file content for code results
    let code_with_content = self.enrich_with_full_content(&results.code_results)?;

    // 4. Format structured output
    let formatted_output = self.format_triple_output(
        &code_with_content,
        &results.doc_results,
        &results.notes_results,
    )?;

    Ok(TripleRetrievalOutput {
        query: query.to_string(),
        search_spec,
        code_results: code_with_content,
        doc_results: results.doc_results,
        notes_results: results.notes_results,
        formatted_output,
        token_count: formatted_output.len() / 4,
    })
}
```

---

#### Task 3.2: Update PipelineConfig
**File:** `src/retrieval/agent/pipeline.rs`
**Effort:** 30 min

```rust
pub struct PipelineConfig {
    // ... existing fields ...

    /// Max code results (default: 10)
    pub max_code_results: usize,
    /// Max doc results (default: 5)
    pub max_doc_results: usize,
    /// Max notes results (default: 3)
    pub max_notes_results: usize,
    /// Enable parallel search (default: true)
    pub parallel_search: bool,
    /// Include full file content (default: true, like Augment)
    pub full_file_content: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            max_code_results: 10,
            max_doc_results: 5,
            max_notes_results: 3,
            parallel_search: true,
            full_file_content: true,
            // Disable reranking
            rerank: false,
            // ... other existing defaults ...
        }
    }
}
```

---

### Phase 4: Full File Content (Like Augment)

#### Task 4.1: Add Full File Reader
**File:** `src/retrieval/agent/pipeline.rs`
**Effort:** 1.5 hours

```rust
/// Enrich search results with full file content
fn enrich_with_full_content(
    &self,
    results: &[HybridSearchResult],
) -> RetrievalResult<Vec<EnrichedResult>> {
    let mut enriched = Vec::new();

    for result in results {
        let file_path = &result.symbol.location.file;

        // Read full file content
        let content = std::fs::read_to_string(file_path)
            .unwrap_or_else(|_| "[File not readable]".to_string());

        // Limit to reasonable size (500 lines max like Augment)
        let lines: Vec<&str> = content.lines().take(500).collect();
        let truncated_content = lines.join("\n");

        enriched.push(EnrichedResult {
            symbol: result.symbol.clone(),
            rrf_score: result.rrf_score,
            full_content: truncated_content,
            line_count: content.lines().count(),
            was_truncated: content.lines().count() > 500,
        });
    }

    Ok(enriched)
}

/// Search result enriched with full file content
pub struct EnrichedResult {
    pub symbol: Symbol,
    pub rrf_score: f32,
    pub full_content: String,
    pub line_count: usize,
    pub was_truncated: bool,
}
```

---

#### Task 4.2: Create Structured JSON Output
**File:** `src/retrieval/agent/output.rs` (new file)
**Effort:** 1 hour

```rust
/// Structured output for LLM consumption
#[derive(Debug, Serialize, Deserialize)]
pub struct StructuredOutput {
    pub query: String,
    pub intent: String,
    pub code_context: Vec<CodeResult>,
    pub doc_context: Vec<DocResult>,
    pub notes_context: Vec<NotesResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeResult {
    pub file: String,
    pub symbol: String,
    pub kind: String,
    pub line: usize,
    pub signature: Option<String>,
    pub full_content: String,  // Full file content (up to 500 lines)
    pub line_count: usize,
    pub truncated: bool,
    pub relevance_score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocResult {
    pub file: String,
    pub section: String,
    pub content: String,
    pub relevance_score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotesResult {
    pub file: String,
    pub section: String,
    pub content: String,
    pub relevance_score: f32,
}
```

**Example JSON output:**
```json
{
  "query": "how does the daemon retry logic work",
  "intent": "Understand",
  "code_context": [
    {
      "file": "src/retrieval/daemon/client.rs",
      "symbol": "DaemonClient",
      "kind": "Struct",
      "line": 70,
      "full_content": "//! Daemon Client...\n\nuse std::...\n\npub struct RetryConfig {...}\n\npub struct DaemonClient {...}\n\nimpl DaemonClient {...}",
      "line_count": 407,
      "truncated": false,
      "relevance_score": 0.92
    }
  ],
  "doc_context": [
    {
      "file": "doc/daemon.md",
      "section": "Retry Logic",
      "content": "## Retry Logic\n\nThe daemon client implements exponential backoff...",
      "relevance_score": 0.78
    }
  ],
  "notes_context": [
    {
      "file": "notes/implementation/daemon-retry.md",
      "section": "Implementation Notes",
      "content": "Added retry logic on 2026-02-01...",
      "relevance_score": 0.65
    }
  ]
}
```

---

### Phase 5: Update CLI & MCP

#### Task 5.1: Update CLI Output
**File:** `src/commands/search.rs`
**Effort:** 1 hour

```rust
fn display_triple_results(output: &TripleRetrievalOutput, format: OutputFormat) {
    match format {
        OutputFormat::Text => {
            println!("\n=== Code Results ({}) ===\n", output.code_results.len());
            for result in &output.code_results {
                println!("📄 {} ({}:{})",
                    result.symbol.name,
                    result.symbol.location.file.display(),
                    result.symbol.location.line
                );
                println!("   {} lines{}\n",
                    result.line_count,
                    if result.was_truncated { " (truncated)" } else { "" }
                );
            }

            println!("\n=== Documentation ({}) ===\n", output.doc_results.len());
            for result in &output.doc_results {
                println!("📖 {} ({})", result.symbol.name, result.symbol.location.file.display());
            }

            println!("\n=== Notes ({}) ===\n", output.notes_results.len());
            for result in &output.notes_results {
                println!("📝 {} ({})", result.symbol.name, result.symbol.location.file.display());
            }
        }
        OutputFormat::Json => {
            let structured = StructuredOutput::from(output);
            println!("{}", serde_json::to_string_pretty(&structured).unwrap());
        }
    }
}
```

---

### Phase 6: Migration

#### Task 6.1: Update Index Version
**File:** `src/indexer/state.rs`
**Effort:** 30 min

```rust
pub const INDEX_VERSION: u32 = 3;  // was 2, forces re-index
```

---

## File Summary

| File | Action | Effort |
|------|--------|--------|
| `src/indexer/symbols.rs` | Add `ContentType` enum (3 variants) | 30 min |
| `src/indexer/triple_search.rs` | **New** - TripleSearchIndex | 2.5 hours |
| `src/indexer/mod.rs` | Export new module | 5 min |
| `src/retrieval/hybrid/triple_vector_store.rs` | **New** - TripleVectorStore | 2.5 hours |
| `src/retrieval/hybrid/triple.rs` | **New** - TripleHybridSearch | 3 hours |
| `src/retrieval/hybrid/mod.rs` | Export new modules | 5 min |
| `src/retrieval/agent/pipeline.rs` | Update for triple search + full content | 2.5 hours |
| `src/retrieval/agent/output.rs` | **New** - StructuredOutput | 1 hour |
| `src/retrieval/agent/mod.rs` | Export new module | 5 min |
| `src/commands/search.rs` | Update CLI display | 1 hour |
| `src/indexer/state.rs` | Bump version | 30 min |
| **Tests** | Unit + integration tests | 2 hours |
| **Total** | | **~16 hours** |

---

## Success Criteria

1. ✅ Code results never pushed out by doc/notes results
2. ✅ Doc results never pushed out by code/notes results
3. ✅ Notes results never pushed out by code/doc results
4. ✅ Full file content returned (like Augment MCP)
5. ✅ Parallel search is faster than sequential
6. ✅ No reranking (clean start)
7. ✅ Structured JSON output for LLM consumption

---

## Storage Structure

```
.rustean-index/
├── state.json          # Index metadata (version 3)
├── code/
│   ├── tantivy/        # Code keyword index
│   └── vectors.json    # Code embeddings
├── docs/
│   ├── tantivy/        # Doc keyword index
│   └── vectors.json    # Doc embeddings
├── notes/
│   ├── tantivy/        # Notes keyword index
│   └── vectors.json    # Notes embeddings
├── graph.json          # Semantic graph (unchanged)
└── refs.json           # References (unchanged)
```
