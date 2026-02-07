# ch-cli vs Augment MCP Codebase-Retrieval: Implementation Notes

**Date:** 2026-02-05
**Test Codebase:** ch-cli (5098 symbols, ~15k lines Rust)
**Status:** ✅ Fixes Applied and Verified

> **Final Report:** See `doc/benchmarks/ch-cli-vs-augment-mcp-2026-02-05.md` for formal documentation

---

## Summary

| Metric | ch-cli | Augment MCP |
|--------|--------|-------------|
| **Overall Score** | **7.5/10** | **8.5/10** |
| Definition Queries | 8/10 | 9/10 |
| Conceptual Queries | 6/10 | 9/10 |
| Code Display | 6/10 | 9/10 |
| Response Speed | 7/10 | 8/10 |
| Relevance Ranking | 7/10 | 8/10 |

---

## Test Results

### Test 1: "BgeEmbedder struct"

**ch-cli (semantic search):**
```
#1 [struct] BgeEmbedder - src/retrieval/hybrid/embedding.rs:19 (score: 0.0462)
#2 [DocumentChunk] doc/retrieval/hybrid-search.md:1 (score: 0.0421)
#3 [function] new - src/retrieval/hybrid/embedding.rs:36 (score: 0.0396)
```
✅ Struct definition at #1 (intent boost working)

**Augment MCP:**
```
Returns full source code of embedding.rs (lines 1-200+)
Shows BgeEmbedder struct definition with all fields and impl blocks
```
✅ Complete implementation with context

### Test 2: "RetrievalError enum"

**ch-cli:**
```
#1 [enum] RetrievalError - src/retrieval/mod.rs:18 (score: 0.0495)
#2 [DocumentChunk] doc/retrieval/agentic-pipeline.md (score: 0.0388)
```
✅ Enum definition at #1

**Augment MCP:**
```
Returns full RetrievalError enum with all variants:
- ConfigError, IndexError, Embedding, QueryExpansion, etc.
Plus impl Display and From implementations
```
✅ Complete enum with all context

### Test 3: "how does hybrid search work"

**ch-cli:**
```
#1 [DocumentChunk] doc/retrieval/hybrid-search.md:1 (score: 0.0310)
#2 [mod] hybrid - src/retrieval/hybrid/mod.rs:1 (score: 0.0288)
#3 [DocumentChunk] doc/retrieval/agentic-pipeline.md (score: 0.0265)
```
⚠️ Returns documentation first, not source code

**Augment MCP:**
```
Returns actual source code from:
- src/retrieval/hybrid/mod.rs (full module with search functions)
- src/retrieval/hybrid/fusion.rs (RRF algorithm)
- src/retrieval/hybrid/adaptive.rs (weight computation)
- src/retrieval/hybrid/triple.rs (parallel pipeline)
```
✅ Returns actual implementation code

### Test 4: "TantivyIndex"

**ch-cli:**
```
Returns SearchIndex struct (wrapper), not Tantivy internals
```
⚠️ Doesn't distinguish between wrapper and actual Tantivy usage

**Augment MCP:**
```
Returns SearchIndex struct with:
- Schema building (build_schema function)
- Index creation (open_or_create)
- Document conversion (symbol_to_doc, doc_to_symbol)
- Search methods (search, fuzzy_search, search_by_kind)
```
✅ Shows actual Tantivy integration code

### Test 5: "RetrievalPipeline"

**ch-cli:**
```
Returns pipeline struct location without content
```

**Augment MCP:**
```
Returns full 900+ line implementation:
- PipelineConfig struct with all fields
- RetrievalPipeline struct definition
- initialize(), retrieve(), search() methods
- Context expansion and reranking logic
```
✅ Complete implementation with flow

---

## Pros and Cons

### ch-cli

**Pros:**
1. **Intent-aware boosting** - Correctly boosts struct/enum definitions for "find X" queries
2. **Adaptive weights** - Adjusts keyword vs semantic weights based on query type
3. **Triple pipeline** - Separate code/doc/notes indexes reduce noise
4. **Local operation** - No cloud dependency, works offline
5. **RRF fusion** - Good algorithm for combining keyword + semantic results
6. **Symbol extraction** - Tree-sitter parsing captures functions, structs, enums

**Cons:**
1. **No source code display** - Only shows location, not actual code
2. **Documentation pollution** - Conceptual queries return docs instead of source
3. **Limited context** - Doesn't show related code or callers/callees
4. **Slow cold start** - 4-5 seconds to build index on first query
5. **Symbol-only indexing** - Doesn't index implementation details within functions
6. **No cross-file understanding** - Can't trace data flow or dependencies

### Augment MCP

**Pros:**
1. **Full source code retrieval** - Returns actual code, not just locations
2. **Semantic understanding** - Understands "how does X work" queries
3. **Cross-file context** - Shows related implementations across files
4. **Balanced results** - Returns both definition and usage examples
5. **Real-time index** - Always reflects current codebase state
6. **Content-aware** - Indexes implementation details, not just signatures

**Cons:**
1. **Cloud dependency** - Requires external service
2. **Privacy concerns** - Code sent to external servers
3. **No explicit intent detection** - Doesn't distinguish "find X" vs "understand X"
4. **Can return too much** - Sometimes overwhelming amount of code
5. **No local customization** - Can't tune ranking or boosting

---

## Key Gaps in ch-cli

### Gap 1: No Source Code Display
ch-cli only shows symbol locations. User must manually open files.
```
Current: [struct] BgeEmbedder - src/retrieval/hybrid/embedding.rs:19
Needed:  Full struct definition with fields and comments
```

### Gap 2: Conceptual Queries Return Docs
"How does X work" returns documentation files instead of source code.
```
Current: Returns doc/hybrid-search.md first
Needed:  Returns src/retrieval/hybrid/mod.rs with actual code
```

### Gap 3: No Implementation Details
Only indexes symbol signatures, not the code inside functions.
```
Current: Indexes "fn search(...)" signature
Needed:  Index what search() actually does (calls, algorithms)
```

### Gap 4: No Cross-File Context
Can't show how a function is called or what it depends on.
```
Current: Shows where HybridSearch is defined
Needed:  Shows where it's instantiated, what calls it
```

---

## Enhancement Proposals for ch-cli

### Enhancement 1: Full Content Display (Priority: HIGH)
**Already partially implemented with --full flag**

Add `--full` flag to display complete file content around matched symbols.
```rust
// In search_command, after finding results:
if full {
    display_full_content(&hit.symbol.location, context_lines);
}
```

### Enhancement 2: Source Code Priority Boost (Priority: HIGH)
For conceptual queries, boost source code over documentation.

```rust
// In adaptive.rs, add conceptual_source_boost():
fn boost_for_conceptual_intent(doc_type: DocumentType) -> f32 {
    match doc_type {
        DocumentType::SourceCode => 2.0,  // Prefer source
        DocumentType::Test => 1.5,
        DocumentType::Documentation => 0.5,  // Deprioritize docs
        DocumentType::Notes => 0.3,
    }
}
```

### Enhancement 3: Implementation Content Indexing (Priority: MEDIUM)
Index function bodies, not just signatures.

```rust
// In symbols.rs, extend Symbol:
pub struct Symbol {
    // existing fields...
    pub body_content: Option<String>,  // Function/method body
    pub body_hash: Option<u64>,        // For incremental updates
}

// Index body content in Tantivy for full-text search
```

### Enhancement 4: Usage Graph Display (Priority: MEDIUM)
Show callers/callees in search results.

```rust
// In search output, add usage context:
[struct] BgeEmbedder - embedding.rs:19
  Used by: HybridSearch::new(), TripleHybridSearch::embed()
  Calls: Model::encode(), DaemonClient::embed()
```

### Enhancement 5: Smart Query Classification (Priority: HIGH)
Better distinguish query intent.

```rust
// Add explicit patterns for conceptual queries:
const CONCEPTUAL_PATTERNS: &[&str] = &[
    "how does", "how do", "what is", "explain", "why",
    "when to use", "difference between", "architecture of"
];

fn classify_query(query: &str) -> QueryType {
    if CONCEPTUAL_PATTERNS.iter().any(|p| query.to_lowercase().starts_with(p)) {
        return QueryType::Conceptual;
    }
    // ... existing logic
}
```

---

## ROOT CAUSE: Missing Intent Boost in Semantic Search

**Location:** `src/cli/commands.rs` - `search_semantic()` function (line 286)

**Problem:**
- Non-semantic search path applies intent-aware boosting (lines 141-170)
- Semantic search path (`search_semantic`) calls `hybrid.search()` directly without post-processing
- Result: "Understand" intent boosts for SourceCode (2.0x) and penalization for Documentation (0.2x) never applied

**Current flow (semantic):**
```
query → hybrid.search() → raw results → display
```

**Needed flow:**
```
query → hybrid.search() → intent detection → apply boosts → re-sort → display
```

**Fix:** Add intent-aware post-processing to `search_semantic()`:

```rust
// In search_semantic(), after getting results:
let spec = fallback_parse(query);
let intent = spec.intent;

// Apply intent-aware boosts
let mut boosted_results: Vec<_> = results
    .into_iter()
    .map(|r| {
        let doc_type = DocumentType::from_path(&r.symbol.location.file);
        let doc_boost = doc_type.boost_factor_for_intent(&intent);
        let kind_boost = r.symbol.kind.boost_factor_for_intent(&intent);
        let boosted_score = r.rrf_score * doc_boost * kind_boost;
        (r, boosted_score)
    })
    .collect();

boosted_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
let results: Vec<_> = boosted_results.into_iter().take(limit).map(|(r, _)| r).collect();
```

---

## Implementation Priority

| Enhancement | Effort | Impact | Priority |
|-------------|--------|--------|----------|
| **BUG FIX: Intent Boost in Semantic** | **Low** | **Critical** | ✅ **FIXED** |
| 1. Full Content Display | Low | High | ✅ Done |
| 2. Source Code Boost at Search Time | Low | High | ✅ **FIXED** |
| 5. Query Classification | Medium | High | ✅ Works (uses "how"/"what" keywords) |
| 3. Body Content Index | High | Medium | P2 |
| 4. Usage Graph Display | Medium | Medium | P2 |

---

## Fixes Applied (2026-02-05)

### Fix 1: Intent Boost in Semantic Search Path
**File:** `src/cli/commands.rs` - `search_semantic()` function

Added post-processing to apply intent-aware boosts (same as non-semantic path):
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
boosted_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
```

### Fix 2: Candidate Pool Size + Search-Time Boost
**File:** `src/cli/commands.rs` - `search_command()` function

Two changes:
1. Use `search_with_boost()` instead of `search()` for intent-aware search-time boosting
2. Increase candidate fetch size for Understand queries (10x vs 3x) to ensure source code enters candidate pool

```rust
let fetch_multiplier = match intent {
    QueryIntent::Understand => 10,  // need more candidates
    _ => 3,
};
// Use search_with_boost for better source code retrieval
search_index.search_with_boost(query, limit * fetch_multiplier)?
```

### Verification Results

| Query | Before | After |
|-------|--------|-------|
| "how does hybrid search work" | doc first | `initialize_hybrid_search` (source) |
| "BgeEmbedder struct" | struct #1 | struct #1 ✅ |
| "what is RetrievalPipeline" | docs mixed | struct #1 ✅ |

---

## Conclusion

**Augment MCP wins on usability** - Returns actual code, not just pointers.

**ch-cli wins on customization** - Intent-aware boosting, local operation.

**To close the gap**, ch-cli needs:
1. ✅ Full content display (--full flag implemented)
2. Source code priority for conceptual queries
3. Better query intent classification
4. Implementation body indexing for deeper search

The core hybrid search architecture is sound. The main issue is **output quality** - users need code, not just file locations.
