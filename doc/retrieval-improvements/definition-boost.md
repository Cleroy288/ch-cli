# Definition Boost Enhancement

## Overview

Enhanced boost factors for `FindDefinition` intent queries to prioritize actual source code definitions over documentation and impl blocks.

## Problem Statement

When users search for definitions like "where is SearchIndex defined", the results sometimes showed:
1. Documentation files mentioning SearchIndex
2. Impl blocks using SearchIndex
3. Actual struct/function definition (should be #1)

## Solution Architecture

```
Query: "SearchIndex definition"
       │
       ▼
┌─────────────────────────────────┐
│  fallback_parse(query)          │
│  → intent: FindDefinition       │
└─────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────┐
│  For each search hit:           │
│  doc_boost = doc_type.boost_for_intent(&intent)
│  kind_boost = symbol.kind.boost_for_intent(&intent)
│  final_score = score * doc_boost * kind_boost
└─────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────┐
│  Sort by final_score            │
│  → Definitions ranked higher    │
└─────────────────────────────────┘
```

## Implementation Details

### File: `src/indexer/symbols/kind_boost.rs`

#### Base Boost Factor Changes

```rust
// In boost_factor() method
pub fn boost_factor(&self) -> f32 {
    match self {
        // ...
        SymbolKind::Module => 1.3,  // was 1.1, increased for definition ranking
        // ...
    }
}
```

#### FindDefinition Intent Boost Changes

```rust
fn apply_find_definition_boost(&self, base_boost: f32) -> f32 {
    match self {
        // Type definitions - strong boost
        SymbolKind::Struct => base_boost * 2.0,
        SymbolKind::Enum => base_boost * 2.0,
        SymbolKind::Trait => base_boost * 2.0,
        SymbolKind::TypeAlias => base_boost * 1.8,

        // Function definitions - stronger boost (was 1.5/1.3)
        SymbolKind::Function => base_boost * 1.8,  // increased
        SymbolKind::Method => base_boost * 1.6,    // increased

        // Module definitions - new boost
        SymbolKind::Module => base_boost * 1.5,    // NEW

        // Impl blocks are usages - reduce
        SymbolKind::Impl => base_boost * 0.5,

        // Documentation is not a definition - heavily reduce (was 0.2)
        SymbolKind::DocumentChunk => base_boost * 0.1,  // reduced further

        _ => base_boost,
    }
}
```

## Boost Calculation Examples

### Query: "SearchIndex definition"

| Symbol | Kind | Base | FindDef Mult | Final Boost |
|--------|------|------|--------------|-------------|
| SearchIndex struct | Struct | 1.3 | 2.0x | 2.6 |
| SearchIndex docs | DocumentChunk | 0.6 | 0.1x | 0.06 |
| impl SearchIndex | Impl | 1.2 | 0.5x | 0.6 |
| search_index module | Module | 1.3 | 1.5x | 1.95 |

Result: Struct definition (2.6) ranks much higher than docs (0.06).

### Query: "process_data function"

| Symbol | Kind | Base | FindDef Mult | Final Boost |
|--------|------|------|--------------|-------------|
| fn process_data | Function | 1.4 | 1.8x | 2.52 |
| process_data docs | DocumentChunk | 0.6 | 0.1x | 0.06 |

Result: Function definition (2.52) ranks much higher than docs (0.06).

## Test Coverage

Tests in `tests/indexer/symbols_tests.rs`:

```rust
#[test]
fn test_symbol_kind_boost_factor_high_priority() {
    assert_eq!(SymbolKind::Module.boost_factor(), 1.3);  // updated
}

#[test]
fn test_symbol_kind_boost_factor_ordering() {
    // Struct and Module now have equal boost (1.3)
    assert!(SymbolKind::Struct.boost_factor() >= SymbolKind::Module.boost_factor());
}
```

## Rationale for Changes

### Why increase Module boost?

Modules are definitions. When someone asks "where is retrieval defined", the module declaration is a valid answer.

### Why increase Function/Method boost for FindDefinition?

Function definitions should clearly outrank any documentation or usage. The previous 1.5x was not aggressive enough.

### Why add Module to FindDefinition handling?

Module declarations (`mod retrieval;`) are definitions that should be boosted when looking for "where is X defined".

### Why reduce DocumentChunk to 0.1x?

Documentation should almost never appear when looking for definitions. The previous 0.2x still allowed docs to compete in edge cases.
