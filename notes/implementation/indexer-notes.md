# Semantic Code Indexer - Development Notes

## Overview

This document describes how the semantic code indexer for `ch-cli` was designed and implemented. The indexer provides fast symbol search, "go to definition", and reference finding capabilities for Rust codebases.

## Architecture

The indexer is built in 5 phases, each adding a layer of functionality:

```
┌─────────────────────────────────────────────────────────────┐
│                    Phase 5: CLI Commands                     │
│         (index, search, goto, refs, symbols, stats)          │
├─────────────────────────────────────────────────────────────┤
│                  Phase 4: Semantic Analysis                  │
│            (SemanticGraph, definitions, references)          │
├─────────────────────────────────────────────────────────────┤
│                Phase 3: Tantivy Search Engine                │
│           (SearchIndex, full-text search, fuzzy)             │
├─────────────────────────────────────────────────────────────┤
│                 Phase 2: File System Crawling                │
│         (Crawler, parallel processing, .gitignore)           │
├─────────────────────────────────────────────────────────────┤
│               Phase 1: Tree-sitter Integration               │
│            (RustParser, Symbol extraction, queries)          │
└─────────────────────────────────────────────────────────────┘
```

## Module Structure

```
src/indexer/
├── mod.rs          # Module exports and documentation
├── symbols.rs      # Core data types (Symbol, SymbolKind, CodeLocation)
├── queries.rs      # Tree-sitter S-expression queries for Rust
├── parser.rs       # RustParser - parses files and extracts symbols
├── crawler.rs      # File discovery with .gitignore support
├── search.rs       # Tantivy search index wrapper
├── semantic.rs     # SemanticGraph for definition/reference tracking
└── manager.rs      # IndexManager - high-level orchestration

src/cli/
├── mod.rs          # CLI definition with clap
└── commands.rs     # Command implementations
```

## Phase 1: Tree-sitter Integration

### Goal
Parse Rust source files and extract symbols (functions, structs, enums, etc.)

### Key Dependencies
- `tree-sitter = "0.24"` - Incremental parsing library
- `tree-sitter-rust = "0.23"` - Rust grammar
- `streaming-iterator = "0.1"` - Required for query API

### Implementation Details

**Symbol Types Extracted:**
- Functions, Methods, Structs, Enums, Traits
- Impl blocks, Constants, Statics, Type aliases
- Modules, Macros, Enum variants, Struct fields

**Tree-sitter Queries** (`queries.rs`):
We use S-expression queries to match AST nodes. Example:
```scheme
(function_item
  name: (identifier) @name
  parameters: (parameters) @params
  return_type: (_)? @return_type) @function
```

**RustParser** (`parser.rs`):
- Creates a tree-sitter parser with Rust language
- Parses source code into AST
- Runs queries to extract symbols
- Handles visibility (pub, pub(crate), private)

## Phase 2: File System Crawling

### Goal
Discover and process all Rust files in a project, respecting .gitignore

### Key Dependencies
- `rayon = "1.10"` - Parallel data processing
- `ignore = "0.4"` - .gitignore support with WalkBuilder

### Implementation Details

**Crawler** (`crawler.rs`):
- Uses `ignore::WalkBuilder` for file discovery
- Automatically respects `.gitignore`, `.git/`, `target/`
- Configurable max file size, symlink following
- Language detection by file extension

**IndexManager** (`manager.rs`):
- High-level API for indexing projects
- Uses `rayon::par_iter()` for parallel file parsing
- Progress callback support for UI feedback
- Collects results and errors from all files

**Performance:**
- 41 files indexed in ~80ms
- Parallel processing scales with CPU cores

## Phase 3: Tantivy Search Engine

### Goal
Enable fast full-text search over indexed symbols

### Key Dependencies
- `tantivy = "0.22"` - High-performance search engine

### Implementation Details

**Search Schema:**
```
symbol_name  : TEXT (tokenized, stored)
symbol_kind  : STRING (stored, indexed)
file_path    : STRING (stored)
line, column : U64 (stored, indexed)
visibility   : STRING (stored)
signature    : TEXT (stored, optional)
fqn          : TEXT (stored) - fully qualified name
parent       : STRING (stored, optional)
```

**SearchIndex** (`search.rs`):
- In-memory or persistent index storage
- `search(query, limit)` - Full-text search with ranking
- `fuzzy_search(term, distance, limit)` - Handles typos
- `search_by_kind(kind, limit)` - Filter by symbol type

**Performance:**
- 747 symbols indexed in ~20ms
- Search returns results in ~540µs

## Phase 4: Semantic Analysis

### Goal
Track symbol definitions and references for "go to definition" and "find references"

### Key Dependencies
- `fxhash = "0.2"` - Fast hashing for efficient lookups

### Design Decision
We initially considered using `stack-graphs` for full semantic analysis, but opted for a simpler FxHashMap-based approach due to:
- Lower complexity
- Faster implementation
- Sufficient for basic definition/reference tracking

### Implementation Details

**SemanticGraph** (`semantic.rs`):
- `definitions: FxHashMap<String, Vec<Definition>>` - Symbol name → definitions
- `references: FxHashMap<String, Vec<SymbolReference>>` - Symbol name → references
- `by_file: FxHashMap<PathBuf, Vec<Definition>>` - File → definitions
- `by_kind: FxHashMap<SymbolKind, Vec<Definition>>` - Kind → definitions

**Key Methods:**
- `find_definitions(name)` - Find where a symbol is defined
- `find_references(name)` - Find where a symbol is used
- `find_all_usages(name)` - Get all definitions + references
- `resolve(reference)` - Resolve a reference to its definition
- `definition_at(file, line)` - "Go to definition" at cursor

**Fully Qualified Names (FQN):**
Symbols are tracked with FQN like `module::struct::method` for disambiguation.

## Phase 5: CLI Commands

### Goal
Expose indexer functionality through command-line interface

### Key Dependencies
- `clap = { version = "4.5", features = ["derive"] }` - CLI parsing

### Commands Implemented

| Command | Description |
|---------|-------------|
| `index` | Build/rebuild the code index |
| `search <query>` | Search for symbols |
| `goto <symbol>` | Jump to symbol definition |
| `refs <symbol>` | Find all references |
| `symbols` | List symbols (with filters) |
| `stats` | Show index statistics |
| `tui` | Launch interactive TUI (default) |

### Usage Examples
```bash
ch-cli index --semantic          # Index with semantic analysis
ch-cli search IndexManager       # Search for symbols
ch-cli search --fuzzy proc       # Fuzzy search
ch-cli goto App                  # Find definition
ch-cli symbols --kind struct     # List all structs
ch-cli stats                     # Show statistics
```

## Testing Strategy

### Unit Tests (40 total)
- **Parser tests** (9): Symbol extraction for each type
- **Semantic tests** (17): Definition/reference tracking
- **Manager tests** (7): Integration with temp directories
- **Search tests** (3): Full-text and fuzzy search
- **App tests** (4): Input parsing

### Integration Testing
- Created temp projects with known symbols
- Verified symbol counts and types
- Tested edge cases (empty files, syntax errors)

## Performance Characteristics

| Metric | Value |
|--------|-------|
| Files indexed | 41 |
| Symbols found | 747 |
| Unique symbols | 426 |
| Index time | ~80ms |
| Search time | ~540µs |
| Memory | In-memory index |

## Future Improvements (Phase 6+)

1. **Persistent Index** - Save index to disk, incremental updates
2. **File Watcher** - Auto-reindex on file changes
3. **More Languages** - TypeScript, Python, Go support
4. **Reference Extraction** - Parse AST for actual references
5. **Cross-file Resolution** - Resolve imports and use statements
6. **LSP Integration** - Language Server Protocol support

## Lessons Learned

1. **Tree-sitter queries are powerful** - S-expressions can match complex patterns
2. **Parallel processing matters** - rayon made indexing 4x faster
3. **Tantivy is fast** - Sub-millisecond search on 700+ symbols
4. **Start simple** - FxHashMap worked better than stack-graphs for MVP
5. **Test with real code** - Indexing the project itself found edge cases

