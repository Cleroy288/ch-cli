# Retrieval Quality Improvements - Implementation Plan

## Executive Summary

Based on benchmark analysis (rustean 7.5/10 vs Augment 8.5/10), we identified 4 remaining improvements to close the gap. This plan provides senior-engineer-level implementation details for each.

---

## Improvement 1: Stronger Definition Boost

**Status**: LOW effort, MEDIUM impact
**Current Issue**: Definition queries sometimes rank documentation/impl blocks too high

### Analysis

Looking at `src/indexer/symbols/kind_boost.rs`:
- `apply_find_definition_boost()` already gives Struct/Enum/Trait 2.0x boost
- BUT the base `boost_factor()` for Module is only 1.1x (line 19)
- Impl blocks get 0.5x reduction for FindDefinition (correct)

### Implementation

**File**: `src/indexer/symbols/kind_boost.rs`

Changes needed:
1. Increase Module base boost from 1.1 to 1.3 (modules are definitions)
2. Add stronger boost for Function/Method in FindDefinition (currently 1.5x/1.3x, increase to 1.8x)
3. Reduce DocumentChunk from 0.2x to 0.1x for FindDefinition (too many false positives)

```rust
// In apply_find_definition_boost():
SymbolKind::Function => base_boost * 1.8,  // was 1.5
SymbolKind::Method => base_boost * 1.6,    // was 1.3
SymbolKind::Module => base_boost * 1.5,    // NEW: modules are definitions
SymbolKind::DocumentChunk => base_boost * 0.1,  // was 0.2
```

### Tests

Add tests in `tests/indexer/symbols_tests.rs`:
- Test FindDefinition intent boosts function higher than before
- Test DocumentChunk gets very low score for FindDefinition

---

## Improvement 2: Caller/Callee Query Integration

**Status**: MEDIUM effort, MEDIUM impact
**Current Issue**: Queries like "who calls search_command" return nothing useful

### Analysis

The infrastructure exists:
- `SemanticGraph` tracks references via `ReferenceContext::Call`
- `find_references()` method returns all references to a symbol
- BUT: search flow doesn't use SemanticGraph for caller queries

### Architecture

```
User Query: "who calls search_command" or "callers of search_command"
     │
     ▼
┌─────────────────────────────────────────────────────┐
│  detect_caller_query() - NEW function               │
│  Pattern: "who calls X", "callers of X", "X callers"│
│  Returns: CallerQuery { symbol_name: "X" }          │
└─────────────────────────────────────────────────────┘
     │
     ▼
┌─────────────────────────────────────────────────────┐
│  handle_caller_query() - NEW function               │
│  1. Get SemanticGraph from IndexManager             │
│  2. Call find_references(symbol_name)               │
│  3. Filter for ReferenceContext::Call only          │
│  4. Format and display call sites                   │
└─────────────────────────────────────────────────────┘
```

### Implementation

**New File**: `src/retrieval/query/caller.rs`

```rust
//! Caller/callee query detection and handling

/// Result of caller query detection
pub struct CallerQuery {
    pub symbol_name: String,
    pub direction: CallerDirection,
}

pub enum CallerDirection {
    Callers,  // who calls X
    Callees,  // what does X call
}

/// Detect if query is asking about callers/callees
pub fn detect_caller_query(query: &str) -> Option<CallerQuery> {
    // Pattern: "who calls X", "callers of X", "X callers", "functions that call X"
    // Pattern: "what does X call", "callees of X", "X calls"
}
```

**Modify**: `src/retrieval/query/mod.rs`
- Add `mod caller;`
- Re-export `detect_caller_query`, `CallerQuery`

**Modify**: `src/cli/commands/search.rs`
- After `detect_structure_query()`, add `detect_caller_query()` check
- If caller query detected, call `handle_caller_query()`

**New File**: `src/cli/commands/search_callers.rs`

```rust
//! Handle caller/callee queries using SemanticGraph

pub fn handle_caller_query(
    caller_query: &CallerQuery,
    semantic_graph: &SemanticGraph,
) -> CommandResult {
    // 1. Find all references to the symbol
    // 2. Filter by ReferenceContext::Call
    // 3. Display call sites with file:line format
}
```

### Tests

Add `tests/retrieval/caller_tests.rs`:
- Test pattern detection: "who calls foo" → CallerQuery { symbol_name: "foo", direction: Callers }
- Test pattern detection: "what does foo call" → CallerQuery { symbol_name: "foo", direction: Callees }
- Test no false positive: "call the function" → None

---

## Improvement 3: Body Content Indexing

**Status**: HIGH effort, MEDIUM impact
**Current Issue**: BM25 can only match symbol names/signatures, not function body content

### Analysis

Current state:
- `Symbol` struct has `content: Option<String>` field (symbol.rs:29)
- Tantivy schema has `content` field (schema.rs:86)
- BUT: Parser doesn't populate body content for functions/methods

### Architecture

```
┌──────────────────────────────────────────────────────────┐
│ Parser extracts symbol                                   │
│ Currently: name, kind, location, signature, doc_comment  │
│ NEW: + body_content (first N lines of function body)     │
└──────────────────────────────────────────────────────────┘
     │
     ▼
┌──────────────────────────────────────────────────────────┐
│ Symbol struct                                            │
│ content field: Some("fn body content...")                │
└──────────────────────────────────────────────────────────┘
     │
     ▼
┌──────────────────────────────────────────────────────────┐
│ SearchIndex                                              │
│ Indexes content field for BM25 matching                  │
└──────────────────────────────────────────────────────────┘
```

### Implementation

**Modify**: `src/indexer/parser/rust_parser.rs` (or wherever symbol extraction happens)

```rust
// When extracting Function/Method:
// 1. Find the opening brace '{'
// 2. Extract content until closing brace or MAX_BODY_LINES
// 3. Store in symbol.content

const MAX_BODY_LINES: usize = 50;  // Limit body content size

fn extract_body_content(source: &str, start_line: usize) -> Option<String> {
    // Find the function body between { and }
    // Return first MAX_BODY_LINES of content
}
```

**Modify**: `src/indexer/search/indexing.rs`
- Ensure `symbol_to_doc()` properly indexes the content field
- May already be done - verify

### Considerations

1. **Index Size**: Body content increases index size significantly
   - Mitigation: Limit to MAX_BODY_LINES (50 lines)
   - Mitigation: Only index Function/Method bodies

2. **Performance**: More text to index = slower indexing
   - Mitigation: Only extract body for files that changed

3. **Relevance**: Body content can match irrelevant tokens
   - Mitigation: Lower boost for body matches vs name matches

### Tests

Add `tests/indexer/body_content_tests.rs`:
- Test body content extraction for simple function
- Test MAX_BODY_LINES limit
- Test search matches body content

---

## Improvement 4: Query Intent in Search Display

**Status**: ALREADY IMPLEMENTED ✓

Looking at `src/cli/commands/search.rs`:
- Line 54: `let spec = fallback_parse(query);`
- Line 55: `let intent = spec.intent;`
- Lines 75-84: Intent-aware boosts applied

This is already done!

---

## Implementation Order

1. **Improvement 1** (Definition Boost) - 15 min
   - Simple constant changes in kind_boost.rs
   - Quick win, immediate improvement

2. **Improvement 2** (Caller Queries) - 45 min
   - New query detection
   - Integration with existing SemanticGraph
   - Medium complexity

3. **Improvement 3** (Body Content) - 90 min
   - Parser changes
   - More complex, needs careful testing
   - Higher risk of performance regression

---

## Agents to Launch

| Agent | Task | Files to Modify |
|-------|------|-----------------|
| 1 | Increase definition boost factors | kind_boost.rs |
| 2 | Implement caller query detection | query/caller.rs (new), query/mod.rs |
| 3 | Implement caller query handling | commands/search_callers.rs (new), search.rs |
| 4 | Implement body content extraction | parser/rust_parser.rs |

---

## Success Metrics

After implementation, re-run benchmarks:
- "where is SearchIndex defined" → Source code ranked #1 (not docs)
- "who calls search_command" → Returns list of call sites
- "connection pool error handling" → Matches function bodies containing "connection" and "error"

Target: rustean score 8.5/10 (matching Augment)
