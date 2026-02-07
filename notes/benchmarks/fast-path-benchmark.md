# Fast-Path vs Augment MCP Benchmark

## Date: 2026-02-01
## Project: ch-cli (1769 symbols)

## Summary

| Metric | ch-cli Fast-Path | ch-cli LLM | Augment MCP |
|--------|------------------|------------|-------------|
| Query Expansion | **0ms** | 20-100ms | N/A (persistent index) |
| Full Pipeline (cold) | ~4.5s | ~4.5s | <1s |
| Full Pipeline (warm) | N/A | N/A | <1s |

**Key Finding**: The fast-path query expansion achieves **0ms** for explicit symbol queries, but the ch-cli CLI re-indexes on each invocation (~4.5s). Augment MCP maintains a persistent index, providing consistently fast responses.

## Test Environment

- **Hardware**: Apple Silicon (Metal GPU, 25GB VRAM)
- **Daemon**: Running with Phi-3, BGE-small, BGE-reranker
- **Project**: ch-cli (1769 symbols, ~15k lines Rust)

## Query Expansion Benchmark

### ch-cli Tiered Expansion Results

| Query | Tier Used | Time |
|-------|-----------|------|
| `BgeEmbedder` | FastPathValidated | **0ms** |
| `HybridSearch` | FastPathValidated | **0ms** |
| `SemanticGraph` | FastPathValidated | **0ms** |
| `FastPathParser` | FastPathValidated | **0ms** |
| `where is RetrievalPipeline` | FastPathValidated | **0ms** |
| `parse_config` | LlmExpansion | 5ms |
| `find AuthService` | LlmExpansion | 67ms |
| `Why does daemon fail` | LlmExpansion | 72ms |
| `How does embedding work` | LlmExpansion | 21ms |
| `explain the search flow` | LlmExpansion | 96ms |

### Analysis

1. **CamelCase symbols** (BgeEmbedder, HybridSearch, etc.): **0ms** via fast-path
2. **Validated symbols that exist**: Fast-path succeeds
3. **Non-existent symbols** (AuthService, parse_config): Fall back to LLM for semantic understanding
4. **Conceptual queries**: Correctly routed to LLM (20-100ms with warm model)

## Pipeline Breakdown

### ch-cli Full Pipeline (~4.5s total)

| Step | Time | Notes |
|------|------|-------|
| Project indexing | ~3.5s | Re-indexes each CLI invocation |
| Hybrid search index build | ~0.5s | Generates embeddings via daemon |
| Query expansion (fast-path) | **0ms** | When symbols validated |
| Query expansion (LLM) | 20-100ms | When LLM needed |
| Hybrid search | ~50ms | BM25 + HNSW |
| Reranking | ~200ms | Cross-encoder |
| Context expansion | ~100ms | SemanticGraph traversal |

### Augment MCP (~instant response)

| Step | Time | Notes |
|------|------|-------|
| Query processing | <100ms | Persistent index |
| Search | <100ms | Pre-built embeddings |
| Response formatting | <50ms | Streaming |

## Fast-Path Decision Logic

```
Query → FastPathParser
         ↓
    Extract symbols via regex:
    - CamelCase: 0.95 confidence
    - SCREAMING_CASE: 0.90 confidence
    - snake_case: 0.85 confidence
         ↓
    Is conceptual? ("why", "how does", "explain")
         ↓ NO
    Validate in SemanticGraph
         ↓
    existence_ratio > 0.5 AND avg_importance > 0.5?
         ↓ YES
    FAST-PATH (0ms) → Skip LLM
```

## Recommendations

### For ch-cli Improvement

1. **Persistent index caching**: Store indexed symbols/embeddings between CLI runs
2. **Daemon-side caching**: Cache project index in daemon process
3. **Incremental indexing**: Only re-index changed files

### Fast-Path Effectiveness

- **100% hit rate** for pure CamelCase symbol queries
- **0% hit rate** for conceptual/explanatory queries (correct behavior)
- **Validation prevents false positives**: Non-existent symbols correctly fall back to LLM

## Conclusion

The fast-path implementation achieves the target **0ms query expansion** for explicit symbol queries. The remaining latency (~4.5s) is dominated by project indexing, which runs on each CLI invocation.

**Comparison**:
- ch-cli fast-path query expansion: **0ms** (goal achieved)
- Augment MCP total response: **<1s** (persistent index advantage)

To match Augment MCP's total latency, ch-cli needs persistent index caching.

## Test Commands

```bash
# Run ch-cli benchmark
./target/release/ch-cli retrieve "BgeEmbedder" --limit 3 2>&1 | grep "Tiered expansion"

# Compare multiple queries
for q in "BgeEmbedder" "HybridSearch" "why does daemon fail"; do
    echo "Query: $q"
    ./target/release/ch-cli retrieve "$q" --limit 3 2>&1 | grep "Tiered expansion"
done
```
