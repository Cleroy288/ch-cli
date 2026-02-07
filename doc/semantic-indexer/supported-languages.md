# Supported Languages

Documentation for language support in the ch-cli semantic code indexer.

## Currently Supported

- **Rust** (`.rs`) - Full support for symbol extraction, definition tracking, and reference finding

## Language Detection System

The indexer includes an intelligent language detection system that:

1. **Scans the codebase** to identify all programming languages present
2. **Detects project type** from configuration files (Cargo.toml, package.json, etc.)
3. **Reports detected languages** and support status to the user
4. **Handles mixed-language projects** gracefully

### Detection Process

When you start ch-cli:
- Analyzes file extensions in your project
- Checks for project configuration files
- Identifies the primary language
- Shows support status before indexing

## Recognized But Not Yet Supported

The system recognizes these languages but indexing is not yet implemented:

- JavaScript (`.js`, `.jsx`, `.mjs`, `.cjs`)
- TypeScript (`.ts`, `.tsx`, `.mts`, `.cts`)
- Python (`.py`, `.pyw`, `.pyi`)
- Go (`.go`)
- Java (`.java`)
- C# (`.cs`)
- C++ (`.cpp`, `.cc`, `.cxx`, `.hpp`, `.hxx`, `.h++`)
- C (`.c`, `.h`)
- Ruby (`.rb`, `.rake`)
- PHP (`.php`)
- Swift (`.swift`)
- Kotlin (`.kt`, `.kts`)

### Adding Support for New Languages

To add indexing support for a new language:

1. **Add language to enum** - `Language` enum in `src/indexer/crawler.rs`
2. **Add file extensions** - Update `Language::from_extension()`
3. **Create parser** - Implement a new parser using tree-sitter for that language
4. **Define queries** - Create tree-sitter queries for symbol extraction
5. **Update manager** - Add case in `IndexManager::parse_file()`
6. **Move from detected to supported** - Update `DetectedLanguage` enum

The language detection system automatically adapts to support new languages without changes to the startup flow.

## User Experience

### Unsupported Language Project
```
  ch-cli - Semantic Code Indexer

  Language Not Supported

  Detected primary language: JavaScript

  Currently supported languages:
    - Rust

  No supported files found in this codebase.
  Semantic indexing will be skipped.

  Press any key to continue to the TUI...
```

### Mixed Language Project (Partial Support)
```
  ch-cli - Semantic Code Indexer

  No code index found for this project.

  Note: Primary language (Python) is not yet supported.
  Will index 15 supported file(s).

  Indexing your codebase enables:
    ...
```

### Supported Language Project
```
  ch-cli - Semantic Code Indexer

  No code index found for this project.

  Indexing your codebase enables:
    - Fast symbol search across all files
    - Go-to-definition functionality
    ...
```
