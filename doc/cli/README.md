# CLI Commands Reference

Complete reference for all rustean command-line interface commands for the semantic code indexer.

## Contents

- [Index Command](./index.md) - Build and manage code indexes
- [Embed Command](./embed.md) - Generate embeddings for semantic search
- [Search Command](./search.md) - Search for symbols by name
- [Retrieve Command](./retrieve.md) - Agentic retrieval with natural language queries
- [Navigation Commands](./navigation.md) - Go-to-definition and find references
- [Info Command](./info.md) - Detailed symbol info (code, callers, callees, refs)
- [Analysis Commands](./analysis.md) - List symbols and view statistics
- [Docs Command](./docs.md) - Manage LLM-generated documentation
- [Daemon Command](./daemon.md) - Manage the ML model daemon

## Quick Reference

```bash
# Indexing
rustean index                 # Interactive indexing with startup flow
rustean index -p .            # Index current directory
rustean index -s              # Enable semantic analysis
rustean index -s -v           # Semantic + verbose progress

# Embedding (for semantic search)
rustean embed                 # Generate embeddings for indexed symbols
rustean embed -f              # Force re-embedding all symbols
rustean embed -p ./myproject  # Embed specific project

# Searching
rustean search MyStruct              # Search for symbol
rustean search -f foo                # Fuzzy search (handles typos)
rustean search -k function handle    # Filter by symbol type
rustean search --semantic "parse"    # Semantic search (requires embed)
rustean search --semantic -r "auth"  # Semantic + rerank
rustean search -c handle_request     # Context expansion (callers/callees)
rustean search --full my_function    # Show file content around matches

# Agentic Retrieval (natural language queries)
rustean retrieve "how does auth work"  # Full pipeline with context
rustean retrieve "error handling" --xml # XML output for LLMs
rustean retrieve "config" --structured  # Separate code/doc/notes

# Navigation
rustean goto MyClass              # Jump to symbol definition
rustean refs my_function          # Find all references to symbol
rustean refs MyStruct -i          # Include definition location

# Symbol Info (instant, no daemon needed)
rustean info MyStruct             # Basic info + doc comments
rustean info my_function -c       # Show source code
rustean info my_function --callers # Who calls this?
rustean info my_function --callees # What does this call?
rustean info my_function -r       # All references grouped by type
rustean info my_function -a       # Everything (code+callers+callees+refs)

# Analysis
rustean symbols -k struct         # List all structs
rustean symbols -f src/main.rs    # List symbols in a specific file
rustean stats                     # Show indexing statistics

# Daemon
rustean daemon start       # Start ML model daemon
rustean daemon stop        # Stop the daemon
rustean daemon status      # Check daemon status
rustean daemon restart     # Restart the daemon

# Documentation (LLM-generated)
rustean docs generate      # Generate docs for all symbols
rustean docs generate -f   # Force regenerate all
rustean docs status        # Check generation progress
rustean docs show MyStruct # View docs for a symbol
rustean docs search "auth" # Search through docs

# Interactive
rustean tui                # Launch interactive TUI
rustean                    # Same as 'tui' (default)
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

### Inspection Commands
- **`info`** - Detailed symbol info: code, callers, callees, refs (no daemon needed)

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

Running `rustean` without arguments or with `rustean tui` launches:
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
rustean index --semantic

# Search for a function
rustean search my_function

# Find all places it's used
rustean refs my_function

# Navigate to the definition
rustean goto my_function
```

### Semantic Search Workflow
```bash
# 1. Index the project
rustean index --semantic

# 2. Start the daemon (auto-starts if needed)
rustean daemon start

# 3. Generate embeddings
rustean embed

# 4. Search by meaning
rustean search --semantic "parse user input"
```

### Explore Codebase
```bash
# List all functions in codebase
rustean symbols --kind function

# List all structs
rustean symbols --kind struct

# See index statistics
rustean stats
```

### Development Workflow
```bash
# Initial setup - index the project
rustean index --semantic

# Quick search while coding
rustean search SomeType
rustean refs some_function

# Check what changed
rustean stats

# Launch interactive mode for exploration
rustean
```

## Typical Workflows

### Basic Workflow: Index, Search, Navigate

Standard workflow for exploring a new codebase:

```bash
# 1. Index the project
rustean index --semantic

# 2. Search for what you need
rustean search AuthHandler
rustean search --kind function login

# 3. Navigate to the code
rustean goto AuthHandler           # Jump to definition
rustean refs handle_login          # Find all usages
```

### Semantic Workflow: Embeddings-Powered Search

Enhanced search using vector embeddings for better relevance:

```bash
# 1. Index the project
rustean index --semantic

# 2. Start the ML daemon (loads embedding models)
rustean daemon start
# Wait a few seconds for models to load
rustean daemon status

# 3. Generate embeddings for all symbols
rustean embed

# 4. Use semantic search
rustean search --semantic "user authentication"
rustean search --semantic --rerank "database connection pooling"
```

### Documentation Workflow: LLM-Generated Docs

Generate comprehensive documentation using LLM analysis:

```bash
# 1. Index the project
rustean index --semantic

# 2. Start the daemon
rustean daemon start

# 3. Generate documentation (runs in background)
rustean docs generate

# 4. Check progress
rustean docs status

# 5. View and search documentation
rustean docs show MyStruct
rustean docs search "error handling"
```

### Agentic Retrieval: Natural Language Queries

Use natural language to find relevant code context for LLM assistants:

```bash
# 1. Start the daemon (required)
rustean daemon start

# 2. Query with natural language
rustean retrieve "how does the authentication system work"

# 3. Get XML output for LLM consumption
rustean retrieve --xml "explain the database layer"

# 4. Get structured output (code/doc/notes separated)
rustean retrieve --structured "find payment processing logic"

# 5. Fine-tune retrieval
rustean retrieve --limit 15 --no-expand "specific_function_name"
```

## Advanced Commands Reference

### Embed Command Options

Generate vector embeddings for all symbols in the index. Requires the daemon to be running.

```bash
rustean embed                    # Embed symbols in current directory
rustean embed --path ./myproject # Embed specific project
rustean embed --force            # Re-embed all symbols (removes existing)
```

**Process:**
1. Indexes the project with semantic analysis
2. Connects to the daemon
3. Generates embeddings in batches of 32
4. Stores embeddings for semantic search

### Retrieve Command Options

The agentic retrieval pipeline for natural language code queries. Uses query expansion, semantic search, and reranking.

```bash
rustean retrieve "how does authentication work"
rustean retrieve --limit 20 "find all database connections"
rustean retrieve --xml "query"              # Raw XML output for LLM consumption
rustean retrieve --structured "query"       # Separate code/doc/notes sections
rustean retrieve --max-tokens 4000 "query"  # Limit output tokens
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
rustean docs generate             # Start background doc generation
rustean docs generate --force     # Regenerate all docs
rustean docs status               # Show progress (completed/pending)
rustean docs show my_function     # Display docs for a symbol
rustean docs search "query"       # Search documentation
rustean docs search --limit 20 q  # More search results
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
rustean search --semantic "query"    # Use hybrid search (keywords + embeddings)
rustean search --context "query"     # Include parent, callers, callees
rustean search --rerank "query"      # Rerank using cross-encoder
rustean search --full "query"        # Show full file content in results
```

## See Also

- [Getting Started](../getting-started/) - First-time setup guide
- [Startup Flow](../startup/) - Understanding the interactive startup
- [Supported Languages](../semantic-indexer/supported-languages.md) - Which languages are supported
