# Ch-CLI vs Augment MCP: Gap Analysis

**Date:** 2026-02-04
**Status:** Complete
**Goal:** Identify what Augment MCP does better than ch-cli

---

## Executive Summary

After testing 6 comparison queries, Augment MCP significantly outperforms ch-cli in **5 key areas**:

1. **Full File Context** - Returns complete files, not truncated snippets
2. **Definition Accuracy** - Finds actual definitions, not usages
3. **Module Structure** - Returns mod.rs with module hierarchy
4. **Caller/Callee Tracking** - Shows actual call sites
5. **Multi-File Retrieval** - Returns related files in single query

---

## Detailed Comparison Results

### Query 1: "BgeEmbedder struct"

| Aspect | ch-cli | Augment MCP |
|--------|--------|-------------|
| Result | Partial struct + impl (truncated) | **Full file (228 lines)** |
| Methods shown | 0 | All 6 methods |
| Implementation details | None | Complete |

**Gap:** ch-cli truncates at ~50 lines, Augment returns full file context.

---

### Query 2: "RetrievalError definition"

| Aspect | ch-cli | Augment MCP |
|--------|--------|-------------|
| Result | Error usage in commands.rs | **Enum definition in mod.rs** |
| Variants shown | 0 | All 5 variants |
| Location accuracy | Wrong file | Correct file |

**Gap:** ch-cli returns where error is *used*, not where it's *defined*.

---

### Query 3: "Module overview"

| Aspect | ch-cli | Augment MCP |
|--------|--------|-------------|
| Result | notes/implementation files | **mod.rs files with structure** |
| Module list | Documentation prose | Actual pub mod declarations |
| Exports shown | None | All re-exports |

**Gap:** ch-cli returns documentation about modules, Augment returns actual module source.

---

### Query 4: "Callers of embed function"

| Aspect | ch-cli | Augment MCP |
|--------|--------|-------------|
| Result | find_callers function definition | **Actual call sites** |
| Locations shown | Function that *finds* callers | Code that *calls* embed |
| Practical use | Low | High |

**Gap:** ch-cli misinterprets "callers" as conceptually related code.

---

### Query 5: "How is daemon client implemented"

| Aspect | ch-cli | Augment MCP |
|--------|--------|-------------|
| Result | Doc headers about daemon | **Full client.rs (400+ lines)** |
| Code shown | 0 lines | RetryConfig, DaemonClient, ConnectionPool |
| Implementation details | None | All methods with retry logic |

**Gap:** ch-cli returns documentation, Augment returns actual implementation.

---

### Query 6: "What modules are in retrieval directory"

| Aspect | ch-cli | Augment MCP |
|--------|--------|-------------|
| Result | Doc headers mentioning retrieval | **All mod.rs files** |
| Modules listed | 0 | 8 submodules with purposes |
| File structure | None | Complete hierarchy |

**Gap:** ch-cli doesn't understand structural queries about code organization.

---

## Root Cause Analysis

### Why Augment MCP Wins

1. **Full File Retrieval**
   - Augment returns complete files (200-400+ lines)
   - ch-cli truncates at ~50 lines per symbol
   - *Impact:* User sees full context vs fragments

2. **Definition vs Usage**
   - Augment prioritizes definition sites
   - ch-cli ranks by keyword match strength
   - *Impact:* Wrong file returned for "definition" queries

3. **Structural Understanding**
   - Augment understands "module", "structure", "what's in"
   - ch-cli treats these as keyword queries
   - *Impact:* Returns mod.rs vs random mentions

4. **Call Graph Awareness**
   - Augment has reference tracking
   - ch-cli has SemanticGraph but doesn't use it for retrieval
   - *Impact:* Can't answer "who calls X" queries

5. **Multi-File Synthesis**
   - Augment returns multiple related files per query
   - ch-cli returns ranked list of single symbols
   - *Impact:* Augment gives complete picture

---

## Recommended Improvements

### Priority 1: Full File Context (High Impact)
```
Current: Return 50-line symbol snippets
Target:  Return full files (up to 500 lines)
Effort:  Medium (change output formatter)
```

### Priority 2: Definition Boosting (High Impact)
```
Current: Rank by keyword match
Target:  Boost definition sites for "definition/struct/enum" queries
Effort:  Low (add query-aware boost like we did for docs)
```

### Priority 3: Module Structure Queries (Medium Impact)
```
Current: Treats "modules in X" as keyword search
Target:  Detect structural queries, return mod.rs files
Effort:  Medium (add query type detection)
```

### Priority 4: Reference Integration (High Impact)
```
Current: SemanticGraph has call graph, not used in retrieval
Target:  Use references for "callers/callees" queries
Effort:  High (integrate reference index into hybrid search)
```

### Priority 5: Multi-File Retrieval (Medium Impact)
```
Current: Return single symbols
Target:  Group related files in output
Effort:  Medium (post-process results by file)
```

---

## Comparison Matrix

| Capability | ch-cli | Augment MCP | Gap |
|------------|--------|-------------|-----|
| Keyword search | ✅ | ✅ | - |
| Semantic search | ✅ | ✅ | - |
| Cross-encoder reranking | ✅ | ❓ | - |
| Query expansion | ✅ | ✅ | - |
| Full file context | ❌ | ✅ | **HIGH** |
| Definition accuracy | ❌ | ✅ | **HIGH** |
| Module structure queries | ❌ | ✅ | **MEDIUM** |
| Caller/callee tracking | ❌ | ✅ | **HIGH** |
| Multi-file results | ❌ | ✅ | **MEDIUM** |
| Intent-aware boosting | ✅ | ❓ | - |

---

## Conclusion

Ch-cli has strong fundamentals (hybrid search, reranking, query expansion) but lacks:

1. **Output completeness** - Need full files, not snippets
2. **Query type awareness** - Need to detect definition/structure/reference queries
3. **Reference integration** - Need to use SemanticGraph for call queries

The 3 priority improvements would close ~80% of the gap with Augment MCP.
