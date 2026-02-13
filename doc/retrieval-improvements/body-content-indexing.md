# Body Content Indexing

## Overview

Body content indexing allows BM25 search to match tokens inside function bodies, not just symbol names and signatures.

## Status

**Already Implemented** in `src/indexer/parser/symbol_processing.rs`

## How It Works

```
┌─────────────────────────────────────────────────────────────────┐
│                    Source File                                   │
│                                                                  │
│  fn process_connection(conn: Connection) -> Result<()> {        │
│      let pool = ConnectionPool::new();                          │
│      pool.validate()?;                                          │
│      // Handle connection errors                                 │
│      Ok(())                                                      │
│  }                                                               │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Parser (tree-sitter)                          │
│                                                                  │
│  Extracts:                                                       │
│  - name: "process_connection"                                    │
│  - kind: Function                                                │
│  - signature: "fn process_connection(conn: Connection) -> ..."  │
│  - content: "fn process_connection...pool.validate()...Ok(())"  │ ← Body
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Symbol Struct                                 │
│                                                                  │
│  Symbol {                                                        │
│      name: "process_connection",                                 │
│      kind: Function,                                             │
│      content: Some("fn process_connection..."),  ← Body stored  │
│      ...                                                         │
│  }                                                               │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Tantivy Index                                 │
│                                                                  │
│  Document:                                                       │
│  - symbol_name: "process_connection" (TEXT)                     │
│  - symbol_kind: "Function" (STRING)                              │
│  - content: "fn process_connection..." (TEXT)  ← Searchable     │
└─────────────────────────────────────────────────────────────────┘
```

## Implementation Details

### File: `src/indexer/parser/symbol_processing.rs`

```rust
const MAX_BODY_CONTENT_LEN: usize = 500;

/// Extract body content from a tree-sitter node
fn extract_body_content(source: &str, node: &tree_sitter::Node, max_len: usize) -> Option<String> {
    let start = node.start_byte();
    let end = node.end_byte().min(start + max_len);
    let content = source.get(start..end)?;

    // Truncate at word boundary if content was cut short
    if end < node.end_byte() && content.len() >= max_len {
        if let Some(last_space) = content.rfind(char::is_whitespace) {
            return Some(content[..last_space].to_string() + "...");
        }
    }
    Some(content.to_string())
}
```

### Supported Symbol Types

Body content is extracted for:
- Function
- Method
- Struct
- Enum
- Trait
- Impl
- Macro

### Tantivy Schema

```rust
// src/indexer/search/schema.rs
schema_builder.add_text_field("content", TEXT | STORED);
```

### Document Conversion

```rust
// src/indexer/search/conversion.rs
if let Some(ref content) = symbol.content {
    doc.add_text(fields.content, content);
}
```

## Search Examples

### Query: "connection pool error"

Before body indexing:
- Only matches symbols named "connection", "pool", or "error"

After body indexing:
- Also matches functions that USE ConnectionPool or handle errors internally

```bash
$ rustean search "connection pool error"

Search results for 'connection pool error':

  1. Function handle_connection_error (error_handler.rs:45)
     fn handle_connection_error(pool: &ConnectionPool) -> Result<...>

  2. Method validate_pool (pool.rs:123)
     pub fn validate_pool(&self) -> Result<(), PoolError>
```

## Configuration

### Current Limits

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| MAX_BODY_CONTENT_LEN | 500 chars | Balance between searchability and index size |

### Trade-offs

| Higher Limit | Lower Limit |
|--------------|-------------|
| More searchable content | Smaller index size |
| More false positives | Fewer matches |
| Slower indexing | Faster indexing |

## Performance Impact

| Metric | Without Body | With Body (500 chars) |
|--------|--------------|----------------------|
| Index Size | baseline | +5-10% |
| Index Time | baseline | +2-5% |
| Search Recall | limited | improved |

## Future Improvements

1. **Configurable limit**: Allow users to set MAX_BODY_CONTENT_LEN
2. **Smart truncation**: Truncate at statement boundaries, not character count
3. **Body-only search**: Flag to search only in body content
4. **Different boost for body matches**: Lower boost for body matches vs name matches
