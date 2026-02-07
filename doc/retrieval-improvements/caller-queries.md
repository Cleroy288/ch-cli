# Caller Query System

## Overview

The caller query system allows users to find all locations where a function/method is called, leveraging the existing `SemanticGraph` infrastructure.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Query                               │
│                    "who calls process_data"                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Query Detection Layer                         │
│                 src/retrieval/query/caller.rs                    │
│                                                                  │
│  detect_caller_query(query) → Option<CallerQuery>               │
│                                                                  │
│  Patterns:                                                       │
│  - "who calls X"      → CallerQuery { X, Callers }              │
│  - "callers of X"     → CallerQuery { X, Callers }              │
│  - "X callers"        → CallerQuery { X, Callers }              │
│  - "that call X"      → CallerQuery { X, Callers }              │
│  - "what does X call" → CallerQuery { X, Callees }              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Search Command Integration                    │
│                 src/cli/commands/search.rs                       │
│                                                                  │
│  if let Some(caller_query) = detect_caller_query(query) {       │
│      let manager = IndexManager::new().with_semantic_analysis();│
│      let result = manager.index_project(".")?;                  │
│      return handle_caller_query(&caller_query, &result.graph);  │
│  }                                                               │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Handler Layer                                 │
│              src/cli/commands/search_callers.rs                  │
│                                                                  │
│  handle_caller_query(query, graph)                              │
│  ├── Callers: find_references(name)                             │
│  │            → filter by ReferenceContext::Call                │
│  │            → display call sites                              │
│  └── Callees: find_definitions(name)                            │
│               → display definition info                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    SemanticGraph                                 │
│                 src/indexer/semantic/                            │
│                                                                  │
│  Stores:                                                         │
│  - definitions_by_name: HashMap<String, Vec<Definition>>        │
│  - references_by_name: HashMap<String, Vec<SymbolReference>>    │
│                                                                  │
│  SymbolReference {                                               │
│      name: String,                                               │
│      location: CodeLocation,                                     │
│      context: ReferenceContext::Call  ← Used for filtering      │
│  }                                                               │
└─────────────────────────────────────────────────────────────────┘
```

## Components

### 1. CallerQuery Struct

```rust
// src/retrieval/query/caller.rs

/// Result of caller query detection
#[derive(Debug, Clone)]
pub struct CallerQuery {
    /// the symbol name being queried
    pub symbol_name: String,
    /// direction of the query (callers or callees)
    pub direction: CallerDirection,
}

/// Direction of caller query
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallerDirection {
    /// who calls X
    Callers,
    /// what does X call
    Callees,
}
```

### 2. Detection Function

```rust
/// Detect if query is asking about callers/callees
pub fn detect_caller_query(query: &str) -> Option<CallerQuery> {
    let query_lower = query.to_lowercase();

    // Pattern 1: "who calls X" or "what calls X"
    if query_lower.contains("who calls") || query_lower.contains("what calls") {
        if let Some(symbol) = extract_symbol_after(&query_lower, "calls ") {
            return Some(CallerQuery {
                symbol_name: symbol,
                direction: CallerDirection::Callers,
            });
        }
    }
    // ... more patterns
}
```

### 3. Handler Function

```rust
// src/cli/commands/search_callers.rs

pub fn handle_caller_query(
    caller_query: &CallerQuery,
    semantic_graph: &SemanticGraph,
) -> CommandResult {
    match caller_query.direction {
        CallerDirection::Callers => display_callers(&caller_query.symbol_name, semantic_graph),
        CallerDirection::Callees => display_callees(&caller_query.symbol_name, semantic_graph),
    }
}
```

## ReferenceContext

The `SemanticGraph` tracks different types of references:

```rust
// src/indexer/semantic/types.rs

pub enum ReferenceContext {
    Call,           // Function/method call ← Used for caller queries
    Type,           // Type annotation
    FieldAccess,    // Field access
    Import,         // Import/use statement
    Identifier,     // Variable/identifier usage
    Unknown,
    // ... more contexts
}
```

For caller queries, we filter references by `ReferenceContext::Call` to only show actual function calls.

## Usage Examples

### Finding Callers

```bash
$ ch-cli search "who calls search_command"

Callers of 'search_command':

  1. main.rs:45 (/project/src/main.rs)
  2. cli.rs:123 (/project/src/cli.rs)
  3. test_cli.rs:67 (/project/tests/test_cli.rs)

Total: 3 call site(s)
```

### Finding Callees (Limited)

```bash
$ ch-cli search "what does main call"

Callees of 'main' (functions it calls):

Note: Callee tracking requires body analysis (not yet fully implemented)

Definition locations:
  1. main (src/main.rs:1)
```

## Limitations

1. **Callee tracking incomplete**: Full callee tracking requires analyzing function bodies to find all function calls within. Currently only shows definition location.

2. **Exact name matching**: Symbol names must match exactly (case-sensitive).

3. **Requires semantic analysis**: The `--context` flag or `with_semantic_analysis()` must be used for caller tracking to work.

## Future Improvements

1. **Full callee tracking**: Parse function bodies to extract all function calls
2. **Fuzzy name matching**: Allow partial/fuzzy symbol name matching
3. **Call graph visualization**: Generate call graph diagrams
4. **Transitive callers**: Show "all functions that eventually call X"
