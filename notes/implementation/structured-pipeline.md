# Structured Pipeline Implementation

## Summary

Implemented a structured retrieval pipeline with separate code, documentation, and notes search to address content type interference issues.

## Problem

The unified pipeline allowed different content types to interfere with each other's rankings. Documentation could dominate code results or vice versa.

## Solution

Created parallel pipelines that guarantee coverage for each content type:

- **Code pipeline**: Max 10 results (source files, tests)
- **Doc pipeline**: Max 5 results (markdown docs in /doc/)
- **Notes pipeline**: Max 3 results (markdown in /notes/, benchmarks)

## Architecture

```
Query → Query Expansion → Parallel Hybrid Search
                              ↓
         ┌──────────────────────────────────────┐
         │         Parallel Execution           │
         ├──────────┬──────────┬───────────────┤
         │   Code   │   Doc    │    Notes      │
         │ Pipeline │ Pipeline │   Pipeline    │
         │ (10 max) │ (5 max)  │   (3 max)     │
         └──────────┴──────────┴───────────────┘
                              ↓
              StructuredOutput (JSON/XML)
              - code_context: full file content
              - doc_context: doc sections
              - notes_context: notes sections
```

## Files Created

| File | Purpose |
|------|---------|
| `src/indexer/symbols.rs` | Added ContentType enum |
| `src/indexer/triple_search.rs` | Separate Tantivy indexes |
| `src/retrieval/hybrid/triple.rs` | Orchestrates hybrid search |
| `src/retrieval/hybrid/triple_vector_store.rs` | Separate HNSW indexes |
| `src/retrieval/agent/output.rs` | StructuredOutput types |

## Key Decisions

1. **No reranking**: Start clean without cross-encoder complexity
2. **Full file content**: Include up to 500 lines per code file (like Augment MCP)
3. **Parallel execution**: Use rayon for fast search
4. **RRF fusion**: Combine keyword + semantic per pipeline

## Usage

```bash
# JSON output (default)
ch-cli retrieve "how does auth work" --structured

# XML output
ch-cli retrieve "database connection" --structured --xml
```

## Output Format

### JSON
```json
{
  "query": "...",
  "intent": "Understand",
  "code_context": [{ "file": "...", "full_content": "...", ... }],
  "doc_context": [{ "file": "...", "content": "...", ... }],
  "notes_context": [{ "file": "...", "content": "...", ... }]
}
```

### XML
```xml
<retrieval_context>
  <query>...</query>
  <intent>...</intent>
  <code_context>...</code_context>
  <doc_context>...</doc_context>
  <notes_context>...</notes_context>
</retrieval_context>
```

## Tests

All 240 tests pass, including 20 new tests for pipeline components:
- TripleSearchIndex: 6 tests
- TripleVectorStore: 5 tests
- TripleHybridSearch: 3 tests
- StructuredOutput: 5 tests

## Date

2026-02-04

---

## Enhancement: Relevance-Based Filtering (2026-02-04)

### Problem

Fixed limits (10 code, 5 doc, 3 notes) returned noise when few results were actually relevant. Low-scoring results polluted the context.

### Solution

RRF score threshold filtering with min/max guarantees:
- Filter results below threshold (default: 0.015)
- Guarantee at least `min_results_per_type` even if below threshold
- Respect existing max limits

### Config Fields Added

| Field | Default | Purpose |
|-------|---------|---------|
| `rrf_score_threshold` | 0.015 | Minimum RRF score to include |
| `min_results_per_type` | 1 | Guaranteed results even if below threshold |

### CLI Flags Added

```bash
ch-cli retrieve "query" --structured --threshold 0.02 --min-results 2
```

### Files Modified

- `src/retrieval/hybrid/mod.rs` - Added config fields
- `src/retrieval/hybrid/triple.rs` - Filtering logic
- `src/retrieval/agent/pipeline.rs` - Config propagation
- `src/cli/mod.rs` - CLI flag parsing
- `src/cli/commands.rs` - Flag wiring

### Results

Noise reduction: irrelevant low-scoring results filtered while maintaining guaranteed minimum coverage per content type.
