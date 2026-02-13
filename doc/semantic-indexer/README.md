# Semantic Code Indexer Documentation

Complete documentation for the semantic code indexing system in rustean.

## Contents

- [Indexer Guide](./indexer.md) - How the indexer works, features, and quick start
- [Architecture](./architecture.md) - System design and architecture
- [Supported Languages](./supported-languages.md) - Language support and detection

## Overview

The rustean semantic indexer is a fast, Rust-native system for:
- **Symbol Search** - Full-text and fuzzy search
- **Go-to-Definition** - Find where symbols are defined
- **Find References** - Locate all usages of a symbol
- **Semantic Analysis** - Track definitions and references across files
- **Parallel Processing** - Index large codebases quickly

Built on:
- **Tree-sitter** - AST parsing
- **Tantivy** - Full-text search engine
- **Rayon** - Parallel processing
