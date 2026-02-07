# Persistent Index Caching - Implementation Progress

## Date: 2026-02-01

## Objective

Reduce ch-cli retrieval time from ~4.5s to <500ms by implementing persistent index caching.

## Implementation Status

### Phase 1: Enable Persistence in RetrievalPipeline [COMPLETE]

**Files Modified:**
- `src/retrieval/agent/pipeline.rs`

**Changes:**
1. Added `enable_persistence: bool` field to `PipelineConfig` (default: `true`)
2. Modified `initialize()` to use `IndexManager.with_persistence()`
3. Added `initialize_hybrid_search()` helper method that:
   - Tries to load cached Tantivy/vector indices from `.ch-index/`
   - Falls back to building fresh indices if cache doesn't exist
   - Persists newly built indices for future warm starts

### Phase 2: Daemon Protocol Messages [COMPLETE]

**Files Modified:**
- `src/retrieval/daemon/protocol.rs`

**New Types Added:**
- `CachedSearchResult` - Serializable search result for IPC
- `DaemonRequest::IndexProject` - Index a project and cache in daemon
- `DaemonRequest::SearchProject` - Search a cached project
- `DaemonRequest::ProjectStatus` - Check if project is cached
- `DaemonRequest::EvictProject` - Remove project from cache
- `DaemonResponse::ProjectIndexed` - Index completion info
- `DaemonResponse::SearchResults` - Cached search results
- `DaemonResponse::ProjectCacheStatus` - Cache status info

### Phase 3: Daemon Project Cache [COMPLETE]

**Files Modified:**
- `src/retrieval/daemon/server.rs`

**Changes:**
1. Added `CachedProject` struct (symbols, graph, hybrid, last_indexed)
2. Added `project_cache: HashMap<PathBuf, CachedProject>` to `ModelDaemon`
3. Implemented handler methods:
   - `handle_index_project()` - Index and cache a project
   - `handle_search_project()` - Search cached project
   - `handle_project_status()` - Check cache status
   - `handle_evict_project()` - Remove from cache
   - `evict_lru_if_needed()` - LRU eviction (max 5 projects)

### Phase 4: Client Methods [COMPLETE]

**Files Modified:**
- `src/retrieval/daemon/client.rs`

**New Methods:**
- `index_project(path, force)` - Index a project via daemon
- `search_project(path, query, limit)` - Search cached project
- `project_status(path)` - Get cache status
- `evict_project(path)` - Evict from cache

### Phase 5: CLI Integration [COMPLETE]

**Files Modified:**
- `src/cli/commands.rs`

**Changes:**
- `retrieve_command()` now uses `enable_persistence: true` by default
- Reduced output noise when `--xml` flag is set

### Phase 6: Unit Tests [COMPLETE]

**Tests Added:**
- `test_config_persistence_enabled_by_default`
- `test_config_with_persistence_disabled`
- `test_index_project_request_serialization`
- `test_project_indexed_response_serialization`
- `test_cached_search_result_serialization`
- `test_project_cache_status_serialization`

**Test Results:**
```
test result: ok. 86 passed; 0 failed; 5 ignored
```

## Actual Performance Results (Benchmarked 2026-02-01)

| Scenario | Before | After | Improvement |
|----------|--------|-------|-------------|
| Cold start | ~6.0s | ~6.0s | Same |
| Warm start (full pipeline) | ~6.0s | **~2.7s** | **2.2x faster** |
| Warm start (no rerank) | ~6.0s | **~2.5s** | **2.4x faster** |

### Bottleneck Analysis

The 500ms target was not achieved. The remaining bottleneck is:

1. **Query embedding inference (~2.5s)** - Each search requires embedding the query via BGE model on Metal GPU
2. **This happens even with cached indices** - The cached index stores document embeddings, but query embeddings must be computed per-query

### Comparison with Augment MCP

| Tool | Query Latency |
|------|---------------|
| Augment MCP codebase-retrieval | **<200ms** |
| ch-cli warm start | ~2.5s |

See `notes/benchmarks/persistent-cache-vs-augment-mcp.md` for full analysis.

## Files Summary

| File | Action | Purpose |
|------|--------|---------|
| `src/retrieval/agent/pipeline.rs` | MODIFIED | Enable persistence |
| `src/retrieval/daemon/protocol.rs` | MODIFIED | Add cache protocol |
| `src/retrieval/daemon/server.rs` | MODIFIED | Implement project cache |
| `src/retrieval/daemon/client.rs` | MODIFIED | Add cache client methods |
| `src/cli/commands.rs` | MODIFIED | Use persistence by default |

## Documentation

- Created: `doc/retrieval/persistent-caching.md`

## Next Steps (To Achieve <500ms)

### Priority 1: Query Embedding Cache
Cache recent query embeddings in daemon memory to avoid re-computing:
```rust
// daemon/server.rs
query_embedding_cache: LruCache<String, Vec<f32>>,
```
**Expected improvement:** -2000ms (down to ~500ms)

### Priority 2: Keyword-Only Fast Path
For exact symbol names (e.g., "BgeEmbedder"), skip semantic search entirely:
```rust
// If query is a single PascalCase/snake_case identifier with high keyword match
// -> use Tantivy only, skip embedding
```
**Expected improvement:** -2500ms (down to ~50ms for exact matches)

### Priority 3: File Watcher Integration
Auto-invalidate cache when files change (already implemented in IndexManager but not integrated with daemon).

### Priority 4: Configurable Cache Size
Allow users to configure `MAX_CACHED_PROJECTS` via config file.
