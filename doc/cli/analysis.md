# Analysis Commands

List symbols in your codebase and view indexing statistics.

## Symbols Command

List and filter symbols in your indexed codebase.

### Syntax
```bash
rustean symbols [OPTIONS]
```

### Options
| Option | Description |
|--------|-------------|
| `-f, --file <PATH>` | List symbols in a specific file only |
| `-k, --kind <TYPE>` | Filter by symbol type (function, struct, etc.) |
| `-h, --help` | Print help information |

### Description
Lists all symbols extracted from your codebase with optional filtering by symbol type.

### Usage Examples

```bash
# List all symbols
rustean symbols

# List only functions
rustean symbols -k function

# List only struct definitions
rustean symbols -k struct

# List symbols in a specific file
rustean symbols -f src/main.rs

# List functions in a specific file
rustean symbols -f src/lib.rs -k function

# List enum variants
rustean symbols -k variant
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
rustean symbols --kind function | grep "Visibility: pub"
```

#### Count different symbol types
```bash
rustean symbols | grep "^  " | head -15
```

#### List large types
```bash
rustean symbols --kind struct
```

#### Find macros
```bash
rustean symbols --kind macro
```

---

## Stats Command

Display statistics about your code index.

### Syntax
```bash
rustean stats
```

### Description
Shows comprehensive statistics about your indexed codebase, including file counts, symbol counts, and indexing metadata.

### Output Example

```
Index Statistics for '.':

📊 Index Status:
  Status: Up to date
  Last updated: 2026-01-29 21:43:15 UTC
  Index directory: .rustean-index/

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
rustean stats

# See how many files indexed
rustean stats | grep "Total files"

# Monitor project growth
rustean stats  # Run periodically to track growth

# Verify semantic analysis
rustean stats | grep "Semantic Analysis"
```

---

## Common Workflows

### Exploring Codebase Size
```bash
# Check overall statistics
rustean stats

# Identify dominant symbol types
rustean symbols --kind function --limit 50
rustean symbols --kind struct --limit 50
```

### Finding Code Hotspots
```bash
# List all public functions
rustean symbols --kind function

# Check which are most referenced
rustean refs each_symbol  # manual review
```

### API Inventory
```bash
# List all public modules
rustean symbols --kind module

# List all public traits
rustean symbols --kind trait

# Understand public API surface
```

### Code Metrics
```bash
# Run stats to see current metrics
rustean stats

# Track over time
# (Run periodically and compare)
```

### Refactoring Planning
```bash
# Understand code structure before refactoring
rustean stats

# List symbols that will be affected
rustean symbols --kind function

# Find specific areas
rustean symbols --kind struct --kind enum
```

---

## Prerequisites

Both commands require:
- A built index: `rustean index`
- For semantic stats, use: `rustean index --semantic`

## See Also

- [Search Command](./search.md) - Find specific symbols
- [Navigation Commands](./navigation.md) - Go-to-definition and find references
- [Index Command](./index.md) - Build the index
- [Getting Started](../getting-started/) - First time setup
