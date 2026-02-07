# Usage & Getting Started

User guides and examples for using ch-cli.

## Contents

- [Usage Guide](./USAGE_GUIDE.md) - Complete guide to all features and commands
- [Visual Demo](./visual-demo.md) - Visual walkthrough of the interface

## Quick Start

```bash
# Run the TUI (interactive mode)
cargo run

# Build the project
cargo build --release

# See available commands
cargo run -- --help
```

## Main Commands

- `index` - Build or rebuild the code index
- `search` - Search for symbols
- `goto` - Jump to symbol definition
- `refs` - Find all references to a symbol
- `stats` - Show index statistics
- `tui` - Launch the interactive interface

For detailed information, see [USAGE_GUIDE.md](./USAGE_GUIDE.md).
