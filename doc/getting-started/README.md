# Getting Started with ch-cli

A quick start guide to using ch-cli for the first time.

## What is ch-cli?

**ch-cli** is a semantic code indexer and TUI assistant for Rust projects. It provides:

- 🔍 **Fast symbol search** - Find functions, types, and symbols instantly
- 🎯 **Go-to-definition** - Jump to where symbols are defined
- 📍 **Find references** - See all usages of any symbol
- 💬 **Interactive TUI** - Browse files and explore code interactively
- 🚀 **Semantic analysis** - Understand code structure and relationships

## Installation & Setup

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourrepo/ch-cli.git
cd ch-cli

# Build the project
cargo build --release

# The binary is at: target/release/ch-cli
```

### Add to PATH

```bash
# Option 1: Copy to system bin
sudo cp target/release/ch-cli /usr/local/bin/

# Option 2: Create symlink
ln -s $(pwd)/target/release/ch-cli ~/.local/bin/ch-cli

# Then run from anywhere
ch-cli
```

## First Run - Startup Flow

When you run `ch-cli` for the first time:

### 1. Language Detection
```
  ch-cli - Semantic Code Indexer

  No code index found for this project.

  Indexing your codebase enables:
    - Fast symbol search across all files
    - Go-to-definition functionality
    - Find all references to a symbol
    - Semantic code understanding

  Would you like to index your codebase now?

    [Y]es  [N]o  [Q]uit

  Press Y, N, or Q:
```

**Options:**
- `Y` or `Enter` - Build the index (recommended for first run)
- `N` - Skip indexing, start TUI without index
- `Q` or `Esc` - Quit

### 2. Indexing Progress
```
  Indexing codebase...

  ⠏ [████████████████████████░░░░░░░░░░░░░░░░] 60%
  Files: 27/45
  Current: src/indexer/manager.rs
  Elapsed: 1.2s
```

The indexer:
- Scans all source files
- Extracts symbols (functions, structs, traits, etc.)
- Builds search index
- Analyzes definitions and references
- Saves index to `.ch-index/` folder

### 3. Interactive TUI Launches
```
  ╔═══════════════════════════════════════════════════╗
  ║  ch-cli                                           ║
  ║  A semantic code indexer and TUI assistant       ║
  ╚═══════════════════════════════════════════════════╝

  > _

  [File/Folder references shown here]

  Debug Panel (F6):
  • Message parsing status
  • References found
  • Parser state
```

You can now:
- Type text or file references
- Use the file picker
- Press `Enter` to submit messages
- Press `ESC` to exit

## Quick Start Examples

### Example 1: First Index
```bash
# 1. Navigate to your project
cd /path/to/rust/project

# 2. Start ch-cli (answer Y to index)
ch-cli

# This builds the semantic index
```

### Example 2: Search for a Symbol
```bash
# 1. After index is built, search for a function
ch-cli search my_function

# Output shows location:
# 1. my_function → Function (src/lib.rs:42)
```

### Example 3: Navigate to Definition
```bash
# 1. Jump to where a symbol is defined
ch-cli goto MyStruct

# Shows the definition location and code
```

### Example 4: Find All References
```bash
# 1. See all places a symbol is used
ch-cli refs process_data

# Shows all reference locations with context
```

### Example 5: List All Symbols
```bash
# 1. See all structs in your code
ch-cli symbols --kind struct

# Lists all struct definitions
```

## Common Tasks

### Task: Explore Your Codebase
```bash
# Step 1: Build index
ch-cli index --semantic

# Step 2: Check overall statistics
ch-cli stats

# Step 3: List key symbols
ch-cli symbols --kind function --limit 20
ch-cli symbols --kind struct
```

### Task: Find Something Specific
```bash
# Step 1: Search for the symbol
ch-cli search MyType

# Step 2: See where it's defined
ch-cli goto MyType

# Step 3: See where it's used
ch-cli refs MyType
```

### Task: Understand Code Flow
```bash
# Step 1: Start with entry point
ch-cli goto main

# Step 2: See what main calls
ch-cli refs main

# Step 3: Navigate to related functions
ch-cli search process
ch-cli goto process_data
```

### Task: Prepare for Refactoring
```bash
# Step 1: Find the function/type to refactor
ch-cli search old_name

# Step 2: See all references
ch-cli refs old_name --include-definition

# Step 3: Review each location
ch-cli goto old_name
```

## File Structure

After first run, ch-cli creates:

```
.ch-index/
├── index.state       # Metadata and file tracking
└── tantivy/          # Full-text search index
```

### .gitignore
Add to your `.gitignore`:
```
.ch-index/
```

## Available Commands

### Interactive Mode (Default)
```bash
ch-cli          # Launch TUI (default)
ch-cli tui      # Explicit TUI mode
```

### Indexing
```bash
ch-cli index              # Build/update index
ch-cli index --semantic   # With semantic analysis
ch-cli index --verbose    # Show detailed progress
```

### Searching
```bash
ch-cli search MyStruct        # Exact search
ch-cli search my --fuzzy      # Fuzzy search
ch-cli search Thing --kind function  # Filter by type
```

### Navigation
```bash
ch-cli goto MyType           # Jump to definition
ch-cli refs my_function      # Find all references
```

### Analysis
```bash
ch-cli symbols --kind struct # List all structs
ch-cli stats                 # Show statistics
```

## Supported Languages

Currently supported:
- ✅ **Rust** (`.rs` files)

The system detects your project's language and shows a message if unsupported. See [Supported Languages](../semantic-indexer/supported-languages.md) for details.

## What Gets Indexed

The indexer extracts and tracks:
- Functions and methods
- Struct and enum definitions
- Traits and trait implementations
- Constants and static variables
- Type aliases
- Modules
- Macros
- And more...

All with location information and semantic analysis.

## Typical Workflow

```
1. First Run: ch-cli
   ↓ (Answer Y to build index)

2. Search: ch-cli search something
   ↓ (Find what you're looking for)

3. Navigate: ch-cli goto symbol
   ↓ (Jump to definition)

4. Explore: ch-cli refs symbol
   ↓ (See all usages)

5. Interactive: ch-cli
   ↓ (Use TUI for detailed exploration)
```

## Tips for Success

1. **Always start with indexing** - `ch-cli index --semantic` on first run
2. **Use fuzzy search** - When unsure of exact names: `ch-cli search --fuzzy`
3. **Combine commands** - Start with search, then goto, then refs
4. **Keep index updated** - Rebuild when making significant changes
5. **Explore statistics** - `ch-cli stats` shows what's indexed

## Troubleshooting

### Issue: "No index found"
**Solution:** Build the index first
```bash
ch-cli index --semantic
```

### Issue: Symbol not found
**Solutions:**
- Make sure index was built: `ch-cli stats`
- Try exact name spelling: `ch-cli search --fuzzy`
- Symbol might be external (not in this codebase)

### Issue: Search is slow
**Solution:** Use `--limit` to reduce results
```bash
ch-cli search something --limit 10
```

### Issue: Language not supported
**Solution:** See [Supported Languages](../semantic-indexer/supported-languages.md) for details and how to add support

## Next Steps

After first run:
1. Explore commands in [CLI Reference](../cli/)
2. Learn about [Language Detection](../semantic-indexer/supported-languages.md)
3. Check out [Full Documentation](../)
4. Read [Startup Flow](../startup/) to understand interactive startup

## Getting Help

Commands support help:
```bash
ch-cli --help              # Overall help
ch-cli search --help       # Command-specific help
ch-cli goto --help         # Get option details
```

## See Also

- [CLI Commands Reference](../cli/) - Detailed command documentation
- [Semantic Indexer](../semantic-indexer/) - How the indexer works
- [TUI Guide](../tui/) - Interactive interface documentation
- [FAQ](../faq.md) - Frequently asked questions (if available)
