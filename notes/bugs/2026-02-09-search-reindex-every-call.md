# Fix: Search Re-indexes Entire Codebase on Every Call

**Date**: 2026-02-09
**Status**: Fixed
**Severity**: High (30s+ penalty on every search)

---

## Problem

Every `rustean search` call (keyword, semantic, navigation) re-indexed the entire codebase from scratch:

1. **Tree-sitter re-parsed every file** (~2s)
2. **Semantic search re-embedded every symbol** via daemon (~30s+ on CPU)
3. **Tantivy keyword index rebuilt** in-memory each time

The `retrieve` command already did this correctly using `.with_persistence()` and `HybridSearch::with_paths()`. The search service simply didn't use these patterns.

## Root Cause

3 files created `IndexManager` and `HybridSearch` without persistence:

| File | Problem |
|------|---------|
| `search_impl.rs` | `IndexManager::new()` without `.with_persistence()` |
| `search_impl.rs` | `SearchIndex::in_memory()` ignoring persisted Tantivy |
| `semantic_impl.rs` | `HybridSearch::new()` — in-memory, no vector cache |
| `navigation_impl.rs` | `IndexManager::new()` without `.with_persistence()` |

## Fix

### 1. `src/service/search/search_impl.rs`

- Added `.with_persistence()` to `IndexManager` in `execute_search()`
- `run_keyword_search()` now uses `result.search_index` (persisted Tantivy) when available, with in-memory fallback

```rust
// Before
let manager = IndexManager::new();
let index = SearchIndex::in_memory()?;

// After
let manager = IndexManager::new().with_persistence();
let index = match result.search_index {
    Some(ref idx) => idx,
    None => { /* in-memory fallback */ }
};
```

### 2. `src/service/search/semantic_impl.rs`

- Replaced `create_hybrid()` + `index_hybrid()` with `load_or_create_hybrid()`
- Uses `HybridSearch::with_paths()` to load cached vectors from `.rustean-index/`
- Skips re-embedding if vector store already populated (cache hit)
- Persists vectors on first build for subsequent runs

```rust
// Before
let mut hybrid = HybridSearch::new()?;
hybrid.index_symbols(&symbols)?;

// After
let hybrid = HybridSearch::with_paths(&tantivy, &vectors)?;
if !hybrid.vector_store().is_empty() {
    return Ok(hybrid); // cache hit
}
hybrid.index_symbols(&symbols)?;
hybrid.persist();
```

### 3. `src/service/search/navigation_impl.rs`

- Added `.with_persistence()` to `IndexManager` in `build_semantic_graph()`

```rust
// Before
let manager = IndexManager::new()
    .with_semantic_analysis();

// After
let manager = IndexManager::new()
    .with_persistence()
    .with_semantic_analysis();
```

## Performance Impact

| Scenario | Before | After |
|----------|--------|-------|
| Keyword search (no changes) | ~2s | ~50ms |
| Semantic search (cached) | ~30s+ | ~1-2s |
| Navigation (no changes) | ~2s | ~50ms |
| First run (no cache) | Same | Same (+ persists) |
| After touching 1 file | Full reindex | 1 file re-parsed |

## Verification

- `cargo build` — clean, zero warnings
- `cargo test` — 637 passed, 0 failed
- Cache files stored in `.rustean-index/` (tantivy/, vectors.json, state.json)

## Reference Implementation

The `retrieve` command pipeline already does this correctly:
- `src/retrieval/agent/pipeline/initialization.rs` — uses `.with_persistence()`
- `src/retrieval/agent/pipeline/hybrid_init.rs` — uses `HybridSearch::with_paths()`
