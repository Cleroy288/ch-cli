# Fast-Path Query Expansion

## Summary

The fast-path architecture enables <200ms response times for explicit symbol queries by bypassing the Phi-3 LLM when high-confidence symbols are detected. Conceptual queries still use the full LLM pipeline (~4.4s).

## Architecture

### 3-Tier Decision Engine

```
Query
  ↓
[Tier 1: FastPathParser] → Extract symbols via regex
  ↓
[Tier 2: SymbolValidator] → Validate in SemanticGraph, score importance
  ↓
High confidence? ──YES──→ Fast Path (skip LLM) → HybridSearch
       │
       NO
       ↓
[Tier 3: LLM Fallback] → Phi-3 expansion → HybridSearch
```

### Components

#### FastPathParser (`src/retrieval/query/fast_path.rs`)

Regex-based symbol extraction:
- **CamelCase**: `[A-Z][a-z]+(?:[A-Z][a-z0-9]+)+` (e.g., AuthService, BgeEmbedder)
- **snake_case**: `[a-z][a-z0-9]*(?:_[a-z0-9]+)+` (e.g., parse_config, get_user)
- **SCREAMING_CASE**: `[A-Z][A-Z0-9]*(?:_[A-Z0-9]+)+` (e.g., MAX_TOKENS)

Features:
- Stop-word filtering (how, where, what, find, etc.)
- Conceptual query detection ("why does", "explain", "how does X work")
- Confidence scoring per pattern type

#### SymbolValidator (`src/retrieval/query/validator.rs`)

Validates extracted symbols against the SemanticGraph:
- Checks symbol existence via `graph.find_definitions(name)`
- Counts references via `graph.find_references(name)`
- Calculates importance score with weighted factors:
  - Reference count (40%): More references = more important
  - Visibility (30%): Public > Crate > Private
  - Symbol kind (30%): Struct/Trait > Function > Field

#### TieredQueryExpander (`src/retrieval/query/tiered.rs`)

Orchestrates the decision logic:
1. Extract symbols via FastPathParser
2. Check if query is conceptual → use LLM
3. Validate symbols if graph available
4. Check thresholds (confidence, importance, existence ratio)
5. Return SearchSpec from fast-path or fall back to LLM

## Configuration

Pipeline configuration in `PipelineConfig`:

```rust
pub struct PipelineConfig {
    // ... other fields ...
    pub tiered_expansion: bool,       // Enable fast-path (default: true)
    pub fast_path_threshold: f32,     // Confidence threshold (default: 0.7)
    pub importance_threshold: f32,    // Importance threshold (default: 0.5)
}
```

## Performance

| Query Type | Before | After | Improvement |
|------------|--------|-------|-------------|
| Explicit symbol ("AuthService") | ~4.4s | <200ms | 22x faster |
| Conceptual ("why does X fail") | ~4.4s | ~4.4s | Same |
| Mixed (symbol + conceptual) | ~4.4s | ~4.4s | Same (uses LLM) |

## Query Classification

### Fast-Path (Explicit)
- "BgeEmbedder"
- "find AuthService"
- "where is parse_config defined"
- "HybridSearch and RetrievalPipeline"

### LLM Path (Conceptual)
- "Why does the daemon fail to start?"
- "Explain how authentication works"
- "What happens when a query is processed?"
- "How does the daemon load models?"

### LLM Path (Mixed)
- "How does AuthService handle tokens?"
- "Why is BgeEmbedder slow?"

## Implementation Details

### Tier Selection Logic

```rust
// Tier 1: Fast-path parsing
let fast_result = parser.extract_symbols(query);

// Conceptual → LLM
if fast_result.intent == Conceptual { return llm_expand(); }

// No symbols → LLM
if fast_result.symbols.is_empty() { return llm_expand(); }

// Low confidence → LLM
if fast_result.confidence < threshold { return llm_expand(); }

// Mixed intent → LLM
if fast_result.intent == Mixed { return llm_expand(); }

// Tier 2: Validate if graph available
if let Some(graph) = graph {
    let validation = validator.validate_symbols(&symbols);
    if validation.existence_ratio < 0.5 { return llm_expand(); }
    if validation.avg_importance < 0.5 { return llm_expand(); }
}

// Fast-path success
return fast_path_spec;
```

### Importance Scoring

```rust
fn calculate_importance(&self, name: &str) -> f32 {
    let ref_score = (refs.len() / 50.0).min(1.0);
    let vis_score = match visibility {
        Public => 1.0, PublicCrate => 0.7, Private => 0.3
    };
    let kind_score = match kind {
        Struct | Trait => 1.0,
        Function => 0.8,
        Field => 0.3,
    };

    0.4 * ref_score + 0.3 * vis_score + 0.3 * kind_score
}
```

## Testing

Run fast-path tests:
```bash
cargo test -- fast_path validator tiered
```

Manual testing:
```bash
# Fast-path query (should be <500ms)
time ./target/release/ch-cli retrieve "BgeEmbedder"

# Conceptual query (should use LLM, ~4s)
time ./target/release/ch-cli retrieve "Why does the daemon load models?"
```

## Files

| File | Purpose |
|------|---------|
| `src/retrieval/query/fast_path.rs` | Regex symbol extraction |
| `src/retrieval/query/validator.rs` | SemanticGraph validation |
| `src/retrieval/query/tiered.rs` | Tiered decision logic |
| `src/retrieval/query/mod.rs` | Module exports |
| `src/retrieval/agent/pipeline.rs` | Pipeline integration |
