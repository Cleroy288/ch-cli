# Context Expansion

## Summary

Context expansion transforms raw search results into rich contextual blocks by traversing the SemanticGraph to gather parent scope, related types, callers, and callees. The output is formatted as XML for LLM consumption.

## How It Works

```
                    Search Results
                         │
                         ▼
              ┌──────────────────────┐
              │    BlockBuilder      │
              │  • extract_code()    │
              │  • format snippets   │
              └──────────┬───────────┘
                         │
                         ▼
              ┌──────────────────────┐
              │    GraphWalker       │
              │  • find_parent()     │
              │  • find_callers()    │
              │  • find_callees()    │
              │  • find_related()    │
              └──────────┬───────────┘
                         │
                         ▼
              ┌──────────────────────┐
              │  ContextualBlock     │
              │  • symbol + code     │
              │  • parent context    │
              │  • related types     │
              │  • call graph        │
              └──────────┬───────────┘
                         │
                         ▼
              ┌──────────────────────┐
              │   ContextExpander    │
              │  • token budgeting   │
              │  • XML formatting    │
              └──────────┬───────────┘
                         │
                         ▼
                   XML Output
              (ready for LLM prompt)
```

## CLI Usage

```bash
# Semantic search with context expansion
ch-cli search --semantic --context "authentication"

# The --context flag adds:
# - Parent scope (class, module, impl block)
# - Callers (who calls this symbol)
# - Callees (what this symbol calls)
# - Related types (traits, parameters, return types)
# - Full code snippets with line numbers
```

## Output Format

The context expander produces XML suitable for LLM prompts:

```xml
<context>
<context-block>
  <symbol kind="Function" name="authenticate" file="src/auth.rs" line="45">
    <doc>Authenticates a user with the given credentials</doc>
    <code>
  42 | /// Authenticates a user with the given credentials
  43 | /// Returns a token on success, error on failure
  44 | pub fn authenticate(user: &User, password: &str) -> Result<Token> {
  45 |     let hash = hash_password(password);
  46 |     if verify_hash(&user.password_hash, &hash) {
  47 |         Ok(Token::new(user.id))
  48 |     } else {
  49 |         Err(AuthError::InvalidPassword)
  50 |     }
  51 | }
    </code>
  </symbol>
  <parent kind="Impl" name="AuthService" file="src/auth.rs" line="10"/>
  <related-types>
    <type name="User" relationship="ParameterType"/>
    <type name="Token" relationship="ReturnType"/>
    <type name="AuthError" relationship="ReturnType"/>
  </related-types>
  <callers>
    <caller name="login_handler" kind="Function" file="src/routes.rs" line="78"/>
    <caller name="api_authenticate" kind="Function" file="src/api.rs" line="122"/>
  </callers>
  <callees>
    <callee name="hash_password"/>
    <callee name="verify_hash"/>
    <callee name="Token::new"/>
  </callees>
</context-block>
</context>
```

## Architecture

### Source Files

| File | Purpose |
|------|---------|
| `context/mod.rs` | Data structures (ContextualBlock, ParentContext, etc.) |
| `context/graph_walker.rs` | SemanticGraph traversal |
| `context/block_builder.rs` | Code extraction and block construction |

### Key Types

```rust
/// A contextual block containing a symbol with surrounding context
pub struct ContextualBlock {
    pub symbol: Symbol,
    pub code_snippet: String,
    pub parent: Option<ParentContext>,
    pub related_types: Vec<RelatedType>,
    pub callers: Vec<CallerInfo>,
    pub callees: Vec<CalleeInfo>,
    pub doc_comment: Option<String>,
}

/// Configuration for context expansion
pub struct ContextConfig {
    pub max_callers: usize,        // default: 5
    pub max_callees: usize,        // default: 5
    pub context_lines_before: usize, // default: 3
    pub context_lines_after: usize,  // default: 10
    pub include_parent: bool,      // default: true
    pub include_related_types: bool, // default: true
}
```

### BlockBuilder

Extracts code snippets from source files:

```rust
let builder = BlockBuilder::new(&semantic_graph);
let block = builder.build(&symbol)?;
// block.code_snippet contains formatted code with line numbers
```

### GraphWalker

Traverses the SemanticGraph for context:

```rust
let walker = GraphWalker::new(&graph, config);

// Find parent scope
let parent = walker.find_parent(&symbol);

// Find who calls this symbol
let callers = walker.find_callers(&symbol);

// Find what this symbol calls
let callees = walker.find_callees(&symbol);

// Find related types from signature
let types = walker.find_related_types(&symbol);
```

### ContextExpander

Orchestrates expansion with token budgeting:

```rust
let expander = ContextExpander::with_config(&graph, config, 8000);
let xml = expander.expand_to_xml(&symbols);
// Returns XML string ready for LLM prompt
```

## Token Budgeting

The ContextExpander enforces a token budget to prevent overwhelming LLM context windows:

1. Processes symbols in relevance order
2. Estimates tokens per block (~4 chars per token)
3. Stops adding blocks when budget is exhausted
4. Default budget: 10,000 tokens

```rust
// Custom token limit
let expander = ContextExpander::with_config(&graph, config, 4000);
```

## Symbol End Detection

The block builder intelligently detects symbol boundaries:

1. **Brace matching** - Counts `{` and `}` to find closing brace
2. **Fallback estimates** - Uses symbol kind to estimate length:
   - Function/Method: 20 lines
   - Struct/Enum: 15 lines
   - Impl: 30 lines
   - Trait: 25 lines
   - Other: 5 lines

## Type Relationship Detection

Related types are categorized by their relationship to the symbol:

| Relationship | Description | Example |
|--------------|-------------|---------|
| `Implements` | Trait implemented | `impl Display for Foo` |
| `ParameterType` | Function parameter | `fn foo(x: Config)` |
| `ReturnType` | Function return | `fn foo() -> Result<T>` |
| `UsedInSignature` | Other signature use | Generic bounds |
| `UsedInBody` | Used in function body | Local variables |
| `FieldType` | Struct field type | `struct Foo { bar: Bar }` |

## Integration with Search

Context expansion integrates with hybrid search:

```bash
# Without context - just symbol names and scores
ch-cli search --semantic "authentication"

# With context - full contextual blocks
ch-cli search --semantic --context "authentication"
```

## Performance

| Operation | Time |
|-----------|------|
| Code extraction | ~1ms per symbol |
| Graph traversal | ~0.5ms per symbol |
| XML formatting | ~0.1ms per block |
| Total (10 symbols) | ~20ms |

## Limitations

1. **File I/O** - Each block reads the source file
2. **Heuristic end detection** - Brace counting may fail on complex code
3. **50-line callee scope** - Callees detected within 50 lines of symbol start

## Future Improvements

1. **Caching** - Cache file contents for repeated access
2. **Syntax-aware extraction** - Use tree-sitter for precise boundaries
3. **Cross-file context** - Include related symbols from other files
4. **Incremental updates** - Update context when files change
