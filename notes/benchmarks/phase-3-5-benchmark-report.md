# Phase 3-5 Benchmark Report

**Date:** 2026-02-04
**Status:** Completed
**Phases Implemented:** 3, 4, 5

---

## Summary

This report compares the improved ch-cli retrieval system against Augment MCP after implementing Phases 3-5. Key improvements include enhanced cross-references (30 usages vs 2-3), connection pooling, graceful degradation, and async client support.

---

## Benchmark Results

### Query Comparison Table

| Query | ch-cli Result | Augment MCP Result | Winner |
|-------|---------------|-------------------|--------|
| `BgeEmbedder struct` | struct + 8 usages, 1.5s | Full file + usages | Tie |
| `How does retrieval pipeline work` | PLAN.md first (docs) | pipeline.rs first (code) | **Augment** |
| `RRF fusion algorithm` | rrf_score function first ✅ | Full fusion.rs | **ch-cli** |
| `SemanticGraph definition + usage` | struct + 30 usages ✅ | struct + multiple files | **ch-cli** |
| `error handling in daemon` | test file first | client.rs with retry logic | **Augment** |

### Response Time Comparison

| Metric | ch-cli | Augment MCP |
|--------|--------|-------------|
| Cold start (first query) | ~3-4s | ~2s |
| Warm cache | ~1.3-1.6s | <1s |
| With daemon restart | ~2s | N/A |

### Quality Metrics

| Metric | Before Phase 3-5 | After Phase 3-5 | Target |
|--------|------------------|-----------------|--------|
| Usages shown | 2-3 | **30** | 10+ ✅ |
| Usage grouping | None | **By file** | By file ✅ |
| Graceful degradation | No | **Yes** | Yes ✅ |
| Auto-recovery | No | **Yes** | Yes ✅ |
| Connection pooling | No | **Yes** | Yes ✅ |
| Async support | No | **Optional** | Optional ✅ |

---

## Test Count Progression

| Phase | Test Count | New Tests |
|-------|------------|-----------|
| Baseline | 139 | - |
| Phase 1-2 | 154 | +15 |
| Phase 3 | 175 | +21 |
| Phase 4 | 184 | +9 |
| Phase 5 | 189 | +5 |
| **Total** | **189** | **+50** |

---

## Improvements Achieved

### Phase 3: Enhanced Cross-References ✅
- **ReferenceContext enum** expanded with 6 new variants (FieldType, ReturnType, ParameterType, GenericArg, TraitBound, ImplTarget)
- **UsageCollection** with deduplication prevents duplicate usages
- **find_all_usages()** returns 30 usages vs 2-3 before
- **XML output** groups usages by file with context info

### Phase 4: Stability Improvements ✅
- **ConnectionPool** with RAII PooledConnection wrapper
- **Graceful degradation** with timeouts and fallback query expansion
- **Health checks** via `health_check()` function
- **Auto-recovery** via `ensure_healthy_daemon()`

### Phase 5: Optional Enhancements ✅
- **AsyncDaemonClient** with Tokio for non-blocking operations
- **Tantivy BoostQuery** for query-time document type boosting
- **document_type field** added to search schema

---

## Remaining Gaps

### 1. Conceptual Queries Favor Documentation
**Issue:** Query "How does retrieval pipeline work?" returns PLAN.md first instead of pipeline.rs code.

**Root Cause:** Conceptual/explanatory queries match documentation text better than code comments.

**Potential Fixes:**
- Increase SourceCode boost from 1.5 to 2.0 for conceptual queries
- Add query classification to detect "how does X work" patterns
- Index function/module docstrings separately with higher weight

### 2. Response Time Gap
**Issue:** ch-cli ~1.3-1.6s vs Augment <1s

**Root Cause:**
- Daemon communication overhead
- Full graph traversal for usages
- No result caching

**Potential Fixes:**
- Implement query result caching
- Optimize graph traversal with lazy loading
- Consider embedding cache warming

### 3. Test File Priority
**Issue:** Query "error handling in daemon" returns test file first.

**Root Cause:** Test files contain many keyword matches for error scenarios.

**Potential Fixes:**
- Add Test document type with 0.9 boost (lower than SourceCode)
- Reduce priority of files matching `*_test.rs`, `test_*.rs` patterns
- Consider separate test index with opt-in search

---

## Boost Configuration

### Document Type Boosts
```rust
SourceCode:    1.5  // Main code files
Documentation: 1.0  // Docs, README
Notes:         0.7  // Planning notes
Test:          0.9  // Test files
Benchmark:     0.3  // Benchmark data
```

### Symbol Kind Boosts
```rust
Function: 1.4
Method:   1.4
Struct:   1.3
Trait:    1.3
Enum:     1.2
Impl:     1.1
Field:    0.7
DocumentChunk: 0.6
```

---

## Recommendations

### Short-term (Next Sprint)
1. **Add Test document type detection** - Reduce test file priority in results
2. **Query classification** - Detect conceptual vs symbol queries
3. **Result caching** - Cache top-k results for repeated queries

### Medium-term
1. **Query-time boost tuning** - Different boost profiles for query types
2. **Lazy usage loading** - Only fetch full usages on demand
3. **Embedding cache warming** - Pre-compute embeddings for hot paths

### Long-term
1. **ML-based reranking** - Train on user click data
2. **Incremental indexing** - Update only changed files
3. **Distributed search** - Parallel search across index shards

---

## Conclusion

Phases 3-5 successfully improved:
- **Cross-references:** 30 usages shown vs 2-3 before
- **Stability:** Connection pooling, graceful degradation, auto-recovery
- **Extensibility:** Async client, Tantivy BoostQuery support

Remaining gaps are primarily in conceptual query handling and response time optimization. The boost configuration provides a solid foundation for future tuning.

**Overall Score:** ch-cli wins 2/5 queries, ties 1/5, loses 2/5 to Augment MCP. Main advantage is detailed usage tracking; main gap is conceptual query relevance.
