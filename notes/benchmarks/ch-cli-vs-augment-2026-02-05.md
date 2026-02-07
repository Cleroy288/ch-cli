# ch-cli vs Augment MCP Benchmark

**Date:** 2026-02-05
**After:** Module structure queries implementation + CodeRabbit fixes

---

## Test Results Summary

| Query | ch-cli Result | Augment MCP Result | Winner |
|-------|--------------|-------------------|--------|
| "modules in retrieval" | Full module hierarchy with docs | Raw mod.rs content | **ch-cli** |
| "BgeEmbedder struct" | 10 doc files (no source) | Full 228-line file | Augment |
| "how does pipeline work" | 10 doc headers | Full pipeline.rs source | Augment |
| "RetrievalError definition" | Docs #1-4, enum at #5 | Enum definition #1 | Augment |
| "callers of embed" | Docs + definitions | Definitions (no callers) | Tie |

**Score: ch-cli 1, Augment 3, Tie 1**

---

## Detailed Results

### Query 1: "modules in retrieval" (IMPROVED - ch-cli wins)

**ch-cli Output:**
```
Module structure for 'retrieval':

📁 ./src/retrieval/mod.rs
   pub mod agent; // Agentic Code Retrieval Module...
   pub mod context;
   pub mod daemon;
   pub mod docgen;
   pub mod hybrid;
   pub mod models;
   pub mod query;
   pub mod rerank;
   // 5 re-exports

📁 ./src/retrieval/context/mod.rs
   pub mod block_builder; // Context Expansion Module...
   pub mod graph_walker;
   // 2 re-exports

📁 ./src/retrieval/agent/mod.rs
   pub mod feedback; // Agentic Execution Module...
   pub mod output;
   pub mod pipeline;
   // 3 re-exports
... (8 more modules)
```

**Augment MCP Output:**
- Returns raw file content of mod.rs files
- More verbose, less structured

**Verdict:** ch-cli now provides purpose-built structure queries with:
- Hierarchical view
- Doc comments inline
- Re-export counts
- Clean formatting

---

### Query 2: "BgeEmbedder struct" (Augment wins)

**ch-cli Output:**
```
1. doc Query 1: "BgeEmbedder struct" (ch-cli-vs-augment-gaps.md:23)
2. doc Test full file content (retrieval-quality-improvements-2026-02-04.md:301)
3. doc Find the struct (search.md:169)
...
(10 doc files, no source code)
```

**Augment MCP Output:**
- Full `src/retrieval/hybrid/embedding.rs` (228 lines)
- Struct definition with all fields
- All implementation methods
- Related files

**Verdict:** Augment clearly better. ch-cli returns docs about the query, not the actual struct.

---

### Query 3: "how does pipeline work" (Augment wins)

**ch-cli Output:**
```
1. doc Run full pipeline (agentic-pipeline.md:63)
2. doc Test intent detection (retrieval-quality-improvements-2026-02-04.md:298)
...
(10 doc headers, no source code)
```

**Augment MCP Output:**
- Full `pipeline.rs` with `RetrievalPipeline` struct
- Complete `retrieve()` method showing flow
- Related modules (hybrid, context)

**Verdict:** Augment significantly better for conceptual queries.

---

### Query 4: "RetrievalError definition" (Augment wins)

**ch-cli Output:**
```
1. doc Test definition boosting (retrieval-quality-improvements-2026-02-04.md:304)
2. doc Query 2: "RetrievalError definition" (ch-cli-vs-augment-gaps.md:35)
3. doc Should NOT trigger structure query (module-structure-queries-implementation.md:539)
4. variant RetrievalError (commands.rs:41)  <- usage
5. enum RetrievalError (mod.rs:38)          <- DEFINITION
```

**Augment MCP Output:**
- `RetrievalError` enum at #1 with all variants
- Full file context

**Verdict:** Augment better. Definition at #5 vs #1.

---

### Query 5: "callers of embed" (Tie)

**ch-cli Output:**
```
1. doc Query 4: "Callers of embed function" (ch-cli-vs-augment-gaps.md:59)
...
7. method embed (client.rs:182)     <- definition
8. method embed (async_client.rs:98) <- definition
```

**Augment MCP Output:**
- Returns `embed` method definitions
- No actual call sites

**Verdict:** Tie - neither shows actual callers. Both return definitions.

---

## Gap Analysis

### What ch-cli Does Well Now
1. **Structure queries** - New feature works excellently
2. **Fast keyword search** - Instant results
3. **Index persistence** - Quick warm starts

### Remaining Gaps

| Gap | Impact | Root Cause |
|-----|--------|------------|
| Docs ranked over source | HIGH | No intent-aware boosting in basic search |
| No full file content | MEDIUM | Only shows file:line pointers |
| Definitions not prioritized | MEDIUM | 1.1x boost too weak |
| No caller tracking | LOW | Would need call graph analysis |

---

## Recommendations

### Priority 1: Add Intent Detection to Basic Search (HIGH impact, LOW effort)
```rust
// In search_command()
let intent = fallback_parse(query).intent;
// Apply intent-aware boosts before displaying results
```

### Priority 2: Add --full Flag (HIGH impact, LOW effort)
```bash
ch-cli search "BgeEmbedder struct" --full
# Returns full file content instead of pointers
```

### Priority 3: Stronger Definition Boosting (MEDIUM impact, LOW effort)
```rust
// Current: 1.1x boost for struct/enum
// Proposed: 2.0x boost + 0.2x penalty for docs
```

---

## Conclusion

The module structure query feature is a significant win - ch-cli now outperforms Augment MCP for structural queries. However, for symbol lookups and conceptual queries, Augment MCP still provides better results by returning actual source code instead of documentation files.

The remaining gaps can be addressed with relatively low effort by:
1. Integrating existing intent detection into basic search
2. Adding a --full flag for complete file content
3. Increasing definition boost factors
