# Agentic Retrieval Pipeline

## Summary

The agentic retrieval pipeline combines all components (query expansion, hybrid search, context expansion, reranking) into a unified workflow with iterative feedback capabilities.

## Pipeline Flow

```
                Natural Language Query
                        │
                        ▼
         ┌──────────────────────────────┐
         │    Step 1: Query Expansion   │
         │    (Phi-3 LLM)               │
         │                              │
         │    "how does auth work?"     │
         │            ↓                 │
         │    symbols: ["auth"]         │
         │    intent: Understand        │
         └──────────────┬───────────────┘
                        │
                        ▼
         ┌──────────────────────────────┐
         │    Step 2: Hybrid Search     │
         │    (Tantivy + BGE)           │
         │                              │
         │    Keyword: BM25 ranking     │
         │    Semantic: Embedding sim   │
         │    Fusion: RRF (k=60)        │
         └──────────────┬───────────────┘
                        │
                        ▼
         ┌──────────────────────────────┐
         │    Step 3: Reranking         │
         │    (BGE Cross-Encoder)       │
         │                              │
         │    Score each (query, doc)   │
         │    Sort by relevance         │
         └──────────────┬───────────────┘
                        │
                        ▼
         ┌──────────────────────────────┐
         │    Step 4: Context Expansion │
         │    (SemanticGraph)           │
         │                              │
         │    + Parent scope            │
         │    + Callers/Callees         │
         │    + Related types           │
         │    + Code snippets           │
         └──────────────┬───────────────┘
                        │
                        ▼
              ┌─────────────────┐
              │   XML Output    │
              │ (LLM-ready)     │
              └─────────────────┘
```

## CLI Usage

```bash
# Run full pipeline
ch-cli retrieve "how does authentication work"

# With options
ch-cli retrieve "find the parser" --limit 5 --max-tokens 4000

# Disable specific steps
ch-cli retrieve "AuthService" --no-expand  # Skip LLM interpretation
ch-cli retrieve "login" --no-rerank        # Skip cross-encoder
ch-cli retrieve "config" --no-context      # Skip context expansion

# Raw XML output (for LLM consumption)
ch-cli retrieve "error handling" --xml

# Structured output (separate code, doc, notes sections)
ch-cli retrieve "how does auth work" --structured
ch-cli retrieve "database connection" --structured --xml
```

## Command Options

| Option | Description | Default |
|--------|-------------|---------|
| `--limit` | Max results to return | 10 |
| `--max-tokens` | Max tokens in output | 8000 |
| `--no-expand` | Skip query expansion | false |
| `--no-rerank` | Skip reranking | false |
| `--no-context` | Skip context expansion | false |
| `--xml` | Output raw XML | false |
| `--structured` | Separate code/doc/notes output | false |
| `--threshold` | RRF score threshold for filtering | 0.015 |
| `--min-results` | Minimum results per content type | 1 |

## Relevance-Based Filtering

The pipeline filters low-relevance results using RRF (Reciprocal Rank Fusion) scores.

### How RRF Scores Work

- **Higher score = more relevant**: Scores range from 0.0 to ~0.03+
- **Dual-list boost**: Results appearing in both keyword and semantic search get higher scores
- **Formula**: `RRF(d) = 1/(k + rank_keyword) + 1/(k + rank_semantic)` where k=60

### Filtering Options

| Option | Default | Description |
|--------|---------|-------------|
| `--threshold` | 0.015 | Results with RRF score below this are filtered out |
| `--min-results` | 1 | Guarantees at least N results per content type |

The `--min-results` ensures you always get results even if all scores are below threshold.

### Example Usage

```bash
# Default filtering (threshold=0.015, min-results=1)
ch-cli retrieve "authentication"

# Stricter filtering - only highly relevant results
ch-cli retrieve "auth flow" --threshold 0.02

# Looser filtering - include more marginal results
ch-cli retrieve "config" --threshold 0.01

# Guarantee at least 3 results per type
ch-cli retrieve "parser" --min-results 3

# Disable filtering entirely
ch-cli retrieve "error handling" --threshold 0.0
```

## Structured Pipeline

The `--structured` flag enables a pipeline with separate search for code, documentation, and notes. This guarantees coverage for each content type without interference.

### Architecture

```
Query → Query Expansion
              ↓
    ┌─────────────────────────────────┐
    │     Parallel Search             │
    ├───────────┬───────────┬─────────┤
    │   Code    │    Doc    │  Notes  │
    │ (10 max)  │  (5 max)  │ (3 max) │
    └───────────┴───────────┴─────────┘
              ↓
    StructuredOutput (JSON/XML)
```

### Structured Output Format

**JSON (default with --structured):**
```json
{
  "query": "how does auth work",
  "intent": "Understand",
  "code_context": [
    {
      "file": "src/auth.rs",
      "symbol": "authenticate",
      "kind": "Function",
      "line": 45,
      "signature": "pub fn authenticate(user: &User) -> Result<Token>",
      "full_content": "// Full file content up to 500 lines...",
      "line_count": 120,
      "truncated": false,
      "relevance_score": 1.0
    }
  ],
  "doc_context": [
    {
      "file": "doc/auth.md",
      "section": "Authentication Flow",
      "content": "# Authentication...",
      "relevance_score": 1.0
    }
  ],
  "notes_context": [
    {
      "file": "notes/security.md",
      "section": "Auth Implementation",
      "content": "## Notes on auth...",
      "relevance_score": 1.0
    }
  ]
}
```

**XML (with --structured --xml):**
```xml
<retrieval_context>
  <query>how does auth work</query>
  <intent>Understand</intent>
  <code_context>
    <symbol kind="Function" name="authenticate" file="src/auth.rs" line="45"
            signature="pub fn authenticate(...)" lines="120" truncated="false" score="1.000">
      <content><![CDATA[
        // Full file content...
      ]]></content>
    </symbol>
  </code_context>
  <doc_context>
    <doc file="doc/auth.md" section="Authentication Flow" score="1.000">
      <content><![CDATA[# Authentication...]]></content>
    </doc>
  </doc_context>
  <notes_context>
    <note file="notes/security.md" section="Auth Implementation" score="1.000">
      <content><![CDATA[## Notes...]]></content>
    </note>
  </notes_context>
</retrieval_context>
```

### Benefits

1. **Guaranteed coverage**: Each content type gets dedicated results
2. **No interference**: Code queries won't be drowned by docs
3. **Full file content**: Code results include complete file content (up to 500 lines)
4. **Parallel execution**: Uses rayon for fast search across all pipelines

## Output Format

### Human-Readable (default)

```
============================================================
RETRIEVAL RESULTS
============================================================

Query: how does authentication work
Intent: Understand
Symbols: auth, authenticate, login

Results: 5 found
Tokens: ~2450

------------------------------------------------------------
CONTEXT OUTPUT
------------------------------------------------------------
<context>
  <context-block>
    <symbol kind="Function" name="authenticate" ...>
      <code>
        45 | pub fn authenticate(user: &User) -> Result<Token> {
        ...
      </code>
    </symbol>
    ...
  </context-block>
</context>

Retrieval complete in 1250 ms
```

### XML Mode (`--xml`)

```xml
<context>
  <context-block>
    <symbol kind="Function" name="authenticate" file="src/auth.rs" line="45">
      <doc>Authenticates a user with credentials</doc>
      <code>
  45 | pub fn authenticate(user: &User, password: &str) -> Result<Token> {
  46 |     let hash = hash_password(password);
  ...
      </code>
    </symbol>
    <parent kind="Impl" name="AuthService" file="src/auth.rs" line="10"/>
    <callers>
      <caller name="login_handler" kind="Function" file="src/routes.rs" line="78"/>
    </callers>
  </context-block>
</context>
```

## Architecture

### Source Files

| File | Purpose |
|------|---------|
| `agent/mod.rs` | Module exports, RetrievalOutput type |
| `agent/pipeline.rs` | RetrievalPipeline orchestrator |
| `agent/feedback.rs` | FeedbackLoop for iterative refinement |

### Key Types

```rust
/// Configuration for the retrieval pipeline
pub struct PipelineConfig {
    pub max_results: usize,      // default: 10
    pub max_tokens: usize,       // default: 8000
    pub expand_query: bool,      // default: true
    pub semantic_search: bool,   // default: true
    pub rerank: bool,            // default: true
    pub expand_context: bool,    // default: true
    pub project_path: String,    // default: "."
}

/// Output from the agentic retrieval pipeline
pub struct RetrievalOutput {
    pub query: String,
    pub search_spec: SearchSpec,
    pub xml_output: String,
    pub result_count: usize,
    pub token_count: usize,
    pub has_more: bool,
}
```

### RetrievalPipeline

The main orchestrator:

```rust
let mut pipeline = RetrievalPipeline::with_config(config);
let output = pipeline.retrieve("how does auth work")?;
println!("{}", output.xml_output);
```

## Feedback Loop

The feedback loop enables iterative refinement:

```rust
let mut loop = FeedbackLoop::new(pipeline);
loop.initialize()?;

// Initial retrieval
let output = loop.initial_retrieve("authentication")?;

// Request more context
loop.apply_feedback(FeedbackAction::MoreContext {
    symbols: vec!["AuthService".to_string()],
})?;

// Refine search
loop.apply_feedback(FeedbackAction::RefineSearch {
    query: "login flow".to_string(),
})?;
```

### Available Actions

| Action | Description |
|--------|-------------|
| `MoreContext { symbols }` | Get expanded context for specific symbols |
| `RefineSearch { query }` | Run a new search with different query |
| `ExpandRelated { symbols }` | Find symbols related to given ones |
| `FilterFiles { patterns }` | Narrow search to specific files |
| `Accept` | Accept current results |

### Parsing Actions from Text

```rust
// For CLI or LLM-driven feedback
let action = parse_feedback_action("more AuthService");
// Returns: Some(FeedbackAction::MoreContext { symbols: ["AuthService"] })

let action = parse_feedback_action("search login");
// Returns: Some(FeedbackAction::RefineSearch { query: "login" })
```

## Performance

| Step | Time |
|------|------|
| Project indexing | ~500-2000ms |
| Query expansion | ~200-500ms |
| Hybrid search | ~50ms |
| Reranking | ~100-200ms |
| Context expansion | ~50-100ms |
| **Total (cached)** | **~400-800ms** |
| **Total (cold)** | **~1500-3000ms** |

## Integration with LLMs

The XML output is designed for LLM consumption:

```
User: How does authentication work in this codebase?

[ch-cli retrieve "authentication" --xml]

LLM receives:
<context>
  <context-block>
    <symbol kind="Function" name="authenticate" ...>
      [full code context]
    </symbol>
    [callers, callees, related types]
  </context-block>
</context>

LLM can now explain the authentication flow with full context.
```

## Token Budget Management

The pipeline respects `max_tokens`:

1. Context expander estimates ~4 chars per token
2. Blocks are added in relevance order
3. Stops when budget exhausted
4. `has_more` indicates more results available

## Caching

After initialization:
- Symbols are cached in memory
- Semantic graph is cached
- Hybrid search index is cached
- Subsequent queries skip re-indexing

```rust
let mut pipeline = RetrievalPipeline::new();
pipeline.initialize()?;  // Index once

// Multiple queries without re-indexing
pipeline.retrieve("auth")?;
pipeline.retrieve("config")?;
pipeline.retrieve("parser")?;
```

## Error Handling

The pipeline handles errors gracefully:

1. **LLM unavailable** → Falls back to heuristic parsing
2. **Reranker unavailable** → Uses RRF scores only
3. **Semantic graph missing** → Skips context expansion
4. **Daemon not running** → Auto-starts on first request

## Limitations

1. **Memory usage**: ~5-6GB with all models loaded
2. **Cold start**: First query includes model loading
3. **Project size**: Large projects may take longer to index
4. **Token estimation**: Rough heuristic (~4 chars/token)

## Future Improvements

1. **Incremental indexing**: Update index when files change
2. **Persistent cache**: Save index to disk
3. **Streaming output**: Return results as they're found
4. **Interactive mode**: REPL-style feedback loop
