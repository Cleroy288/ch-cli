# Retrieve Command

Run the full agentic retrieval pipeline for natural language code queries.

## Summary

The `retrieve` command is the most powerful way to search your codebase. Unlike the basic `search` command that matches symbol names, `retrieve` uses a multi-stage AI pipeline to understand your intent and return contextually relevant code with full surrounding context.

## Syntax

```bash
rustean retrieve <QUERY> [OPTIONS]
```

## Arguments

| Argument | Description |
|----------|-------------|
| `<QUERY>` | Natural language query (required) |

## Options

| Option | Description | Default |
|--------|-------------|---------|
| `-l, --limit <LIMIT>` | Maximum number of results | 10 |
| `-m, --max-tokens <MAX_TOKENS>` | Maximum tokens in output | 8000 |
| `--no-expand` | Disable query expansion (LLM interpretation) | false |
| `--no-rerank` | Disable cross-encoder reranking | false |
| `--no-context` | Disable context expansion | false |
| `--xml` | Output raw XML (for LLM consumption) | false |
| `--structured` | Use structured output with separate code, doc, notes sections | false |
| `--threshold <THRESHOLD>` | RRF score threshold for relevance filtering | 0.015 |
| `--min-results <MIN_RESULTS>` | Minimum results per content type | 1 |
| `-h, --help` | Print help information | - |

## Description

The `retrieve` command orchestrates a full agentic retrieval pipeline that transforms natural language queries into relevant code context. It combines multiple AI components to deliver intelligent, context-aware search results.

### Pipeline Stages

```
Natural Language Query
        |
        v
+---------------------------+
| 1. Query Expansion        |
|    (Phi-3 LLM)            |
|                           |
|    Extracts:              |
|    - Symbol names         |
|    - Intent (Understand,  |
|      FindDefinition, etc) |
|    - Search context       |
+------------+--------------+
             |
             v
+---------------------------+
| 2. Hybrid Search          |
|    (Tantivy + BGE)        |
|                           |
|    - Keyword: BM25        |
|    - Semantic: Embedding  |
|    - Fusion: RRF (k=60)   |
+------------+--------------+
             |
             v
+---------------------------+
| 3. Reranking              |
|    (BGE Cross-Encoder)    |
|                           |
|    Score (query, doc)     |
|    pairs for relevance    |
+------------+--------------+
             |
             v
+---------------------------+
| 4. Context Expansion      |
|    (SemanticGraph)        |
|                           |
|    Adds:                  |
|    - Parent scope         |
|    - Callers/Callees      |
|    - Related types        |
|    - Code snippets        |
+------------+--------------+
             |
             v
      XML/JSON Output
      (LLM-ready)
```

## Usage Examples

### Natural Language Queries

```bash
# Ask how something works
rustean retrieve "how does authentication work"

# Find where something is defined
rustean retrieve "where is the parser defined"

# Debug an issue
rustean retrieve "what causes the login error"

# Understand a concept
rustean retrieve "explain the retrieval pipeline"
```

### Controlling Results

```bash
# Limit to 5 results
rustean retrieve "config handling" --limit 5

# Allow more tokens in output
rustean retrieve "how does caching work" --max-tokens 16000

# Get more marginal results (lower threshold)
rustean retrieve "auth" --threshold 0.01

# Guarantee at least 3 results per content type
rustean retrieve "parser" --min-results 3
```

### Disabling Pipeline Stages

```bash
# Skip LLM query expansion (faster, uses heuristics)
rustean retrieve "AuthService" --no-expand

# Skip cross-encoder reranking
rustean retrieve "login" --no-rerank

# Skip context expansion (callers, callees, etc.)
rustean retrieve "config" --no-context

# Minimal pipeline (fastest)
rustean retrieve "MyStruct" --no-expand --no-rerank --no-context
```

### LLM Consumption (--xml)

```bash
# Output raw XML for piping to an LLM
rustean retrieve "error handling" --xml

# Combine with other options
rustean retrieve "authentication flow" --xml --limit 5
```

Use the `--xml` flag when integrating with LLM tools. The output is structured XML designed for LLM context windows.

### Structured Output (--structured)

```bash
# Get separate code, doc, notes sections
rustean retrieve "how does auth work" --structured

# Structured XML output for LLMs
rustean retrieve "database connection" --structured --xml
```

The `--structured` flag runs three parallel search pipelines:
- **Code** (max 10 results): Source code symbols
- **Doc** (max 5 results): Documentation files (doc/, docs/)
- **Notes** (max 3 results): Development notes (notes/)

This guarantees coverage for each content type without interference.

## Output Formats

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

### XML Mode (--xml)

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

### Structured JSON (--structured)

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
  "doc_context": [...],
  "notes_context": [...]
}
```

### Structured XML (--structured --xml)

```xml
<retrieval_context>
  <query>how does auth work</query>
  <intent>Understand</intent>
  <code_context>
    <symbol kind="Function" name="authenticate" file="src/auth.rs" line="45"
            signature="pub fn authenticate(...)" lines="120" truncated="false" score="1.000">
      <content><![CDATA[// Full file content...]]></content>
    </symbol>
  </code_context>
  <doc_context>...</doc_context>
  <notes_context>...</notes_context>
</retrieval_context>
```

## Search vs Retrieve

| Feature | `search` | `retrieve` |
|---------|----------|------------|
| Input | Symbol name | Natural language query |
| Query Expansion | No | Yes (LLM-powered) |
| Intent Detection | Limited | Yes (Understand, FindDefinition, FindUsages, Debug) |
| Hybrid Search | Optional (`--semantic`) | Always |
| Reranking | Optional (`--rerank`) | Default (disable with `--no-rerank`) |
| Context Expansion | Optional (`--context`) | Default (disable with `--no-context`) |
| Output Format | Text list | XML context blocks |
| Use Case | Find a known symbol | Understand how something works |
| Performance | Fast | Slower (more processing) |

### When to Use Each

**Use `search` when:**
- You know the exact or partial symbol name
- You need quick, simple results
- Performance is critical

```bash
rustean search MyStruct
rustean search handle --fuzzy --kind function
```

**Use `retrieve` when:**
- You have a conceptual question
- You want to understand how code works
- You need context (callers, callees, related types)
- You're building LLM context

```bash
rustean retrieve "how does the parser work"
rustean retrieve "authentication flow" --xml
```

## Query Intent Detection

The pipeline detects your intent from the query:

| Intent | Trigger Words | Behavior |
|--------|---------------|----------|
| `Understand` | "how does", "explain", "what is" | Broad context, related types |
| `FindDefinition` | "where is", "defined", "declaration" | Focus on definitions |
| `FindUsages` | "used", "called", "references" | Focus on call sites |
| `Debug` | "error", "bug", "fix", "wrong" | Error-related context |
| `Search` | (default) | Balanced search |

## Relevance-Based Filtering

The pipeline filters low-relevance results using RRF (Reciprocal Rank Fusion) scores.

### How RRF Scores Work

- **Higher score = more relevant**: Scores range from 0.0 to ~0.03+
- **Dual-list boost**: Results appearing in both keyword and semantic search score higher
- **Formula**: `RRF(d) = 1/(k + rank_keyword) + 1/(k + rank_semantic)` where k=60

### Filtering Examples

```bash
# Default filtering (threshold=0.015, min-results=1)
rustean retrieve "authentication"

# Stricter filtering - only highly relevant results
rustean retrieve "auth flow" --threshold 0.02

# Looser filtering - include more marginal results
rustean retrieve "config" --threshold 0.01

# Guarantee at least 3 results per type
rustean retrieve "parser" --min-results 3

# Disable filtering entirely
rustean retrieve "error handling" --threshold 0.0
```

## Performance

| Step | Time |
|------|------|
| Project indexing | ~500-2000ms |
| Query expansion | ~200-500ms |
| Hybrid search | ~50ms |
| Reranking | ~100-200ms |
| Context expansion | ~50-100ms |
| **Total (warm cache)** | **~400-800ms** |
| **Total (cold start)** | **~1500-3000ms** |

### Performance Tips

1. **Use daemon caching**: The pipeline uses persistent caching for fast warm starts (<500ms vs ~4.5s cold)
2. **Disable unused stages**: Use `--no-expand`, `--no-rerank`, `--no-context` for speed
3. **Lower limits**: Use `--limit 5` if you only need a few results
4. **Higher threshold**: Use `--threshold 0.02` to reduce noise

## Error Handling

The pipeline handles errors gracefully:

| Error | Fallback |
|-------|----------|
| LLM unavailable | Uses heuristic parsing (symbols, intent) |
| Reranker unavailable | Uses RRF scores only |
| Semantic graph missing | Skips context expansion |
| Daemon not running | Auto-starts on first request |

## Tips for Optimal Usage

### Writing Good Queries

```bash
# Good: Specific and descriptive
rustean retrieve "how does the authentication middleware validate JWT tokens"

# OK: General but clear
rustean retrieve "authentication flow"

# Avoid: Too vague
rustean retrieve "auth"
```

### LLM Integration Pattern

```bash
# 1. Get context for LLM
context=$(rustean retrieve "error handling in database module" --xml)

# 2. Pipe to your LLM tool
echo "$context" | llm "Explain this error handling approach"
```

### Exploring a Codebase

```bash
# Start broad
rustean retrieve "main entry point"

# Then narrow down
rustean retrieve "how does the CLI parse arguments"

# Finally, specific questions
rustean retrieve "where is the search command defined"
```

### Structured Output for Multi-Source Context

```bash
# Get code + docs + notes in one query
rustean retrieve "authentication" --structured --xml

# This ensures you get relevant code AND documentation
```

## Prerequisites

The retrieval pipeline requires:

1. **Indexed project**: Run `rustean index --semantic` first
2. **ML daemon**: Automatically started on first use
3. **Memory**: ~5-6GB for all models loaded

## See Also

- [Search Command](./search.md) - Basic symbol search
- [Index Command](./index.md) - Build the code index
- [Navigation Commands](./navigation.md) - Go-to-definition and find references
- [Agentic Pipeline](../retrieval/agentic-pipeline.md) - Technical deep-dive
