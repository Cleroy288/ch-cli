# Cross-Reference Depth Improvement Plan

**Goal:** Match Augment MCP's cross-file reference quality
**Date:** 2026-02-03
**Status:** Draft - Awaiting Approval

---

## Problem Statement

rustean returns **definition only** when searching for a symbol like `SemanticGraph`.
Augment MCP returns **definition + all 8+ files where the symbol is used** with context.

### Current Benchmark Gap
| Query | rustean | Augment MCP |
|-------|--------|-------------|
| "where is SemanticGraph defined" | Definition at `semantic.rs:69` | Definition + 8 usage files |
| Usage context depth | Limited | Full graph |

---

## Research Summary

### How rust-analyzer Does It
Source: [rust-analyzer Architecture](https://rust-analyzer.github.io/book/contributing/architecture.html)

**Three-stage funnel approach:**
1. **Text pre-filtering**: Fast trigram/text search across all files
2. **Syntax validation**: Parse candidates to filter comments/strings
3. **Semantic verification**: Confirm candidates resolve to target definition

Key insight: *"First thing an IDE does is a text search across all files, which finds potential matches. Then filters false positives by doing bottom-up traversal from candidate matches."*

### How Augment Does It
Source: [How Augment Solved the Large Codebase Problem](https://blog.codacy.com/ai-giants-how-augment-code-solved-the-large-codebase-problem)

**Context Engine approach:**
- Real-time semantic index of entire codebase
- Dependency graph tracking across files
- Semantic embeddings that capture relationships beyond syntax
- Custom embedding models trained for code retrieval

### Trigram Index Approach
Source: [Zoekt - Fast trigram based code search](https://github.com/sourcegraph/zoekt)

**How it works:**
- Map each 3-character sequence to files containing it
- Query: intersect sets for all trigrams in search pattern
- Then verify semantically in candidate files only

---

## Implementation Plan

### Phase 1: Enhanced Reference Output (Quick Win)
**Time:** 1-2 hours
**Files to modify:**
- `src/retrieval/context/block_builder.rs`
- `src/retrieval/context/mod.rs`

**What:**
1. Include full usage locations in XML output (not just count)
2. Add `<usages>` section with file:line for each reference
3. Include reference context (Call, Type, Import, etc.)

**Current output:**
```xml
<symbol kind="struct" name="SemanticGraph" file="semantic.rs" line="69" usages="15">
```

**Proposed output:**
```xml
<symbol kind="struct" name="SemanticGraph" file="semantic.rs" line="69" usages="15">
  <code>...</code>
  <usages>
    <usage file="pipeline.rs" line="89" context="Type"/>
    <usage file="commands.rs" line="339" context="Call"/>
    <usage file="block_builder.rs" line="8" context="Import"/>
    ...
  </usages>
</symbol>
```

### Phase 2: Cross-File Context Snippets
**Time:** 2-3 hours
**Files to modify:**
- `src/retrieval/context/block_builder.rs`
- `src/retrieval/context/graph_walker.rs`

**What:**
1. For each usage location, extract 2-3 lines of surrounding code
2. Group usages by file
3. Include the containing function/method name

**Proposed output:**
```xml
<usages total="15">
  <file path="src/retrieval/agent/pipeline.rs">
    <usage line="89" in="RetrievalPipeline::new" context="Type">
      graph: Option&lt;SemanticGraph&gt;,
    </usage>
    <usage line="195" in="initialize_local" context="Assignment">
      self.graph = result.semantic_graph;
    </usage>
  </file>
  <file path="src/cli/commands.rs">
    <usage line="339" in="goto_command" context="Variable">
      let graph = result.semantic_graph...
    </usage>
  </file>
</usages>
```

### Phase 3: Trigram Pre-Filter for Fast Search
**Time:** 4-6 hours
**Files to create:**
- `src/indexer/trigram.rs`

**Files to modify:**
- `src/indexer/mod.rs`
- `src/retrieval/hybrid/mod.rs`

**What:**
1. Build trigram index during symbol indexing
2. Store as `HashMap<[u8; 3], HashSet<PathBuf>>`
3. Use for fast pre-filtering before semantic search

**Algorithm:**
```rust
/// Build trigram index for fast text search
pub struct TrigramIndex {
    /// trigram -> files containing it
    index: HashMap<[u8; 3], HashSet<PathBuf>>,
}

impl TrigramIndex {
    /// Find files that might contain identifier
    pub fn candidate_files(&self, identifier: &str) -> HashSet<PathBuf> {
        let trigrams = self.extract_trigrams(identifier);
        let mut result: Option<HashSet<PathBuf>> = None;

        for trigram in trigrams {
            if let Some(files) = self.index.get(&trigram) {
                match &mut result {
                    None => result = Some(files.clone()),
                    Some(r) => *r = r.intersection(files).cloned().collect(),
                }
            }
        }

        result.unwrap_or_default()
    }

    fn extract_trigrams(&self, s: &str) -> Vec<[u8; 3]> {
        s.as_bytes()
            .windows(3)
            .map(|w| [w[0], w[1], w[2]])
            .collect()
    }
}
```

### Phase 4: Semantic Validation Layer
**Time:** 3-4 hours
**Files to modify:**
- `src/indexer/semantic.rs`
- `src/retrieval/context/graph_walker.rs`

**What:**
1. After trigram pre-filter, validate candidates semantically
2. Check that identifier resolves to correct definition
3. Filter out false positives (same name, different symbol)

**Implementation:**
```rust
/// Validate that a reference actually refers to the target symbol
pub fn validate_reference(
    &self,
    reference: &SymbolReference,
    target: &Symbol,
) -> bool {
    // 1. Check name matches
    if reference.name != target.name {
        return false;
    }

    // 2. Check if reference resolves to target definition
    let resolved = self.resolve(reference);
    resolved.definitions.iter().any(|def| {
        def.file == target.location.file && def.line == target.location.line
    })
}
```

### Phase 5: Usage Graph Persistence
**Time:** 3-4 hours
**Files to modify:**
- `src/indexer/state.rs`
- `src/indexer/manager.rs`

**What:**
1. Persist cross-reference graph to `.rustean-index/refs.json`
2. Incremental updates on file changes
3. Load from cache on startup

**Schema:**
```json
{
  "version": 2,
  "references": {
    "SemanticGraph": {
      "definition": {"file": "semantic.rs", "line": 69},
      "usages": [
        {"file": "pipeline.rs", "line": 89, "context": "Type"},
        {"file": "commands.rs", "line": 339, "context": "Call"}
      ]
    }
  }
}
```

---

## Architecture Changes

### Current Flow
```
Query → Expand → Hybrid Search → Rerank → Format XML
                                           ↓
                               (definition only)
```

### Proposed Flow
```
Query → Expand → Hybrid Search → Rerank → Collect Usages → Format XML
                                              ↓
                                    ┌─────────────────────┐
                                    │ 1. Trigram filter   │
                                    │ 2. Semantic validate│
                                    │ 3. Extract snippets │
                                    └─────────────────────┘
```

---

## File Changes Summary

| File | Change |
|------|--------|
| `src/retrieval/context/mod.rs` | Add `usages: Vec<UsageInfo>` to ContextualBlock |
| `src/retrieval/context/block_builder.rs` | Add `collect_usages()` method |
| `src/retrieval/context/graph_walker.rs` | Add `find_all_usages_with_context()` |
| `src/indexer/trigram.rs` | NEW: Trigram index implementation |
| `src/indexer/semantic.rs` | Add `validate_reference()` method |
| `src/indexer/state.rs` | Add reference cache persistence |

---

## Success Metrics

| Metric | Before | Target |
|--------|--------|--------|
| Usage locations returned | 0-1 | All (8+) |
| Cross-file snippets | None | 2-3 lines each |
| Reference validation | None | Semantic check |
| Query "where is X used" | Definition only | Full usage graph |

---

## Testing Plan

1. **Unit tests** for trigram extraction
2. **Integration test**: Query "SemanticGraph" should return 8+ usage files
3. **Benchmark**: Compare output with Augment MCP
4. **Performance**: Ensure warm query <1s

---

## Phase Order (Recommended)

1. **Phase 1** - Quick win, immediate improvement
2. **Phase 2** - Major quality jump for LLM context
3. **Phase 5** - Persistence for fast startup
4. **Phase 3** - Performance optimization
5. **Phase 4** - Accuracy improvement

---

## Questions for User

1. Should we limit max usages shown (e.g., top 20)?
2. Include usage snippets by default or only with `--verbose`?
3. Priority: Performance vs Completeness?

---

## References

- [rust-analyzer Find Usages](https://rust-analyzer.github.io/blog/2019/11/13/find-usages.html)
- [rust-analyzer Architecture](https://rust-analyzer.github.io/book/contributing/architecture.html)
- [Zoekt Trigram Search](https://github.com/sourcegraph/zoekt)
- [Augment Code Context Engine](https://blog.codacy.com/ai-giants-how-augment-code-solved-the-large-codebase-problem)
- [Trigram Index Research Paper](https://dl.acm.org/doi/10.1145/3643796.3648460)
