# Search Command

Search for symbols in your indexed codebase by name, with support for keyword, semantic, and hybrid search modes.

## Syntax

```bash
ch-cli search <QUERY> [OPTIONS]
```

## Arguments

| Argument | Description |
|----------|-------------|
| `<QUERY>` | Symbol name or natural language query to search for (required) |

## Options

| Option | Description |
|--------|-------------|
| `-l, --limit <N>` | Maximum results to show (default: 10) |
| `-f, --fuzzy` | Use fuzzy matching (handles typos and partial matches) |
| `-k, --kind <TYPE>` | Filter by symbol type (e.g., function, struct) |
| `--semantic` | Use semantic (hybrid) search combining keywords and embeddings |
| `-c, --context` | Expand results with contextual information (parent, callers, callees) |
| `-r, --rerank` | Rerank results using cross-encoder for better relevance |
| `--full` | Show full file content for results |
| `-h, --help` | Print help information |

## Description

Search for symbols by name in your indexed codebase. Supports multiple search modes from simple keyword matching to advanced semantic search with context expansion.

## Search Modes

### Keyword Search (Default)

Standard BM25-based text search. Fast and precise for exact symbol names.

```bash
ch-cli search handle_input
```

**When to use:**
- You know the exact symbol name
- Quick lookups for specific functions or types
- Performance-critical searches

### Fuzzy Search

Tolerates typos and partial matches using edit distance (up to 2 character differences).

```bash
ch-cli search MyStructt --fuzzy
```

**When to use:**
- Unsure of exact spelling
- Exploring similar symbols
- Handling typos in queries

### Semantic Search (Hybrid)

Combines keyword matching (BM25) with neural embeddings for natural language queries. Uses Reciprocal Rank Fusion (RRF) to merge results from both methods.

```bash
ch-cli search "function that handles user input" --semantic
```

**When to use:**
- Natural language queries describing what you need
- Finding conceptually related code
- When exact names are unknown
- Understanding code intent rather than names

**How it works:**
1. Generates embeddings for your query via the ML daemon
2. Runs parallel keyword and semantic searches
3. Merges results using RRF scoring
4. Returns results ranked by combined relevance

## Usage Examples

### Basic Search

```bash
# Search for exact symbol name
ch-cli search MyStruct

# Search with more results
ch-cli search my_function --limit 20
```

### Fuzzy Search

```bash
# Fuzzy search (handles typos)
ch-cli search MyStructt --fuzzy

# Fuzzy search with limited results
ch-cli search processs --fuzzy --limit 5
```

### Semantic Search

```bash
# Natural language query
ch-cli search "error handling logic" --semantic

# Find related code by concept
ch-cli search "database connection pooling" --semantic

# Combine with limit for top results
ch-cli search "authentication middleware" --semantic --limit 5
```

### Context Expansion

The `-c, --context` flag expands results with related code: parent modules, functions that call the result, and functions the result calls.

```bash
# Get full context for symbol matches
ch-cli search handle_request --semantic --context

# Understand call chains
ch-cli search process_data --context
```

**Output includes:**
- Parent module/struct containing the symbol
- Functions that call this symbol (callers)
- Functions this symbol calls (callees)
- Formatted as XML for LLM consumption

**Note:** Context expansion requires semantic analysis. Run `ch-cli index --semantic` first.

### Reranking

The `-r, --rerank` flag uses a cross-encoder model to re-score results for better relevance.

```bash
# Rerank for better precision
ch-cli search "parse configuration" --semantic --rerank

# Combine reranking with context
ch-cli search validate_input --semantic --rerank --context
```

**How it works:**
1. Fetches 3x the requested limit as candidates
2. Sends query + candidates to cross-encoder model
3. Re-scores based on query-document relevance
4. Returns top results by rerank score

**When to use:**
- Need highest precision results
- Complex queries where initial ranking may miss relevance
- Quality over speed scenarios

### Full Content Display

The `--full` flag shows the actual file content around each result.

```bash
# View code context for matches
ch-cli search my_function --full

# Combine with semantic search
ch-cli search "error handler" --semantic --full
```

**Output shows:**
- 5 lines before the symbol
- Up to 200 lines of content
- Target line highlighted with `>`
- Line numbers for reference

### Filter by Type

```bash
# Find all structs named "Config"
ch-cli search Config --kind struct

# Find all functions with "handle" in the name
ch-cli search handle --kind function

# Find all traits
ch-cli search Trait --kind trait
```

### Combined Options

```bash
# Full semantic pipeline: hybrid search + rerank + context
ch-cli search "user authentication" --semantic --rerank --context

# Fuzzy search for functions with full content
ch-cli search handl --fuzzy --kind function --full

# Semantic search with type filter
ch-cli search "data validation" --semantic --kind function --limit 20
```

## Symbol Types

Available types for `--kind` filter:

| Type | Aliases | Examples |
|------|---------|----------|
| `function` | `fn`, `func` | Function definitions |
| `method` | - | Methods in impl blocks |
| `struct` | - | Struct types |
| `enum` | - | Enum definitions |
| `trait` | - | Trait definitions |
| `module` | `mod` | Module definitions |
| `constant` | `const` | Const definitions |
| `static` | - | Static variables |
| `type_alias` | `type`, `typealias` | Type alias definitions |
| `macro` | - | Macro definitions |
| `variant` | - | Enum variants |
| `field` | - | Struct fields |
| `impl` | - | Impl block definitions |

## Search Results

### Keyword Search Output

Each result shows:
- **Symbol name** - The name of the symbol
- **Type** - Symbol kind (function, struct, etc.)
- **Location** - File path and line number
- **Signature** - Function signature if available

Example output:
```
Search results for 'handle':

  1. Function handle_input (input.rs:12)
     fn handle_input(event: Event) -> Result<()>

  2. Method handle_key (mod.rs:42)
     fn handle_key(&mut self, key: Key)

  3. Function handle_event (events.rs:156)
     fn handle_event(e: &Event) -> bool
```

### Semantic Search Output

Semantic results include additional scoring information:

```
Hybrid search results for 'user authentication':

  1. Function authenticate_user (auth.rs:45) [RRF:0.0312 K:1 S:3]
     fn authenticate_user(creds: &Credentials) -> Result<User>

  2. Method verify_token (session.rs:78) [RRF:0.0298 K:2 S:1]
     fn verify_token(&self, token: &str) -> bool

  3. Struct AuthManager (manager.rs:12) [RRF:0.0245 K:5 S:2]
```

**Score breakdown:**
- `RRF` - Combined Reciprocal Rank Fusion score
- `K:N` - Keyword search rank (or `-` if not in keyword results)
- `S:N` - Semantic search rank (or `-` if not in semantic results)
- `R:N` - Rerank score (shown when `--rerank` is used)

### Context-Expanded Output

With `--context`, output is formatted as XML for LLM consumption:

```xml
<context-results query="handle_request">
  <symbol name="handle_request" kind="Function" file="src/server.rs" line="45">
    <code>fn handle_request(req: Request) -> Response { ... }</code>
    <parent name="server" kind="Module"/>
    <callers>
      <caller name="main_loop" file="src/main.rs" line="23"/>
    </callers>
    <callees>
      <callee name="parse_body" file="src/parser.rs" line="12"/>
      <callee name="send_response" file="src/server.rs" line="89"/>
    </callees>
  </symbol>
</context-results>
```

## Tips

### Finding Related Symbols

```bash
# Find all handler functions
ch-cli search handle --fuzzy --kind function

# Find all Config structs
ch-cli search Config --kind struct
```

### Understanding Code Flow

```bash
# See what calls a function and what it calls
ch-cli search process_request --semantic --context

# Full investigation: semantic + rerank + context
ch-cli search "main entry point" --semantic --rerank --context
```

### Natural Language Exploration

```bash
# Describe what you're looking for
ch-cli search "functions that validate user input" --semantic

# Find error handling patterns
ch-cli search "error handling and recovery" --semantic --kind function

# Locate configuration-related code
ch-cli search "parse configuration from file" --semantic
```

### Debugging and Investigation

```bash
# Quick lookup with full code context
ch-cli search suspicious_function --full

# Deep investigation with all flags
ch-cli search my_function --semantic --rerank --context --full
```

### Performance Optimization

```bash
# Fast exact search when you know the name
ch-cli search exact_symbol_name

# Limit results for faster response
ch-cli search "complex query" --semantic --limit 5

# Skip reranking for speed
ch-cli search "quick lookup" --semantic
```

### LLM Integration

```bash
# Generate context for LLM prompts
ch-cli search "relevant code" --semantic --context > context.xml

# Structured output for parsing
ch-cli search authenticate --semantic --context
```

## Performance Notes

| Mode | Speed | Quality | Use Case |
|------|-------|---------|----------|
| Keyword (default) | Fast | Good for exact names | Quick lookups |
| Fuzzy | Fast | Good for typos | Exploration |
| Semantic | Medium | Best for concepts | Natural language |
| Semantic + Rerank | Slower | Highest precision | Critical searches |
| + Context | Adds overhead | Adds relationships | Understanding code flow |

- **Keyword search** is fastest, use when you know the exact name
- **Semantic search** requires the ML daemon running
- **Reranking** adds latency but improves precision
- **Context expansion** requires semantic analysis index
- Limiting results with `--limit` improves performance

## Prerequisites

### For Basic Search

Build the index:
```bash
ch-cli index
```

### For Semantic Search

1. Build index with semantic analysis:
```bash
ch-cli index --semantic
```

2. Generate embeddings:
```bash
ch-cli embed
```

3. Start the ML daemon (if not running):
```bash
ch-cli daemon start
```

### For Context Expansion

Requires semantic analysis enabled:
```bash
ch-cli index --semantic
```

## Common Workflows

### 1. Quick Symbol Lookup

```bash
# Fast exact search
ch-cli search my_function
```

### 2. Explore Unknown Code

```bash
# Describe what you need
ch-cli search "file parsing utilities" --semantic --limit 10

# View promising results
ch-cli search parse_file --full
```

### 3. Understand Call Relationships

```bash
# See callers and callees
ch-cli search handle_request --semantic --context
```

### 4. Find Best Matches

```bash
# Full pipeline for best results
ch-cli search "authentication logic" --semantic --rerank --context
```

### 5. Code Review Preparation

```bash
# Get all context around a change
ch-cli search modified_function --semantic --context --full
```

## See Also

- [Go-to-Definition](./navigation.md#go-to-definition) - Jump to symbol definition
- [Find References](./navigation.md#find-references) - Find all symbol usages
- [Index Command](./index.md) - How to build the index
- [Retrieve Command](./retrieve.md) - Full agentic retrieval pipeline
- [Getting Started](../getting-started/) - First time setup
