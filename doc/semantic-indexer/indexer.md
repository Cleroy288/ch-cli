# Semantic Code Indexer

A fast, Rust-native code indexer built into `rustean` for symbol search and navigation.

## Features

- **Fast Symbol Search** - Full-text and fuzzy search in sub-millisecond
- **Go to Definition** - Find where any symbol is defined
- **Find References** - Locate all usages of a symbol
- **Semantic Analysis** - Track definitions and references across files
- **Parallel Processing** - Indexes large codebases quickly using rayon

## Quick Start

```bash
# Index the current project
rustean index --semantic

# Search for symbols
rustean search MyStruct

# Fuzzy search (handles typos)
rustean search --fuzzy procs

# Go to definition
rustean goto App

# Find all references
rustean refs --include-definition handle_key

# List all structs
rustean symbols --kind struct

# Show statistics
rustean stats
```

## How It Works

The indexer is built in 5 layers:

1. **Tree-sitter** - Parses Rust source into AST, extracts symbols
2. **Crawler** - Discovers files, respects .gitignore, parallel processing
3. **Tantivy** - Full-text search engine for fast symbol lookup
4. **SemanticGraph** - Tracks definitions and references
5. **CLI** - User-friendly commands via clap

## Supported Symbol Types

| Symbol | Example |
|--------|---------|
| Function | `fn main()` |
| Method | `impl Foo { fn bar() }` |
| Struct | `struct MyStruct` |
| Enum | `enum Status` |
| Trait | `trait Iterator` |
| Impl | `impl Foo for Bar` |
| Constant | `const MAX: i32` |
| Static | `static COUNTER: i32` |
| Module | `mod utils` |
| Macro | `macro_rules! my_macro` |

## Performance

| Metric | Value |
|--------|-------|
| Index 41 files | ~80ms |
| 747 symbols | ~20ms to search index |
| Search query | ~540µs |

## Architecture

```
src/indexer/
├── symbols.rs    # Symbol, SymbolKind, CodeLocation
├── queries.rs    # Tree-sitter queries for Rust
├── parser.rs     # RustParser - AST parsing
├── crawler.rs    # File discovery with .gitignore
├── search.rs     # Tantivy search wrapper
├── semantic.rs   # SemanticGraph for definitions
└── manager.rs    # IndexManager orchestration

src/cli/
├── mod.rs        # CLI definition (clap)
└── commands.rs   # Command implementations
```

## Dependencies

- `tree-sitter` + `tree-sitter-rust` - Parsing
- `tantivy` - Search engine
- `rayon` - Parallel processing
- `ignore` - .gitignore support
- `fxhash` - Fast hashing
- `clap` - CLI parsing

## Development

See `notes/INDEXER_DEVELOPMENT.md` for detailed implementation notes.

```bash
# Run tests
cargo test indexer

# Run all tests
cargo test

# Build release
cargo build --release
```

## Future Plans

- Persistent index storage
- File watcher for auto-reindex
- More language support (TypeScript, Python)
- LSP integration

