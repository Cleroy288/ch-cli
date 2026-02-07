# Analysis Commands

List symbols in your codebase and view indexing statistics.

## Symbols Command

List and filter symbols in your indexed codebase.

### Syntax
```bash
ch-cli symbols [OPTIONS]
```

### Options
| Option | Description |
|--------|-------------|
| `--kind <TYPE>` | Filter by symbol type (function, struct, etc.) |
| `--limit <N>` | Maximum results to show (default: all) |
| `-h, --help` | Print help information |

### Description
Lists all symbols extracted from your codebase with optional filtering by symbol type.

### Usage Examples

```bash
# List all symbols
ch-cli symbols

# List only functions
ch-cli symbols --kind function

# List only struct definitions
ch-cli symbols --kind struct

# List first 20 traits
ch-cli symbols --kind trait --limit 20

# List enum variants
ch-cli symbols --kind variant
```

### Available Symbol Types

| Type | Description |
|------|-------------|
| `function` | Function definitions |
| `method` | Methods in impl blocks |
| `struct` | Struct type definitions |
| `enum` | Enum type definitions |
| `trait` | Trait definitions |
| `module` | Module definitions |
| `constant` | Const definitions |
| `static` | Static variable definitions |
| `type_alias` | Type alias definitions |
| `macro` | Macro definitions |
| `variant` | Enum variants |
| `field` | Struct fields |
| `impl` | Impl block definitions |

### Output Format

```
Symbols in codebase (filtered):

Total: 1,234 symbols

By Type:
  Functions: 234
  Structs: 89
  Methods: 456
  Traits: 12
  Enums: 23
  Modules: 15
  Constants: 12
  ...

Functions in codebase:

1. main (src/main.rs:5)
   Visibility: pub

2. process_data (src/lib.rs:42)
   Visibility: pub

3. handle_input (src/app/handlers.rs:88)
   Visibility: crate

...
```

### Common Queries

#### Find all public APIs
```bash
ch-cli symbols --kind function | grep "Visibility: pub"
```

#### Count different symbol types
```bash
ch-cli symbols | grep "^  " | head -15
```

#### List large types
```bash
ch-cli symbols --kind struct
```

#### Find macros
```bash
ch-cli symbols --kind macro
```

---

## Stats Command

Display statistics about your code index.

### Syntax
```bash
ch-cli stats
```

### Description
Shows comprehensive statistics about your indexed codebase, including file counts, symbol counts, and indexing metadata.

### Output Example

```
Index Statistics for '.':

📊 Index Status:
  Status: Up to date
  Last updated: 2026-01-29 21:43:15 UTC
  Index directory: .ch-index/

📁 Files:
  Total files indexed: 45
  Total lines of code: 12,847
  Average file size: 285 lines

📚 Symbols:
  Total symbols: 1,234
  Functions: 234
  Methods: 456
  Structs: 89
  Traits: 12
  Enums: 23
  Modules: 15
  Constants: 12
  Static variables: 8
  Type aliases: 18
  Macros: 5
  Other: 12

🔍 Search Index:
  Index size: 2.3 MB
  Search entries: 1,234
  Last indexed: 2026-01-29 21:43:15 UTC

📈 Performance:
  Indexing time: 1.2 seconds
  Files processed: 45
  Average per file: 26.7 ms

🌐 Languages:
  Rust: 45 files
  Total languages: 1

✅ Semantic Analysis:
  Definitions tracked: 234
  References tracked: 1,450
  Scope analysis: Enabled
```

### What Statistics Mean

#### Files
- **Total files indexed** - Number of source files analyzed
- **Total lines of code** - Combined LOC across all files
- **Average file size** - Mean file size in lines

#### Symbols
- Breakdown of all symbol types found
- Helps understand code structure
- Useful for identifying large modules

#### Search Index
- **Index size** - Disk space used for search index
- **Search entries** - Number of searchable symbols
- **Last indexed** - When index was last built/updated

#### Performance
- **Indexing time** - How long the last index took
- **Files processed** - Number of files analyzed
- **Average per file** - Indexing performance

#### Languages
- Which languages detected in project
- File count per language
- Shows language distribution

#### Semantic Analysis
- **Definitions tracked** - Symbol definitions found
- **References tracked** - Symbol usages found
- **Scope analysis** - Whether semantic features enabled

### Usage Examples

```bash
# Check index status
ch-cli stats

# See how many files indexed
ch-cli stats | grep "Total files"

# Monitor project growth
ch-cli stats  # Run periodically to track growth

# Verify semantic analysis
ch-cli stats | grep "Semantic Analysis"
```

---

## Common Workflows

### Exploring Codebase Size
```bash
# Check overall statistics
ch-cli stats

# Identify dominant symbol types
ch-cli symbols --kind function --limit 50
ch-cli symbols --kind struct --limit 50
```

### Finding Code Hotspots
```bash
# List all public functions
ch-cli symbols --kind function

# Check which are most referenced
ch-cli refs each_symbol  # manual review
```

### API Inventory
```bash
# List all public modules
ch-cli symbols --kind module

# List all public traits
ch-cli symbols --kind trait

# Understand public API surface
```

### Code Metrics
```bash
# Run stats to see current metrics
ch-cli stats

# Track over time
# (Run periodically and compare)
```

### Refactoring Planning
```bash
# Understand code structure before refactoring
ch-cli stats

# List symbols that will be affected
ch-cli symbols --kind function

# Find specific areas
ch-cli symbols --kind struct --kind enum
```

---

## Prerequisites

Both commands require:
- A built index: `ch-cli index`
- For semantic stats, use: `ch-cli index --semantic`

## See Also

- [Search Command](./search.md) - Find specific symbols
- [Navigation Commands](./navigation.md) - Go-to-definition and find references
- [Index Command](./index.md) - Build the index
- [Getting Started](../getting-started/) - First time setup
