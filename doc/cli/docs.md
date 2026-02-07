# Docs Command

Manage LLM-generated documentation for code symbols in your indexed codebase.

## Summary

The `docs` command provides tools to generate, view, and search AI-generated documentation for all symbols in your codebase. Using a local LLM (Phi-3), it creates detailed descriptions for functions, structs, enums, traits, and other code elements.

## Syntax

```bash
ch-cli docs <COMMAND>
```

## Subcommands

| Command | Description |
|---------|-------------|
| `generate` | Generate documentation for all symbols |
| `status` | Show documentation generation status |
| `show` | Show documentation for a specific symbol |
| `search` | Search through generated documentation |

---

## Concept: LLM-Generated Documentation

### What is LLM-Generated Documentation?

LLM-generated documentation uses a local language model (Phi-3) to automatically create descriptions for code symbols. The system:

1. **Analyzes code context** - Reads the source code, signature, and structure
2. **Considers user comments** - Preserves and incorporates existing `///` and `//!` doc comments
3. **Generates descriptions** - Creates 2-4 sentence technical descriptions
4. **Tracks cross-references** - Maps dependencies between symbols

### What Gets Documented?

Documentation is generated for all symbol types:
- Functions and methods
- Structs and enums
- Traits and trait implementations
- Constants and static variables
- Type aliases and modules
- Macros and impl blocks

### Documentation Entry Contents

Each documentation entry includes:
- **Symbol name and kind** - What the symbol is
- **Location** - File path and line number
- **Signature** - Function/method signature (if applicable)
- **User comment** - Existing doc comments from source code
- **LLM documentation** - AI-generated description
- **Dependencies** - What this symbol depends on
- **Dependents** - What depends on this symbol
- **External crates** - External dependencies used

---

## Generate Subcommand

Start background documentation generation for all symbols.

### Syntax

```bash
ch-cli docs generate [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `-f, --force` | Force regeneration even if docs already exist |
| `-h, --help` | Print help information |

### Description

Initiates documentation generation in the background via the daemon. The LLM processes each symbol and generates technical descriptions. Progress can be monitored with `ch-cli docs status`.

### Usage Examples

```bash
# Generate docs for all symbols
ch-cli docs generate

# Force regenerate all docs (even existing ones)
ch-cli docs generate --force
```

### Output

```
Starting documentation generation...
Documentation generation started in background.
Use 'ch-cli docs status' to check progress.
```

### Notes

- Generation runs in background via the daemon
- Large codebases may take several minutes
- Incremental: only processes symbols without existing docs (unless `--force`)

---

## Status Subcommand

Display the current documentation generation progress.

### Syntax

```bash
ch-cli docs status
```

### Description

Shows how many symbols have been documented, how many are pending, and displays a progress bar.

### Usage Examples

```bash
# Check generation progress
ch-cli docs status
```

### Output Example

```
Documentation Status:

  Total entries:    234
  Completed:        156
  Pending:          78

  Progress: [????????????????????????????????] 66.7%

  Generation in progress...
```

When complete:

```
Documentation Status:

  Total entries:    234
  Completed:        234
  Pending:          0

  Progress: [????????????????????????????????????????] 100.0%

  Documentation is ready!
```

### Status States

| Status | Meaning |
|--------|---------|
| Pending | Not yet processed |
| Generating | LLM is currently generating |
| Ready | Generation complete |
| Failed | Generation encountered an error |

---

## Show Subcommand

Display documentation for a specific symbol.

### Syntax

```bash
ch-cli docs show <SYMBOL>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `<SYMBOL>` | Symbol name to look up (required) |

### Description

Retrieves and displays all documentation for a named symbol, including user comments, LLM-generated docs, and cross-references.

### Usage Examples

```bash
# Show documentation for a function
ch-cli docs show parse_config

# Show documentation for a struct
ch-cli docs show SearchIndex

# Show documentation for a method
ch-cli docs show HybridSearchEngine::search
```

### Output Example

```
Documentation for 'parse_config'

  Kind:     Function
  File:     src/config/parser.rs:42

  Signature:
    pub fn parse_config(path: &Path) -> Result<Config, ConfigError>

  User Comment:
    Parses a configuration file from the given path.
    Returns an error if the file is invalid or missing.

  Generated Documentation:
    Reads and parses a configuration file from disk, validating its
    structure and returning a strongly-typed Config struct. Handles
    TOML format with support for environment variable expansion.
    Returns ConfigError::NotFound if path doesn't exist, or
    ConfigError::Parse for malformed content.

  Depends On:
    - Config
    - ConfigError
    - read_file

  Used By:
    - main
    - reload_config

  External Crates:
    - toml
    - serde

  Status: ready
```

### When Symbol Not Found

```
No documentation found for 'unknown_symbol'
Run 'ch-cli docs generate' to generate docs.
```

---

## Search Subcommand

Search through generated documentation.

### Syntax

```bash
ch-cli docs search <QUERY> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `<QUERY>` | Search query (required) |

### Options

| Option | Description |
|--------|-------------|
| `-l, --limit <N>` | Maximum results to show (default: 10) |
| `-h, --help` | Print help information |

### Description

Searches through all generated documentation for symbols matching the query. Searches symbol names, documentation content, and cross-references.

### Usage Examples

```bash
# Search for authentication-related docs
ch-cli docs search "authentication"

# Search with more results
ch-cli docs search "parser" --limit 20

# Search for error handling patterns
ch-cli docs search "error handling"
```

### Output Example

```
Documentation search results for 'parser':

  1. Function parse_config (src/config/parser.rs:42)
     Reads and parses a configuration file from disk, validating its...

  2. Struct Parser (src/parser/mod.rs:15)
     Main parser state machine that processes input tokens and builds...

  3. Method Parser::parse (src/parser/mod.rs:89)
     Consumes the input stream and produces an abstract syntax tree....

  4. Function parse_args (src/cli/args.rs:12)
     Parses command-line arguments using clap, returning a validated...
```

---

## Prerequisites

### Required Setup

Before using `docs` commands, ensure:

1. **Index exists** - Run `ch-cli index --semantic` first
2. **Embeddings generated** - Run `ch-cli embed` for semantic search
3. **Daemon is running** - Start with `ch-cli daemon start`

### Verify Prerequisites

```bash
# Check if index exists
ch-cli stats

# Check daemon status
ch-cli daemon status
```

### Error Messages

If daemon not running:
```
Error starting doc generation: connection refused
Make sure the daemon is running: ch-cli daemon start
```

If no index:
```
No documentation found.
Run 'ch-cli docs generate' to start.
```

---

## Typical Workflow

### Complete Setup Workflow

```bash
# Step 1: Build the semantic index
ch-cli index --semantic

# Step 2: Generate embeddings for semantic search
ch-cli embed

# Step 3: Start the background daemon
ch-cli daemon start

# Step 4: Generate documentation
ch-cli docs generate

# Step 5: Monitor progress
ch-cli docs status

# Step 6: View specific symbol docs
ch-cli docs show MyStruct

# Step 7: Search documentation
ch-cli docs search "configuration"
```

### Quick Reference After Setup

```bash
# Check if docs are ready
ch-cli docs status

# Look up a symbol
ch-cli docs show function_name

# Search for concepts
ch-cli docs search "error handling"
```

### Regenerating After Code Changes

```bash
# Re-index changed files
ch-cli index --semantic

# Regenerate embeddings
ch-cli embed

# Force regenerate all docs
ch-cli docs generate --force

# Or just generate for new symbols
ch-cli docs generate
```

---

## Architecture

### How It Works

```
Index (symbols) ──> DocStore ──> DocGenerator (LLM) ──> docs.json
                         │
                         └──> DaemonClient ──> Background Processing
```

1. **DocStore** loads symbols from the index
2. **DocGenerator** uses Phi-3 LLM to generate descriptions
3. **Background daemon** processes symbols asynchronously
4. Results stored in `.ch-index/docs.json`

### Storage

Documentation is stored in:
```
.ch-index/
├── index.state      # Index metadata
├── tantivy/         # Search index
├── embeddings.bin   # Vector embeddings
└── docs.json        # Generated documentation
```

---

## Tips

### Efficient Usage

```bash
# Generate docs once, then use show/search frequently
ch-cli docs generate
ch-cli docs show symbol_name  # Fast lookup
ch-cli docs search "query"    # Fast search
```

### Understanding Code Flow

```bash
# Find a symbol
ch-cli docs search "handler"

# View its documentation and dependencies
ch-cli docs show handle_request

# Follow the dependency chain
ch-cli docs show AuthMiddleware
```

### Finding Related Symbols

```bash
# Search by concept
ch-cli docs search "database connection"

# View docs to see what each depends on
ch-cli docs show DatabasePool
```

---

## Troubleshooting

### Daemon Not Responding

```bash
# Check if daemon is running
ch-cli daemon status

# Restart daemon if needed
ch-cli daemon stop
ch-cli daemon start
```

### Documentation Not Generating

```bash
# Ensure index exists and is up to date
ch-cli index --semantic

# Check generation status
ch-cli docs status

# Force regeneration
ch-cli docs generate --force
```

### Symbol Not Found

```bash
# Verify symbol exists in index
ch-cli search symbol_name

# Check exact name (case-sensitive)
ch-cli symbols --kind function | grep -i symbol_name

# Regenerate docs if symbol is new
ch-cli docs generate
```

### Slow Generation

- Large codebases take time (LLM processes each symbol)
- Generation runs in background - continue using other commands
- Check progress with `ch-cli docs status`

---

## See Also

- [Index Command](./index.md) - Building the code index
- [Search Command](./search.md) - Searching for symbols
- [Navigation Commands](./navigation.md) - Go-to-definition and find references
- [Analysis Commands](./analysis.md) - List symbols and view statistics
- [Getting Started](../getting-started/) - First time setup
