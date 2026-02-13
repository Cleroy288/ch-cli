# rustean vs Augment MCP: Final Comparison Report

**Date:** 2026-02-05
**Test Codebase:** rustean (~15k lines Rust, 5098 symbols)
**Version:** After intent boost fixes applied

---

## Executive Summary

| Metric | rustean | Augment MCP | Winner |
|--------|--------|-------------|--------|
| **Overall Score** | **8.0/10** | **8.5/10** | Augment |
| Definition Queries | 9/10 | 9/10 | Tie |
| Conceptual Queries | 8/10 | 9/10 | Augment |
| Code Display | 6/10 | 10/10 | Augment |
| Local Operation | 10/10 | 0/10 | rustean |
| Customization | 9/10 | 3/10 | rustean |
| Response Speed | 7/10 | 8/10 | Augment |

---

## Test Results

### Test 1: "BgeEmbedder struct" (Definition Query)

**rustean Output:**
```
1. struct BgeEmbedder (embedding.rs:25)
2. impl BgeEmbedder (embedding.rs:36)
3. variant Struct (symbols.rs:55)
4. doc Find the struct (search.md:169)
5. doc DeviceInfo Struct (gpu-acceleration.md:131)
```
✅ **Struct definition at #1**

**Augment MCP Output:**
- Full BgeEmbedder struct with fields (model, tokenizer, device, dim)
- Complete impl block with new(), from_model_id(), embed_text(), embed_batch()
- 200+ lines of actual source code

✅ **Full implementation with context**

**Verdict:** Tie on relevance, Augment wins on content depth

---

### Test 2: "how does hybrid search work" (Conceptual Query)

**rustean Output:**
```
1. method initialize_hybrid_search (pipeline.rs:448)
2. fn initialize_hybrid_search (pipeline.rs:448)
3. mod hybrid (mod.rs:15)
4. field hybrid (server.rs:37)
5. field hybrid (pipeline.rs:120)
```
✅ **Source code first (after fix)**

**Augment MCP Output:**
- HybridSearch struct definition with all fields
- search(), search_adaptive(), search_with_spec() methods
- RRF fusion algorithm (fusion.rs)
- Adaptive weights logic (adaptive.rs)
- 500+ lines of implementation

✅ **Complete flow with algorithm details**

**Verdict:** Both return source code. Augment provides complete implementation.

---

### Test 3: "RetrievalError enum" (Definition Query)

**rustean Output:**
```
1. enum RetrievalError (mod.rs:38)
2. fn test_parse_enum (parser.rs:499)
3. variant RetrievalError (commands.rs:41)
4. variant Enum (symbols.rs:57)
5. const DOC_PROMPT_ENUM (prompts.rs:48)
```
✅ **Enum definition at #1**

**Augment MCP Output:**
- Full enum with all variants:
  - DaemonNotRunning, DaemonCommunication, ModelLoading, Embedding, Io
- Complete Display and From implementations
- Related error types (ModelError, etc.)

✅ **Full enum with all context**

**Verdict:** Tie on position, Augment wins on completeness

---

### Test 4: "what is RetrievalPipeline" (Conceptual Query)

**rustean Output:**
```
1. struct RetrievalPipeline (pipeline.rs:110)
2. impl RetrievalPipeline (pipeline.rs:129)
3. impl RetrievalPipeline (pipeline.rs:919)
4. method is_active (state.rs:48)
5. method is_empty (history.rs:63)
```
✅ **Struct at #1**

**Augment MCP Output:**
- PipelineConfig struct (23 configuration fields)
- RetrievalPipeline struct (8 fields: config, daemon, symbols, graph, hybrid, etc.)
- Key methods: new(), initialize(), retrieve(), retrieve_structured()
- Complete 900+ line implementation

✅ **Full architecture understanding**

**Verdict:** Both accurate. Augment provides complete implementation.

---

### Test 5: "DaemonClient" (Symbol Lookup)

**rustean Output:**
```
1. struct DaemonClient (client.rs:71)
2. impl DaemonClient (client.rs:404)
3. impl DaemonClient (client.rs:78)
4. doc Daemon Cache API (persistent-caching.md:128)
5. doc Source Files (daemon.md:154)
```
✅ **Struct at #1**

**Augment MCP Output:**
- DaemonClient struct with socket_path and timeout fields
- All methods: new(), connect(), embed(), rerank(), expand(), ping()
- ConnectionPool implementation for connection reuse
- AsyncDaemonClient variant

✅ **Full client implementation with async variant**

**Verdict:** Tie on ranking. Augment provides more depth.

---

## Detailed Comparison

### Strengths: rustean

| Strength | Description |
|----------|-------------|
| **Local Operation** | Runs entirely offline, no cloud dependency |
| **Intent Detection** | Detects "how/what/explain" → Understand intent |
| **Document Type Boost** | SourceCode: 2x boost, Docs: 0.2x for Understand |
| **Symbol Kind Boost** | Structs/Enums: 2x for FindDefinition |
| **Adaptive Weights** | Keyword/semantic balance based on query type |
| **Triple Pipeline** | Separate code/doc/notes indexes |
| **Privacy** | Code never leaves local machine |
| **Customization** | Boost factors can be tuned |

### Strengths: Augment MCP

| Strength | Description |
|----------|-------------|
| **Full Source Code** | Returns actual code, not just locations |
| **Cross-File Context** | Shows related implementations across files |
| **Semantic Understanding** | Deep understanding of "how does X work" |
| **Implementation Details** | Shows function bodies, not just signatures |
| **Real-Time Index** | Always reflects current codebase state |
| **Response Format** | Code blocks ready for reading |

### Weaknesses: rustean

| Weakness | Impact |
|----------|--------|
| **Location Only** | Shows file:line, user must open file |
| **No Function Bodies** | Doesn't index code inside functions |
| **Cold Start** | 4-5 seconds to build index |
| **No Cross-File Flow** | Can't trace how functions call each other |

### Weaknesses: Augment MCP

| Weakness | Impact |
|----------|--------|
| **Cloud Dependency** | Requires external service |
| **Privacy Concerns** | Code sent to external servers |
| **No Customization** | Can't tune ranking or boosting |
| **No Offline Mode** | Unusable without internet |

---

## Score Breakdown

### rustean: 8.0/10

| Category | Score | Notes |
|----------|-------|-------|
| Definition Queries | 9/10 | Struct/enum at #1 after fixes |
| Conceptual Queries | 8/10 | Source code first after fixes |
| Symbol Lookup | 9/10 | Accurate and fast |
| Code Display | 6/10 | Location only, no content |
| Architecture | 9/10 | Clean hybrid search design |
| Extensibility | 9/10 | Tunable boost factors |
| Privacy | 10/10 | Fully local |
| Speed | 7/10 | Cold start overhead |

### Augment MCP: 8.5/10

| Category | Score | Notes |
|----------|-------|-------|
| Definition Queries | 9/10 | Full struct with context |
| Conceptual Queries | 9/10 | Complete implementation |
| Symbol Lookup | 9/10 | Accurate with usage examples |
| Code Display | 10/10 | Full source code |
| Architecture | 8/10 | Black box, unknown internals |
| Extensibility | 3/10 | No customization options |
| Privacy | 2/10 | Code sent to cloud |
| Speed | 8/10 | Fast responses |

---

## Fixes Applied to rustean

### Fix 1: Semantic Search Intent Boost
**Location:** `src/cli/commands.rs` - `search_semantic()`

Added intent-aware post-processing to semantic search path (was missing):
```rust
let spec = fallback_parse(query);
let intent = spec.intent;
let mut boosted_results: Vec<_> = results.into_iter().map(|r| {
    let doc_type = DocumentType::from_path(&r.symbol.location.file);
    let doc_boost = doc_type.boost_factor_for_intent(&intent);
    let kind_boost = r.symbol.kind.boost_factor_for_intent(&intent);
    let boosted_score = r.rrf_score * doc_boost * kind_boost;
    (r, boosted_score)
}).collect();
```

### Fix 2: Search-Time Boost + Candidate Pool
**Location:** `src/cli/commands.rs` - `search_command()`

- Changed from `search()` to `search_with_boost()` for source code priority
- Increased candidate pool for Understand queries (10x vs 3x)

```rust
let fetch_multiplier = match intent {
    QueryIntent::Understand => 10,
    _ => 3,
};
search_index.search_with_boost(query, limit * fetch_multiplier)?
```

---

## Recommendations

### For rustean Users

1. **Use --full flag** for complete file content display
2. **Phrase queries explicitly**: "BgeEmbedder struct" vs "BgeEmbedder"
3. **Use --semantic** for conceptual queries
4. **Run `rustean daemon start`** for faster repeated queries

### Future Improvements for rustean

| Priority | Enhancement | Effort |
|----------|-------------|--------|
| P1 | Full content display by default | Low |
| P2 | Index function bodies for deeper search | Medium |
| P2 | Cross-file call graph display | Medium |
| P3 | Streaming results as they're found | High |

---

## Conclusion

**Augment MCP** remains the better tool for developers who need:
- Complete source code in results
- Deep understanding of implementations
- Cross-file context

**rustean** is the better choice for developers who need:
- Local/offline operation
- Privacy (code never leaves machine)
- Customizable ranking

After the fixes, rustean now correctly prioritizes source code over documentation for conceptual queries. The main remaining gap is **content display** - rustean shows locations, Augment shows code.
