# Persistent Index Caching

## Summary

Persistent index caching dramatically reduces retrieval time from ~4.5s to <500ms on warm starts by persisting index data to disk and caching indexed projects in daemon memory.

## Architecture Overview

```
+-------------------------+     +------------------------+
|      rustean retrieve    | --> |   RetrievalPipeline    |
+-------------------------+     +------------------------+
                                          |
                    +---------------------+---------------------+
                    |                     |                     |
           +--------v--------+   +--------v--------+   +--------v--------+
           |  IndexManager   |   |   HybridSearch  |   |  SemanticGraph  |
           | .with_persist() |   |  .with_paths()  |   |   (in memory)   |
           +--------+--------+   +--------+--------+   +--------+--------+
                    |                     |                     |
                    v                     v                     v
           +----------------------------------------------------------+
           |                    .rustean-index/ (disk)                     |
           |  +---------------+  +---------------+  +---------------+ |
           |  |  state.json   |  |   tantivy/    |  | vectors.json  | |
           |  | (file mtimes) |  | (BM25 index)  |  | (embeddings)  | |
           |  +---------------+  +---------------+  +---------------+ |
           +----------------------------------------------------------+
```

## Performance Characteristics

| Scenario | Before | After | Improvement |
|----------|--------|-------|-------------|
| Cold start (first run) | ~4.5s | ~4.5s | Same |
| Warm start (cached on disk) | ~4.5s | **<500ms** | **9x faster** |
| Incremental (few files changed) | ~4.5s | **<1s** | **4x faster** |
| Daemon cache hit | ~4.5s | **<100ms** | **45x faster** |

## Implementation

### Phase 1: Pipeline Persistence

The `RetrievalPipeline` now uses `IndexManager.with_persistence()` by default:

```rust
// PipelineConfig now includes:
pub struct PipelineConfig {
    // ... other fields ...
    pub enable_persistence: bool,  // default: true
}

// Initialize uses persistence when enabled:
let manager = IndexManager::new()
    .with_persistence()  // Enables disk caching
    .with_semantic_analysis();
```

### Phase 2: Daemon Project Cache

The model daemon maintains an in-memory cache of indexed projects:

```rust
// New protocol messages:
enum DaemonRequest {
    IndexProject { project_path: String, force: bool },
    SearchProject { project_path: String, query: String, limit: usize },
    ProjectStatus { project_path: String },
    EvictProject { project_path: String },
}

enum DaemonResponse {
    ProjectIndexed { symbol_count: usize, cached: bool, index_time_ms: u64 },
    SearchResults(Vec<CachedSearchResult>),
    ProjectCacheStatus { cached: bool, symbol_count: usize, last_indexed: u64 },
}
```

### Incremental Indexing

The index manager tracks file modification times in `.rustean-index/state.json`:

```json
{
  "version": 1,
  "root": "/path/to/project",
  "last_updated": 1706745600,
  "files": {
    "src/main.rs": { "mtime": 1706745500, "size": 1024, "symbol_count": 15 },
    "src/lib.rs": { "mtime": 1706745400, "size": 2048, "symbol_count": 30 }
  }
}
```

On subsequent runs, only changed files are re-indexed.

## Usage

### CLI Commands

```bash
# First run (cold start ~4.5s)
rustean retrieve "BgeEmbedder"

# Second run (warm start <500ms)
rustean retrieve "HybridSearch"
```

### Programmatic API

```rust
use rustean::retrieval::agent::{PipelineConfig, RetrievalPipeline};

// Default config has persistence enabled
let config = PipelineConfig::default();
assert!(config.enable_persistence);

// Or explicitly configure
let config = PipelineConfig {
    enable_persistence: true,
    ..Default::default()
};

let mut pipeline = RetrievalPipeline::with_config(config);
let output = pipeline.retrieve("query")?;
```

### Daemon Cache API

```rust
use rustean::retrieval::daemon::DaemonClient;

let client = DaemonClient::new();

// Index a project
let (symbol_count, cached, time_ms) = client.index_project("/path/to/project", false)?;

// Search cached project
let results = client.search_project("/path/to/project", "query", 10)?;

// Check cache status
let (cached, symbols, last_indexed) = client.project_status("/path/to/project")?;

// Evict from cache
client.evict_project("/path/to/project")?;
```

## Cache Directory Structure

```
project/
  .rustean-index/
    state.json      # File modification tracking
    tantivy/        # BM25 keyword search index
    vectors.json    # Embedding vectors for semantic search
```

## LRU Eviction

The daemon maintains up to 5 cached projects (configurable via `MAX_CACHED_PROJECTS`). When the limit is exceeded, the least recently used project is evicted.

## Testing

Run the cache tests:

```bash
cargo test --lib cache
cargo test --lib persistence
cargo test --lib incremental
```

## Related Documentation

- [Agentic Pipeline](./agentic-pipeline.md)
- [Hybrid Search](./hybrid-search.md)
- [Daemon](./daemon.md)
