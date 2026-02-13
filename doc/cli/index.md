# Index Command

Build, update, and manage semantic code indexes for your project.

## Syntax

```bash
rustean index [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `-p, --path <PATH>` | Path to project root (default: `.`) |
| `-s, --semantic` | Enable semantic analysis for definitions and references |
| `-v, --verbose` | Show detailed progress output |
| `-h, --help` | Print help information |

## Description

The `index` command builds a semantic code index of your project. This enables:
- **Fast symbol search** across all files
- **Go-to-definition** functionality
- **Find all references** to symbols
- **Semantic analysis** for intelligent name resolution

## Usage Examples

### Basic Indexing
```bash
# Index current project
rustean index

# Index specific directory
rustean index -p /path/to/project
```

### Semantic Analysis
```bash
# Index with semantic analysis (recommended)
rustean index -s

# Full semantic analysis with verbose output
rustean index -s -v
```

### Monitoring Progress
```bash
# See detailed progress of indexing
rustean index -v

# With semantic analysis and progress
rustean index -s -v
```

## What Gets Indexed

The indexer analyzes your codebase and extracts:

### Supported Symbol Types
- Functions and methods
- Structs and enums
- Traits and trait implementations
- Constants and static variables
- Type aliases and modules
- Macros and enum variants
- Struct and enum fields

### Supported Languages
Currently: **Rust** (`.rs` files)

See [Supported Languages](../semantic-indexer/supported-languages.md) for details on multi-language support and detection.

## Language Detection

When you run `rustean index`:
1. The system automatically detects your project's primary language
2. If your language isn't supported, a message will be shown
3. Mixed-language projects will index only supported files

See [Language Detection](../semantic-indexer/supported-languages.md) for more details.

## Index Storage

Indexes are stored in `.rustean-index/` directory at your project root:
```
.rustean-index/
├── state.json         # Index metadata and file tracking
├── refs.json          # Persisted symbol references (per-file invalidation)
├── trigrams.json      # Trigram index for fuzzy search
└── tantivy/           # Full-text search index (BM25)
```

### .gitignore
Add to `.gitignore` to exclude from version control:
```
.rustean-index/
```

## Incremental Indexing

On subsequent runs, the indexer automatically:
1. Detects which files have changed
2. Re-indexes only modified files
3. Updates the search index incrementally
4. Preserves unchanged file information

This makes re-indexing much faster after small changes.

## Performance

### Typical Indexing Times
- Small projects (< 100 files): < 1 second
- Medium projects (100-1000 files): 1-5 seconds
- Large projects (> 1000 files): 5-30 seconds

### Performance Factors
- **File count** - More files = longer indexing
- **File size** - Larger files take longer to parse
- **Semantic analysis** - Enabled with `--semantic` flag
- **System resources** - Uses parallel processing (rayon)

## Semantic Analysis

Use `--semantic` flag to enable:
- **Definition tracking** - Know where each symbol is defined
- **Reference resolution** - Find all usages of a symbol
- **Scope analysis** - Understand symbol visibility and scoping

Recommended for most use cases.

## Troubleshooting

### Index is out of date
```bash
# Rebuild the entire index
rustean index --semantic
```

### Slow indexing
```bash
# Check if problem files exist
rustean index --verbose

# The verbose output shows what's being processed
```

### Language not supported
```
Language Not Supported

Detected primary language: JavaScript

Currently supported languages:
  - Rust
```

If your language isn't supported, see [Supported Languages](../semantic-indexer/supported-languages.md) for information on adding support.

## See Also

- [Search Command](./search.md) - How to search the index
- [Navigation Commands](./navigation.md) - Go-to-definition and find references
- [Getting Started](../getting-started/) - First time setup
- [Startup Flow](../startup/) - Interactive startup behavior
