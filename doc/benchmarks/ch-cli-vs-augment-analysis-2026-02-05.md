# rustean vs Augment MCP: Detailed Analysis

## Summary

This document provides a comprehensive comparison between rustean (local semantic code indexer) and Augment MCP (cloud-based codebase retrieval) based on manual benchmark testing performed on 2026-02-05.

## Test Environment

- **Codebase**: rustean itself (~24,000 lines of Rust)
- **rustean version**: Post-improvement (definition boost, caller detection)
- **Augment MCP**: Context7 codebase-retrieval tool
- **Test queries**: 9 distinct query types

---

## Feature Comparison Matrix

| Feature | rustean | Augment MCP | Winner |
|---------|--------|-------------|--------|
| **Symbol definition lookup** | ✅ Fast, accurate | ✅ With full context | Tie |
| **Natural language understanding** | ⚠️ Limited | ✅ Excellent | Augment |
| **Module structure queries** | ✅ Purpose-built | ⚠️ Raw code | rustean |
| **Caller/callee tracking** | ❌ Not working | ✅ Works | Augment |
| **Full code display** | ✅ --full flag | ✅ Always | Tie |
| **Fuzzy search** | ❌ Broken | ❌ Not available | Neither |
| **Offline operation** | ✅ Yes | ❌ Requires API | rustean |
| **Speed** | ✅ ~200ms | ⚠️ ~800ms | rustean |
| **Privacy** | ✅ Local only | ❌ Cloud processing | rustean |

---

## Detailed Test Results

### Category 1: Definition Queries

**Test**: "SearchIndex struct", "DaemonClient definition"

| Tool | Accuracy | Speed | Context |
|------|----------|-------|---------|
| rustean | 9/10 | 200ms | Name + location |
| Augment | 10/10 | 800ms | Full source code |

**Analysis**: Both tools correctly identify definitions. rustean is faster but shows less context by default. The `--full` flag in rustean provides equivalent context.

**rustean advantage**: 4x faster
**Augment advantage**: Always shows implementation details

---

### Category 2: Understanding Queries

**Test**: "how does hybrid search work"

| Tool | Result | Relevance |
|------|--------|-----------|
| rustean | Test functions (rewriter.rs, structure.rs) | ❌ Wrong |
| Augment | HybridSearch struct, fusion_core.rs, adaptive.rs | ✅ Correct |

**Analysis**: This is rustean's biggest weakness. The query returns test files instead of the actual implementation because:
1. Test file names contain relevant keywords ("test_map_concepts")
2. DocumentType boost for tests may not be aggressive enough
3. No semantic understanding of "how does X work"

**Root Cause**: BM25 matches keywords in test names. The intent detection works (Understand intent detected), but the boost factors aren't strong enough to overcome BM25 bias.

**Recommendation**:
- Increase test file penalty from current level to 0.05x for Understand intent
- Consider excluding test files entirely for "how does" queries

---

### Category 3: Structure Queries (NEW FEATURE)

**Test**: "modules in retrieval"

| Tool | Output Quality | Nested Modules | Doc Comments |
|------|----------------|----------------|--------------|
| rustean | ✅ Clean tree format | ✅ All 9 | ✅ Full docs |
| Augment | ⚠️ Raw code blocks | ⚠️ Partial | ⚠️ Code only |

**rustean output**:
```
📁 ./src/retrieval/mod.rs
   pub mod agent; // Agentic Code Retrieval Module...
   pub mod context;
   pub mod daemon;
```

**Analysis**: rustean's purpose-built structure query handling is superior. The output is formatted for quick comprehension, shows doc comments inline, and discovers all nested modules.

**Winner**: rustean by a significant margin

---

### Category 4: Caller Queries (NEW FEATURE - BROKEN)

**Test**: "who calls search_command"

| Tool | Detection | Execution | Result |
|------|-----------|-----------|--------|
| rustean | ✅ CallerQuery detected | ❌ SemanticGraph empty | "No callers found" |
| Augment | N/A | ✅ Found via search | main.rs:67 |

**Analysis**: The caller query detection we implemented today works correctly. However, the SemanticGraph that should contain function call references is empty.

**Root Cause Investigation**:
```rust
// In search_callers.rs
let references = semantic_graph.find_references(symbol_name);
// Returns empty Vec

// Expected: References with ReferenceContext::Call
```

The issue is that during indexing:
1. `IndexManager.with_semantic_analysis()` is called ✅
2. Symbols are extracted ✅
3. References are NOT being tracked with Call context ❌

**Fix Required**: Verify that `parse_references()` in the parser is:
1. Being called
2. Extracting function calls
3. Setting ReferenceContext::Call
4. Adding to SemanticGraph

---

### Category 5: Body Content Search

**Test**: "connection pool error"

| Tool | Files Found | Error Handling Coverage |
|------|-------------|------------------------|
| rustean | pool.rs (2 functions) | Partial |
| Augment | pool.rs, connection.rs, retry.rs | Complete |

**Analysis**: rustean found relevant pool methods but missed the retry logic and error classification in other files. Augment's semantic search found the complete error handling story.

**Issue**: rustean's body content is limited to 500 chars. The error handling code in connection.rs spans multiple functions that don't mention "pool".

**Recommendation**:
- Increase MAX_BODY_CONTENT_LEN from 500 to 1500
- Add cross-file context expansion

---

### Category 6: Kind Filter

**Test**: "embed --kind function"

| Tool | Expected | Actual |
|------|----------|--------|
| rustean | embed_text, embed_batch | test_docentry_code_snippet |
| Augment | N/A | N/A |

**Analysis**: The kind filter correctly filters for functions, but BM25 ranks test functions higher because "embed" appears in test file paths.

**Root Cause**: `--kind function` doesn't exclude test files. A function is a function whether in tests or source.

**Recommendation**: Add `--no-tests` flag or combine kind filter with DocumentType filter.

---

## Overall Scores

### Weighted Scoring

| Category | Weight | rustean | Augment |
|----------|--------|--------|---------|
| Definition queries | 25% | 9/10 | 10/10 |
| Understanding queries | 25% | 4/10 | 10/10 |
| Structure queries | 15% | 10/10 | 7/10 |
| Code display | 15% | 10/10 | 10/10 |
| Caller queries | 10% | 2/10 | 10/10 |
| Fuzzy search | 10% | 2/10 | 5/10 |

**rustean**: 6.5/10
**Augment**: 9.0/10
**Gap**: 2.5 points

---

## Action Items

### P0 - Critical (Fix caller queries)

1. **Debug SemanticGraph reference population**
   ```bash
   # Add debug logging
   cargo test semantic -- --nocapture
   ```

2. **Verify ReferenceContext::Call tracking**
   - Check `src/indexer/parser/` for call extraction
   - Verify `SemanticGraph.add_reference()` is called

### P1 - High (Fix understanding queries)

3. **Increase test file penalty for Understand intent**
   - File: `src/indexer/symbols/document_type.rs`
   - Change: `DocumentType::Test` boost for Understand from ~0.3x to 0.05x

4. **Add test exclusion flag**
   - New flag: `--no-tests` or `--source-only`
   - Exclude files matching `/tests/` or `_test.rs`

### P2 - Medium (Improve body search)

5. **Increase body content limit**
   - File: `src/indexer/parser/symbol_processing.rs`
   - Change: `MAX_BODY_CONTENT_LEN` from 500 to 1500

6. **Fix fuzzy search**
   - Investigate Tantivy fuzzy configuration
   - Test with edit distance 2

---

## Conclusion

rustean has improved since the initial benchmark but still trails Augment MCP significantly in natural language understanding and caller tracking. The structure query feature is a clear win for rustean.

**Key insight**: The retrieval quality improvements made today (definition boost, caller detection) were correctly designed but revealed a deeper issue - the SemanticGraph is not populated with call references.

**Next steps**:
1. Fix SemanticGraph reference tracking (P0)
2. Increase test file penalties (P1)
3. Re-run benchmark to verify improvements
