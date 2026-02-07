# CLI Commands Reference

Complete reference for all ch-cli command-line interface commands for the semantic code indexer.

## Contents

- [Index Command](./index.md) - Build and manage code indexes
- [Embed Command](./embed.md) - Generate embeddings for semantic search
- [Search Command](./search.md) - Search for symbols by name
- [Retrieve Command](./retrieve.md) - Agentic retrieval with natural language queries
- [Navigation Commands](./navigation.md) - Go-to-definition and find references
- [Analysis Commands](./analysis.md) - List symbols and view statistics
- [Docs Command](./docs.md) - Manage LLM-generated documentation
- [Daemon Command](./daemon.md) - Manage the ML model daemon

## Quick Reference

```bash
# Indexing
ch-cli index              # Interactive indexing with startup flow
ch-cli index --path .     # Index current directory
ch-cli index --semantic   # Enable semantic analysis
ch-cli index --verbose    # Show detailed progress

# Embedding (for semantic search)
ch-cli embed              # Generate embeddings for indexed symbols
ch-cli embed --force      # Force re-embedding all symbols
ch-cli embed --path .     # Embed specific project

# Searching
ch-cli search MyStruct    # Search for symbol
ch-cli search --fuzzy foo # Fuzzy search (handles typos)
ch-cli search --kind function # Filter by symbol type
ch-cli search --semantic "parse input" # Semantic search (requires embed)

# Agentic Retrieval (natural language queries)
ch-cli retrieve "how does auth work"  # Full pipeline with context
ch-cli retrieve "error handling" --xml # XML output for LLMs
ch-cli retrieve "config" --structured  # Separate code/doc/notes

# Navigation
ch-cli goto MyClass       # Jump to symbol definition
ch-cli refs my_function   # Find all references to symbol

# Analysis
ch-cli symbols --kind struct # List all structs
ch-cli stats              # Show indexing statistics

# Daemon
ch-cli daemon start       # Start ML model daemon
ch-cli daemon stop        # Stop the daemon
ch-cli daemon status      # Check daemon status
ch-cli daemon restart     # Restart the daemon

# Documentation (LLM-generated)
ch-cli docs generate      # Generate docs for all symbols
ch-cli docs status        # Check generation progress
ch-cli docs show MyStruct # View docs for a symbol
ch-cli docs search "auth" # Search through docs

# Interactive
ch-cli tui                # Launch interactive TUI
ch-cli                    # Same as 'tui' (default)
```

## Command Categories

### Indexing Commands
- **`index`** - Build or update the project index
- **`embed`** - Generate embeddings for semantic search

### Search Commands
- **`search`** - Find symbols by name with optional fuzzy matching
- **`retrieve`** - Agentic retrieval with natural language (LLM-powered pipeline)

### Navigation Commands
- **`goto`** - Jump to where a symbol is defined
- **`refs`** - Find all usages/references to a symbol

### Analysis Commands
- **`symbols`** - List symbols with filtering options
- **`stats`** - Show index statistics and metadata

### Daemon Commands
- **`daemon start`** - Start the ML model daemon
- **`daemon stop`** - Stop the running daemon
- **`daemon status`** - Show daemon status and loaded models
- **`daemon restart`** - Restart the daemon

### Interface
- **`tui`** - Launch the interactive terminal UI (default)

### Documentation Commands
- **`docs generate`** - Generate LLM-powered documentation for all symbols
- **`docs status`** - Show documentation generation progress
- **`docs show`** - Display documentation for a specific symbol
- **`docs search`** - Search through generated documentation

## Default Behavior

Running `ch-cli` without arguments or with `ch-cli tui` launches:
1. Startup flow (language detection + optional indexing)
2. Interactive terminal UI for file/folder selection
3. Message parsing and conversation interface

## Symbol Types

When filtering with `--kind`, use one of:
- `function` - Function definitions
- `method` - Methods in impl blocks
- `struct` - Struct types
- `enum` - Enum types
- `trait` - Trait definitions
- `module` - Module definitions
- `constant` - Const definitions
- `static` - Static variables
- `type_alias` - Type aliases
- `macro` - Macro definitions
- `variant` - Enum variants
- `field` - Struct fields
- `impl` - Impl blocks

## Common Patterns

### Index and Search
```bash
# Create or update index with semantic analysis
ch-cli index --semantic

# Search for a function
ch-cli search my_function

# Find all places it's used
ch-cli refs my_function

# Navigate to the definition
ch-cli goto my_function
```

### Semantic Search Workflow
```bash
# 1. Index the project
ch-cli index --semantic

# 2. Start the daemon (auto-starts if needed)
ch-cli daemon start

# 3. Generate embeddings
ch-cli embed

# 4. Search by meaning
ch-cli search --semantic "parse user input"
```

### Explore Codebase
```bash
# List all functions in codebase
ch-cli symbols --kind function

# List all structs
ch-cli symbols --kind struct

# See index statistics
ch-cli stats
```

### Development Workflow
```bash
# Initial setup - index the project
ch-cli index --semantic

# Quick search while coding
ch-cli search SomeType
ch-cli refs some_function

# Check what changed
ch-cli stats

# Launch interactive mode for exploration
ch-cli
```

## Typical Workflows

### Basic Workflow: Index, Search, Navigate

Standard workflow for exploring a new codebase:

```bash
# 1. Index the project
ch-cli index --semantic

# 2. Search for what you need
ch-cli search AuthHandler
ch-cli search --kind function login

# 3. Navigate to the code
ch-cli goto AuthHandler           # Jump to definition
ch-cli refs handle_login          # Find all usages
```

### Semantic Workflow: Embeddings-Powered Search

Enhanced search using vector embeddings for better relevance:

```bash
# 1. Index the project
ch-cli index --semantic

# 2. Start the ML daemon (loads embedding models)
ch-cli daemon start
# Wait a few seconds for models to load
ch-cli daemon status

# 3. Generate embeddings for all symbols
ch-cli embed

# 4. Use semantic search
ch-cli search --semantic "user authentication"
ch-cli search --semantic --rerank "database connection pooling"
```

### Documentation Workflow: LLM-Generated Docs

Generate comprehensive documentation using LLM analysis:

```bash
# 1. Index the project
ch-cli index --semantic

# 2. Start the daemon
ch-cli daemon start

# 3. Generate documentation (runs in background)
ch-cli docs generate

# 4. Check progress
ch-cli docs status

# 5. View and search documentation
ch-cli docs show MyStruct
ch-cli docs search "error handling"
```

### Agentic Retrieval: Natural Language Queries

Use natural language to find relevant code context for LLM assistants:

```bash
# 1. Start the daemon (required)
ch-cli daemon start

# 2. Query with natural language
ch-cli retrieve "how does the authentication system work"

# 3. Get XML output for LLM consumption
ch-cli retrieve --xml "explain the database layer"

# 4. Get structured output (code/doc/notes separated)
ch-cli retrieve --structured "find payment processing logic"

# 5. Fine-tune retrieval
ch-cli retrieve --limit 15 --no-expand "specific_function_name"
```

## Advanced Commands Reference

### Embed Command Options

Generate vector embeddings for all symbols in the index. Requires the daemon to be running.

```bash
ch-cli embed                    # Embed symbols in current directory
ch-cli embed --path ./myproject # Embed specific project
ch-cli embed --force            # Re-embed all symbols (removes existing)
```

**Process:**
1. Indexes the project with semantic analysis
2. Connects to the daemon
3. Generates embeddings in batches of 32
4. Stores embeddings for semantic search

### Retrieve Command Options

The agentic retrieval pipeline for natural language code queries. Uses query expansion, semantic search, and reranking.

```bash
ch-cli retrieve "how does authentication work"
ch-cli retrieve --limit 20 "find all database connections"
ch-cli retrieve --xml "query"              # Raw XML output for LLM consumption
ch-cli retrieve --structured "query"       # Separate code/doc/notes sections
ch-cli retrieve --max-tokens 4000 "query"  # Limit output tokens
```

**Options:**
| Flag | Description |
|------|-------------|
| `--limit N` | Maximum number of results (default: 10) |
| `--max-tokens N` | Maximum tokens in output (default: 8000) |
| `--no-expand` | Disable LLM query expansion |
| `--no-rerank` | Disable cross-encoder reranking |
| `--no-context` | Disable context expansion |
| `--xml` | Output raw XML for LLM consumption |
| `--structured` | Separate code, doc, notes sections |
| `--threshold F` | RRF score threshold (default: 0.015) |
| `--min-results N` | Min results per content type (default: 1) |

### Docs Command Options

Generate and manage LLM-powered documentation for symbols.

```bash
ch-cli docs generate             # Start background doc generation
ch-cli docs generate --force     # Regenerate all docs
ch-cli docs status               # Show progress (completed/pending)
ch-cli docs show my_function     # Display docs for a symbol
ch-cli docs search "query"       # Search documentation
ch-cli docs search --limit 20 q  # More search results
```

**Docs show output includes:**
- Symbol kind and location
- Function signature
- User comments (from source)
- LLM-generated documentation
- Dependencies (depends on / used by)
- External crates used

### Search Command Additional Options

The search command supports advanced options for semantic and context-aware search:

```bash
ch-cli search --semantic "query"    # Use hybrid search (keywords + embeddings)
ch-cli search --context "query"     # Include parent, callers, callees
ch-cli search --rerank "query"      # Rerank using cross-encoder
ch-cli search --full "query"        # Show full file content in results
```

## See Also

- [Getting Started](../getting-started/) - First-time setup guide
- [Startup Flow](../startup/) - Understanding the interactive startup
- [Supported Languages](../semantic-indexer/supported-languages.md) - Which languages are supported
