# Query Expansion

## Summary

Query expansion uses a local LLM (Phi-3-mini) to interpret natural language queries and convert them into structured SearchSpec. This enables users to search code using plain English questions.

## How It Works

```
              Natural Language Query
                      │
                      │ "where is user authentication handled?"
                      ▼
              ┌───────────────────┐
              │  Query Interpreter│
              │                   │
              │  ┌─────────────┐  │
              │  │  Phi-3 LLM  │  │
              │  │  (4GB RAM)  │  │
              │  └──────┬──────┘  │
              │         │         │
              │  ┌──────▼──────┐  │
              │  │ JSON Parser │  │
              │  └─────────────┘  │
              └────────┬──────────┘
                       │
                       ▼
              ┌───────────────────┐
              │    SearchSpec     │
              │                   │
              │ symbols: ["auth"] │
              │ intent: Understand│
              │ filters: ["*.rs"] │
              └───────────────────┘
```

## SearchSpec Structure

The LLM converts queries into structured search specifications:

```rust
pub struct SearchSpec {
    /// original user query
    pub original_query: String,
    /// extracted symbol names to search for
    pub symbol_names: Vec<String>,
    /// what the user wants to do
    pub intent: QueryIntent,
    /// file filters to apply
    pub file_filters: Vec<FileFilter>,
    /// additional context hints
    pub context_hints: Vec<String>,
}
```

### Query Intents

| Intent | Description | Example Query |
|--------|-------------|---------------|
| `FindDefinition` | Locate where something is defined | "where is AuthService defined?" |
| `FindUsages` | Find where something is used | "who calls the authenticate function?" |
| `Understand` | Understand how something works | "how does the login flow work?" |
| `Modify` | Change existing code | "update the error handling in auth" |
| `Debug` | Fix an issue | "why is login failing?" |
| `Search` | General search | "find authentication code" |

## CLI Integration

Query expansion is automatic when using semantic search:

```bash
# With LLM expansion
rustean search --semantic "how does authentication work"

# The LLM interprets this as:
# - symbols: ["authenticate", "auth", "login"]
# - intent: Understand
# - filters: ["*.rs"]

# Without LLM (fallback heuristics)
rustean search "authenticate"
```

## LLM Prompt Template

The interpreter uses this prompt to guide the LLM:

```
You are a code search assistant. Given a natural language query about code, extract:
1. Symbol names (functions, classes, structs, etc.) to search for
2. The user's intent (find_definition, find_usages, understand, modify, debug, search)
3. File patterns to filter (e.g., "*.rs", "src/auth/**")

Query: {query}

Respond in this exact JSON format:
{
  "symbols": ["symbol1", "symbol2"],
  "intent": "search",
  "file_patterns": ["*.rs"],
  "hints": ["any additional context"]
}
```

## Fallback Parsing

When the LLM is unavailable or returns invalid output, a heuristic fallback is used:

1. **Symbol extraction**: Words that look like identifiers (CamelCase or snake_case)
2. **Intent detection**: Keywords in the query:
   - "where", "definition" → FindDefinition
   - "used", "calls", "references" → FindUsages
   - "how", "what", "explain" → Understand
   - "fix", "change", "modify" → Modify
   - "bug", "error", "debug" → Debug

```rust
// Example fallback parsing
let query = "where is AuthService defined";
// Result:
// - symbols: ["AuthService"]
// - intent: FindDefinition
```

## Architecture

### Source Files

| File | Purpose |
|------|---------|
| `query/mod.rs` | Module exports, parsing utilities |
| `query/llm.rs` | Phi3Model for text generation |
| `query/interpreter.rs` | QueryInterpreter orchestrator |

### Key Types

```rust
/// Phi-3 Model for text generation
pub struct Phi3Model {
    model: Phi3Model_,      // candle transformer
    tokenizer: Tokenizer,   // text tokenization
    device: Device,         // CPU/GPU
    eos_token_id: u32,      // end of sequence
}

/// Query interpreter
pub struct QueryInterpreter {
    model: Option<Phi3Model>,
}
```

### Daemon Integration

The interpreter is loaded in the daemon:

```rust
// In daemon server
DaemonRequest::Expand { query } => {
    let spec = self.interpreter.interpret(&query);
    DaemonResponse::SearchSpec(spec)
}
```

## Performance

| Operation | Time |
|-----------|------|
| Model loading | ~5-10s (one-time) |
| Query expansion | ~200-500ms |
| Fallback parsing | <1ms |

## Memory Usage

| Model | Memory |
|-------|--------|
| Phi-3-mini-4k | ~4GB |
| Embedder (BGE) | ~130MB |
| Reranker (BGE) | ~1.1GB |
| **Total** | **~5.3GB** |

## Configuration

The LLM model is configured in `RetrievalConfig`:

```rust
RetrievalConfig {
    expansion_model: "microsoft/Phi-3-mini-4k-instruct".to_string(),
    // ...
}
```

## Example Queries and Expansions

| Query | Extracted Symbols | Intent |
|-------|-------------------|--------|
| "where is user authentication?" | ["auth", "user", "authenticate"] | FindDefinition |
| "how does the parser work?" | ["parser", "parse"] | Understand |
| "fix the login error" | ["login", "error"] | Debug |
| "find all database queries" | ["database", "query", "db"] | Search |
| "who calls the validate function?" | ["validate"] | FindUsages |

## Limitations

1. **Model size**: ~4GB RAM required for LLM
2. **Latency**: 200-500ms per query expansion
3. **Context length**: 4k tokens max input
4. **Accuracy**: LLM may misinterpret complex queries

## Fallback Behavior

If the LLM fails to load or produce valid output:

```
[daemon] Failed to load LLM: <error>
[daemon] Query expansion will use fallback heuristics
```

The fallback parser still provides useful results for simple queries.

## References

- [Phi-3 Model](https://huggingface.co/microsoft/Phi-3-mini-4k-instruct)
- [Candle ML Framework](https://github.com/huggingface/candle)
