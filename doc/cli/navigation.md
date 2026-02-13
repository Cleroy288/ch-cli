# Navigation Commands

Jump to symbol definitions and find all references to symbols.

## Go-to-Definition Command

Jump directly to where a symbol is defined in your codebase.

### Syntax
```bash
rustean goto <SYMBOL>
```

### Arguments
| Argument | Description |
|----------|-------------|
| `<SYMBOL>` | Symbol name to navigate to (required) |

### Description
Finds the definition of a symbol and displays its location. Useful for understanding how symbols are implemented.

### Usage Examples
```bash
# Navigate to struct definition
rustean goto MyStruct

# Navigate to function definition
rustean goto process_data

# Navigate to trait definition
rustean goto Iterator
```

### Output
Shows the definition location:
```
Definition of 'MyStruct':

  File: src/data/structs.rs
  Line: 12
  Column: 7

  pub struct MyStruct {
      field: String,
  }
```

### Requirements
- Requires semantic analysis index: `rustean index --semantic`
- Symbol must be defined in the codebase
- Symbol must have a definition (not external)

---

## Find References Command

Find all places where a symbol is used (referenced) in your codebase.

### Syntax
```bash
rustean refs <SYMBOL> [OPTIONS]
```

### Arguments
| Argument | Description |
|----------|-------------|
| `<SYMBOL>` | Symbol name to find references for (required) |

### Options
| Option | Description |
|--------|-------------|
| `-i, --include-definition` | Also show the definition location |
| `-h, --help` | Print help information |

### Description
Finds all usages of a symbol throughout your codebase. Helps understand how a symbol is used and its impact.

### Usage Examples
```bash
# Find all references to a function
rustean refs my_function

# Include the definition location
rustean refs MyStruct --include-definition

# Find usage of a constant
rustean refs MAX_BUFFER_SIZE

# Find method usages
rustean refs process
```

### Output
Shows all reference locations:
```
References to 'my_function' (5 total):

📍 Definition(s):
  src/lib.rs - line 42

📍 References:
  1. src/app.rs - line 15
     my_function();

  2. src/handlers.rs - line 88
     result = my_function();

  3. src/utils.rs - line 201
     let x = my_function();

  4. src/main.rs - line 5
     use crate::my_function;

  5. src/tests.rs - line 312
     assert_eq!(my_function(), expected);

Total: 1 definition + 5 references = 6 usages
```

### Options in Detail

#### `--include-definition`
```bash
# Show definition along with references
rustean refs my_function --include-definition

# Output includes:
# 📍 Definition(s):
#   src/lib.rs - line 42
# 📍 References:
#   ... (all references listed)
```

Without this flag, only references are shown.

---

## Combined Navigation Workflow

### Explore a Symbol Completely
```bash
# Step 1: Find the symbol
rustean search MyClass --kind struct

# Step 2: Jump to its definition
rustean goto MyClass

# Step 3: See where it's used
rustean refs MyClass --include-definition
```

### Track Feature Usage
```bash
# Find all references to understand impact
rustean refs deprecated_function

# Then navigate to each reference
rustean goto deprecated_function  # see the definition
# Review each reference manually
```

### Refactoring Support
```bash
# Before renaming/removing a function:
rustean refs old_function_name

# Shows all locations that need updating
# Helps ensure nothing is missed during refactoring
```

---

## Common Patterns

### Finding Root Causes
```bash
# Error mentions a function - find where it's defined
rustean goto error_handler

# See all places that call it
rustean refs error_handler
```

### Understanding Code Flow
```bash
# Start with main entry point
rustean goto main

# Follow function calls using goto
rustean goto init_app
rustean goto setup_ui
```

### API Usage Patterns
```bash
# Find all places a public API is used
rustean refs public_api_function --include-definition

# Helps understand how API should be modified
```

---

## Requirements

Both navigation commands require:
- A built semantic index: `rustean index --semantic`
- Semantic analysis enabled during indexing
- The symbol must exist in the indexed codebase

## Prerequisites

Build the index first:
```bash
rustean index --semantic
```

This enables:
- Definition tracking
- Reference resolution
- Scope analysis

---

## Tips & Tricks

### Navigate Multiple Related Symbols
```bash
# For related functionality
rustean goto init_system
rustean refs init_system
rustean goto setup_handlers
rustean refs setup_handlers
```

### Find Unused Code
```bash
# Check references to potential unused code
rustean refs old_utility_function

# If no references found, it might be unused
# (unless used externally)
```

### Trace Dependencies
```bash
# Understand what a type depends on
rustean goto DataProcessor
rustean refs DataProcessor  # see all usages

# Then check what DataProcessor uses
rustean search "impl DataProcessor"
```

### Code Review
```bash
# Before approving a change
rustean refs changed_function

# See all impact points
# Ensures nothing is missed
```

---

## See Also

- [Search Command](./search.md) - Find symbols by name
- [Index Command](./index.md) - Build semantic index
- [Analysis Commands](./analysis.md) - List symbols and statistics
- [Getting Started](../getting-started/) - First time setup
