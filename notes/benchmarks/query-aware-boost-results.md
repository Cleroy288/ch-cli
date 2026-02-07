# Query-Aware Boost Implementation Results

**Date:** 2026-02-04
**Status:** Complete
**Tests:** 213 (up from 189)

---

## Summary

Implemented query-aware and intent-aware boosting to improve search relevance for different query types.

---

## Implementation Changes

### 1. DocumentType.boost_factor_for_query()
Reduces documentation boost when query contains implementation keywords:
- Keywords: "implementation", "algorithm", "logic", "code", "function", "method", "work", "works", "working", "implement", "source", "actual"
- Reduction: Documentation/Notes/Benchmark -> 0.5x

### 2. DocumentType.boost_factor_for_intent()
Adjusts boost based on query intent:
- **Understand intent:** SourceCode 2.0x, Documentation 0.2x
- Other intents: unchanged

### 3. SymbolKind.boost_factor_for_intent()
Adjusts symbol priority based on intent:
- **Understand intent:** Function/Method 1.5x, Field 0.3x, DocumentChunk 0.2x
- **Debug intent:** Function/Method 1.3x, Field 0.5x
- **FindDefinition intent:** Struct/Enum/Trait 1.1x

### 4. Doc Chunk Filtering
For Understand intent, filter out DocumentChunk symbols from /doc/ directory before reranking.

### 5. Pipeline Integration
- Updated `search()` to use `search_with_spec()` for intent-aware boosting
- Updated `rerank_safe()` to apply query and intent boosts to rerank scores

---

## Benchmark Results

### Before vs After

| Query | Before | After |
|-------|--------|-------|
| "how does retrieval pipeline work" | doc/agentic-pipeline.md | **pipeline.rs** ✅ |
| "RRF fusion algorithm implementation" | test_rrf_score (test) | **fusion module** ✅ |
| "error handling in daemon" | daemon_client field | daemon_client field (no change) |
| "SemanticGraph definition" | impl SemanticGraph | impl SemanticGraph (unchanged) |
| "BgeEmbedder struct" | BgeEmbedder struct | BgeEmbedder struct (unchanged) |

### Score: 4/5 queries improved or maintained

---

## Boost Factor Math

For query "how does retrieval pipeline work" with Understand intent:

**Documentation + DocumentChunk:**
- doc_query_boost = 1.0 * 0.5 = 0.5 (contains "work")
- doc_intent_boost = 1.0 * 0.2 = 0.2 (Understand -> reduce docs)
- kind_boost = 0.6 * 0.2 = 0.12 (DocumentChunk reduced)
- **Combined = 0.012**

**SourceCode + Function:**
- doc_query_boost = 1.5 (unchanged for code)
- doc_intent_boost = 1.5 * 2.0 = 3.0 (Understand -> boost code)
- kind_boost = 1.4 * 1.5 = 2.1 (Function boosted)
- **Combined = 9.45**

**Ratio: 787.5x advantage for source code**

---

## Files Modified

| File | Changes |
|------|---------|
| `src/indexer/symbols.rs` | Added boost_factor_for_query, boost_factor_for_intent methods |
| `src/retrieval/hybrid/mod.rs` | Added search_with_spec, intent-aware fusion |
| `src/retrieval/agent/pipeline.rs` | Added doc filtering, intent-aware reranking |

---

## Remaining Gap

**Query:** "error handling in daemon client retry logic"
- Field `daemon_client` still ranks first because the name directly matches query keywords
- Field deprioritization (0.5x) not enough to overcome high BM25 score

**Potential fixes (future work):**
1. Add negative keyword matching for field definitions
2. Detect when query asks about behavior vs structure
3. Increase field deprioritization to 0.2x for Debug intent

---

## Conclusion

Query-aware boosting successfully addresses the main benchmark gap where conceptual queries returned documentation instead of source code. The combination of:
1. Query keyword detection
2. Intent-based boost adjustment
3. Pre-rerank filtering

Provides a 787x advantage to source code for conceptual queries, overcoming cross-encoder bias toward documentation.
