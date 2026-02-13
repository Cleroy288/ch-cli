# Info Command

The `rustean info` command retrieves detailed documentation and references for any symbol in your codebase. It works **instantly** without requiring LLM generation or a running daemon.

## Usage

```bash
rustean info <SYMBOL> [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `-c, --code` | Show source code snippet |
| `--callers` | Show callers (who calls this symbol) |
| `--callees` | Show callees (what this symbol calls) |
| `-r, --refs` | Show all references grouped by type |
| `-a, --all` | Show everything (code + callers + callees + refs) |

## Output Sections

### Basic Info (always shown)
- **Symbol kind**: fn, struct, enum, trait, impl, etc.
- **File and line**: Location in codebase
- **Documentation**: Doc comments from source (/// or //!)
- **Signature**: Function/method signature if applicable

### Source Code (`--code` or `--all`)
Shows the source code of the symbol with line numbers and a marker (`>`) on the definition line.

### Callers (`--callers` or `--all`)
Lists all locations where this symbol is called. Useful for understanding:
- How widely used a function is
- Where to update when changing a signature
- Entry points for a function

### Callees (`--callees` or `--all`)
Lists functions/methods called within this symbol. Useful for:
- Understanding dependencies
- Tracing execution flow
- Impact analysis

### References (`--refs` or `--all`)
Shows all references grouped by type:
- **Calls**: Function/method calls
- **Type usages**: Where a type is used in signatures/fields
- **Imports**: Use statements
- **Other**: Identifiers in expressions

## Examples

### Basic lookup
```bash
rustean info search_command
```
Output:
```
📖 fn search_command
   File: search.rs:12
   Path: /path/to/src/cli/commands/search.rs

📄 Documentation:
   Execute the `search` command
```

### Show source code
```bash
rustean info IndexManager --code
```

### Find who calls a function
```bash
rustean info search_command --callers
```
Output:
```
📞 Callers (1 call sites):
   1. main.rs:67
```

### Full analysis
```bash
rustean info search_command --all
```
Shows everything: code, callers, callees, and all references.

## How It Works

1. **Indexes the project** with semantic analysis enabled
2. **Finds all definitions** matching the symbol name
3. **Queries the SemanticGraph** for references with context (Call, Type, Import)
4. **Reads source files** to extract code snippets
5. **Displays results** in a structured format

## Comparison with `docs show`

| Feature | `info` | `docs show` |
|---------|--------|-------------|
| Requires daemon | No | Yes |
| Requires LLM generation | No | Yes |
| Shows source code | Yes | No |
| Shows callers/callees | Yes | Yes (if generated) |
| Shows LLM description | No | Yes |
| Speed | ~300ms | Instant (if cached) |

Use `info` for quick lookups during development.
Use `docs show` for rich LLM-generated documentation.

## Related Commands

- `rustean search <query>` - Find symbols by name
- `rustean refs <symbol>` - Find all references (simpler output)
- `rustean goto <symbol>` - Jump to definition
- `rustean docs show <symbol>` - LLM-generated documentation
