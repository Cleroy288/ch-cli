# Getting Started with rustean

A quick start guide to using rustean for the first time.

## What is rustean?

**rustean** is a semantic code indexer and TUI assistant for Rust projects. It provides:

- 🔍 **Fast symbol search** - Find functions, types, and symbols instantly
- 🎯 **Go-to-definition** - Jump to where symbols are defined
- 📍 **Find references** - See all usages of any symbol
- 💬 **Interactive TUI** - Browse files and explore code interactively
- 🚀 **Semantic analysis** - Understand code structure and relationships

## Installation & Setup

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourrepo/rustean.git
cd rustean

# Build the project
cargo build --release

# The binary is at: target/release/rustean
```

### Add to PATH

```bash
# Option 1: Copy to system bin
sudo cp target/release/rustean /usr/local/bin/

# Option 2: Create symlink
ln -s $(pwd)/target/release/rustean ~/.local/bin/rustean

# Then run from anywhere
rustean
```

## First Run - Startup Flow

When you run `rustean` for the first time:

### 1. Language Detection
```
  rustean - Semantic Code Indexer

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
- Saves index to `.rustean-index/` folder

### 3. Interactive TUI Launches
```
  ╔═══════════════════════════════════════════════════╗
  ║  rustean                                           ║
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

# 2. Start rustean (answer Y to index)
rustean

# This builds the semantic index
```

### Example 2: Search for a Symbol
```bash
# 1. After index is built, search for a function
rustean search my_function

# Output shows location:
# 1. my_function → Function (src/lib.rs:42)
```

### Example 3: Navigate to Definition
```bash
# 1. Jump to where a symbol is defined
rustean goto MyStruct

# Shows the definition location and code
```

### Example 4: Find All References
```bash
# 1. See all places a symbol is used
rustean refs process_data

# Shows all reference locations with context
```

### Example 5: List All Symbols
```bash
# 1. See all structs in your code
rustean symbols --kind struct

# Lists all struct definitions
```

## Common Tasks

### Task: Explore Your Codebase
```bash
# Step 1: Build index
rustean index --semantic

# Step 2: Check overall statistics
rustean stats

# Step 3: List key symbols
rustean symbols --kind function --limit 20
rustean symbols --kind struct
```

### Task: Find Something Specific
```bash
# Step 1: Search for the symbol
rustean search MyType

# Step 2: See where it's defined
rustean goto MyType

# Step 3: See where it's used
rustean refs MyType
```

### Task: Understand Code Flow
```bash
# Step 1: Start with entry point
rustean goto main

# Step 2: See what main calls
rustean refs main

# Step 3: Navigate to related functions
rustean search process
rustean goto process_data
```

### Task: Prepare for Refactoring
```bash
# Step 1: Find the function/type to refactor
rustean search old_name

# Step 2: See all references
rustean refs old_name --include-definition

# Step 3: Review each location
rustean goto old_name
```

## File Structure

After first run, rustean creates:

```
.rustean-index/
├── index.state       # Metadata and file tracking
└── tantivy/          # Full-text search index
```

### .gitignore
Add to your `.gitignore`:
```
.rustean-index/
```

## Available Commands

### Interactive Mode (Default)
```bash
rustean          # Launch TUI (default)
rustean tui      # Explicit TUI mode
```

### Indexing
```bash
rustean index              # Build/update index
rustean index --semantic   # With semantic analysis
rustean index --verbose    # Show detailed progress
```

### Searching
```bash
rustean search MyStruct        # Exact search
rustean search my --fuzzy      # Fuzzy search
rustean search Thing --kind function  # Filter by type
```

### Navigation
```bash
rustean goto MyType           # Jump to definition
rustean refs my_function      # Find all references
```

### Analysis
```bash
rustean symbols --kind struct # List all structs
rustean stats                 # Show statistics
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
1. First Run: rustean
   ↓ (Answer Y to build index)

2. Search: rustean search something
   ↓ (Find what you're looking for)

3. Navigate: rustean goto symbol
   ↓ (Jump to definition)

4. Explore: rustean refs symbol
   ↓ (See all usages)

5. Interactive: rustean
   ↓ (Use TUI for detailed exploration)
```

## Tips for Success

1. **Always start with indexing** - `rustean index --semantic` on first run
2. **Use fuzzy search** - When unsure of exact names: `rustean search --fuzzy`
3. **Combine commands** - Start with search, then goto, then refs
4. **Keep index updated** - Rebuild when making significant changes
5. **Explore statistics** - `rustean stats` shows what's indexed

## Troubleshooting

### Issue: "No index found"
**Solution:** Build the index first
```bash
rustean index --semantic
```

### Issue: Symbol not found
**Solutions:**
- Make sure index was built: `rustean stats`
- Try exact name spelling: `rustean search --fuzzy`
- Symbol might be external (not in this codebase)

### Issue: Search is slow
**Solution:** Use `--limit` to reduce results
```bash
rustean search something --limit 10
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
rustean --help              # Overall help
rustean search --help       # Command-specific help
rustean goto --help         # Get option details
```

## See Also

- [CLI Commands Reference](../cli/) - Detailed command documentation
- [Semantic Indexer](../semantic-indexer/) - How the indexer works
- [TUI Guide](../tui/) - Interactive interface documentation
- [FAQ](../faq.md) - Frequently asked questions (if available)
