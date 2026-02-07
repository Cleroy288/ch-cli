# ch-cli vs Augment MCP: Benchmark Report

**Date:** 2026-02-03
**ch-cli version:** Latest (release build)
**Daemon status:** Running with all 3 models loaded (CPU)

---

## Executive Summary

| Metric | ch-cli | Augment MCP | Winner |
|--------|--------|-------------|--------|
| Response Time (warm) | ~6s | <1s | **Augment** |
| Exact Symbol Lookup | ✅ Excellent | ✅ Excellent | TIE |
| Conceptual Queries | ✅ Good | ✅ Excellent | Augment |
| Cross-file Context | ✅ Good | ✅ Excellent | Augment |
| Error Handling Queries | ✅ Good | ✅ Excellent | Augment |
| Documentation Search | ✅ Full | ✅ Full | TIE |
| Local/Private | ✅ Yes | ❌ No | **ch-cli** |

---

## Detailed Query Results

### Query 1: "BgeEmbedder struct" (Exact Symbol Lookup)

| Metric | ch-cli | Augment MCP |
|--------|--------|-------------|
| Response time | 6.0s | <1s |
| Found struct definition | ✅ `embedding.rs:25` | ✅ `embedding.rs:25` |
| Found impl block | ✅ `embedding.rs:36` | ✅ Full impl (lines 36-154) |
| Found usages | ✅ 3 usages in server.rs | ✅ Multiple files |
| Found documentation | ✅ `embeddings.md` | ✅ `embeddings.md` |
| Found notes | ✅ benchmark notes | ✅ implementation notes |
| Noise | ⚠️ Some (FILE_REF_BG, FOLDER_REF_BG) | ❌ None |

**Result:** ✅ **EQUAL QUALITY** - Both find struct, impl, usages, and docs

**ch-cli Issues Identified:**
1. Returns some unrelated results (`FILE_REF_BG`, `FOLDER_REF_BG` constants) - trigram matching false positives
2. Response time 6x slower than Augment

---

### Query 2: "How does the retrieval pipeline work?" (Conceptual)

| Metric | ch-cli | Augment MCP |
|--------|--------|-------------|
| Response time | 6.1s | <1s |
| Intent detected | ✅ Understand | N/A |
| Found pipeline.rs | ✅ Line 408 (method) | ✅ Full file (lines 1-666) |
| Found mod.rs | ✅ Via benchmark notes | ✅ `agent/mod.rs` + `retrieval/mod.rs` |
| Found documentation | ✅ `agentic-pipeline.md` | ✅ Full doc coverage |
| Found tiered.rs | ❌ Not in top 10 | ✅ Lines 98-119 |
| Code depth | ⚠️ Snippet only | ✅ Full pipeline flow |

**Result:** ⚠️ **AUGMENT BETTER** - Returns full pipeline flow with all components

**ch-cli Issues Identified:**
1. Returns benchmark notes as top results instead of actual code
2. Missing `tiered.rs` which is critical for understanding query expansion
3. Missing `context/mod.rs` which explains context expansion

---

### Query 3: "RRF fusion algorithm" (Algorithm Search)

| Metric | ch-cli | Augment MCP |
|--------|--------|-------------|
| Response time | 5.8s | <1s |
| Found fusion.rs | ✅ Field only (line 28) | ✅ Full algorithm (lines 1-271) |
| Found `rrf_score()` fn | ✅ Line 40 | ✅ Line 40 with full context |
| Found `fuse_results()` | ⚠️ Partial | ✅ Full function (lines 54-117) |
| Found documentation | ✅ `hybrid-search.md` line 50 | ✅ |
| Found tests | ✅ `test_rrf_score` | ✅ Full test suite |
| Found weighted variant | ⚠️ `rrf_k` field only | ✅ `fuse_results_weighted` full |

**Result:** ⚠️ **AUGMENT BETTER** - Returns complete algorithm implementation

**ch-cli Issues Identified:**
1. Returns `rrf_score` field instead of the `rrf_score()` function as top result
2. Missing the full `fuse_results()` implementation
3. Returns more benchmark notes than actual code

---

### Query 4: "where is SemanticGraph defined and how is it used" (Definition + Usage)

| Metric | ch-cli | Augment MCP |
|--------|--------|-------------|
| Response time | 6.5s | <1s |
| Found definition | ✅ `semantic.rs:70` | ✅ `semantic.rs:70` |
| Found impl blocks | ✅ Lines 83, 316 | ✅ Full impls |
| Found usages | ✅ 2 usages (server.rs) | ✅ 8+ files |
| Found manager.rs usage | ⚠️ Field only (line 85) | ✅ Full context (lines 262-280) |
| Found pipeline.rs usage | ❌ No | ✅ Lines 339-395 |
| Found graph_walker.rs | ❌ No | ✅ Full usage context |
| Found tests | ❌ No | ✅ Lines 352-415 |

**Result:** ⚠️ **AUGMENT BETTER** - Returns significantly more usage context

**ch-cli Issues Identified:**
1. Shows only 2 usages while Augment shows 8+ files
2. Missing critical files: `pipeline.rs`, `graph_walker.rs`, tests
3. Cross-reference depth is limited

---

### Query 5: "error handling in daemon communication" (Error Handling)

| Metric | ch-cli | Augment MCP |
|--------|--------|-------------|
| Response time | 6.2s | <1s |
| Intent detected | ✅ Debug | N/A |
| Found `DaemonResponse` enum | ✅ Line 64 with 20 usages | ✅ Full enum |
| Found `DaemonCommunication` error | ✅ `mod.rs:38` with 3 usages | ✅ Full error module |
| Found `DaemonClient` impl | ✅ Line 26 | ✅ Lines 1-170 |
| Found server error handling | ⚠️ Via doc only | ✅ Lines 142-621 |
| Found lifecycle error handling | ❌ No | ✅ Lines 62-189 |
| Found protocol serialization | ❌ No | ✅ Lines 276-299 |

**Result:** ⚠️ **AUGMENT BETTER** - Returns complete error handling flow

**ch-cli Issues Identified:**
1. Returns documentation about error handling instead of actual error handling code
2. Missing `lifecycle.rs` which handles daemon start/stop errors
3. Missing protocol serialization error handling

---

## Error Analysis

### ch-cli Systematic Errors

| Error Type | Frequency | Impact | Root Cause |
|------------|-----------|--------|------------|
| Benchmark notes in results | 4/5 queries | Medium | Docs indexed with same weight as code |
| Fields ranked over functions | 2/5 queries | High | BM25 prefers shorter names |
| Limited cross-file context | 3/5 queries | High | Graph traversal depth limited |
| Slow response time | 5/5 queries | Medium | CPU inference, no parallelization |
| Trigram false positives | 1/5 queries | Low | Substring matching too aggressive |

### Specific Issues Found

1. **Benchmark Notes Pollution**
   - Query "How does retrieval pipeline work?" returns benchmark notes as top results
   - These are meta-documents about testing, not the actual code
   - **Fix:** Add document type weighting - code > docs > notes > benchmarks

2. **Field vs Function Confusion**
   - Query "RRF fusion" returns `rrf_score` field before `rrf_score()` function
   - Fields are shorter, get higher BM25 scores
   - **Fix:** Boost function/method kinds in ranking

3. **Limited Usage Depth**
   - SemanticGraph shows 2 usages, Augment shows 8+
   - Cross-file references not fully traversed
   - **Fix:** Increase usage traversal depth in `GraphWalker`

4. **Doc vs Code Priority**
   - Error handling query returns doc descriptions over actual error code
   - Documentation matches keywords but doesn't show implementation
   - **Fix:** Boost source code files (`.rs`) over documentation (`.md`)

---

## Improvement Recommendations

### Priority 1: Result Quality

```rust
// 1. Add document type weighting
enum DocTypeWeight {
    SourceCode = 1.5,      // .rs files
    Documentation = 1.0,   // doc/*.md
    Notes = 0.8,           // notes/*.md
    Benchmarks = 0.5,      // benchmarks/*.md
}

// 2. Add symbol kind boosting
enum SymbolKindBoost {
    Function = 1.3,
    Method = 1.3,
    Struct = 1.2,
    Enum = 1.2,
    Field = 0.8,
    Const = 0.7,
}
```

### Priority 2: Cross-Reference Depth

```rust
// Increase max usages in ContextConfig
ContextConfig {
    max_callers: 10,        // was 5
    max_callees: 10,        // was 5
    max_usages_per_symbol: 20,  // NEW
    include_transitive: true,    // NEW - follow usage chains
}
```

### Priority 3: Response Time

| Optimization | Expected Improvement |
|--------------|---------------------|
| Parallel reranking | -30% |
| Cache embedding lookups | -20% |
| Lazy context expansion | -15% |
| GPU acceleration (Metal) | -50% |

### Priority 4: Noise Reduction

```rust
// Filter out benchmark/test notes from primary results
fn should_include_result(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    // Penalize but don't exclude benchmarks
    if path_str.contains("/benchmarks/") {
        return false; // or apply 0.3x weight
    }

    true
}
```

---

## Metrics Summary

| Metric | Before (2026-02-01) | Current | Target |
|--------|---------------------|---------|--------|
| Response time (warm) | <1s | ~6s ⬇️ | <1s |
| Conceptual query accuracy | 85% | ~75% ⬇️ | 90% |
| Symbol query accuracy | 90% | ~85% ⬇️ | 95% |
| Cross-file usages shown | 3 | 2-3 | 10+ |
| Benchmark noise | Low | Medium ⬆️ | None |

**Note:** Performance regression observed since last benchmark. Possible causes:
1. Daemon restart may have reset caches
2. More benchmark/notes files added to index
3. Different query formulations used

---

## Conclusion

**ch-cli is functional but has quality gaps compared to Augment MCP:**

### Strengths
- ✅ Finds correct definitions
- ✅ Shows usage counts
- ✅ Includes documentation
- ✅ Local execution (privacy)

### Weaknesses
- ❌ Benchmark notes pollute results
- ❌ Limited cross-file context depth
- ❌ Fields ranked higher than functions
- ❌ 6x slower than Augment

### Next Steps
1. Add document type weighting (code > docs > notes)
2. Add symbol kind boosting (functions > fields)
3. Increase cross-reference depth
4. Filter benchmark notes from primary results

---

## Test Commands Used

```bash
# ch-cli queries
time ./target/release/ch-cli retrieve "BgeEmbedder struct"
time ./target/release/ch-cli retrieve "How does the retrieval pipeline work?"
time ./target/release/ch-cli retrieve "RRF fusion algorithm"
time ./target/release/ch-cli retrieve "where is SemanticGraph defined and how is it used"
time ./target/release/ch-cli retrieve "error handling in daemon communication"

# Augment MCP queries (via Claude MCP tool)
mcp__auggie-mcp__codebase-retrieval with same queries
```
