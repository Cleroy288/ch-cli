# Startup Flow

Complete documentation of the rustean startup sequence and user interactions.

## Overview

When you run `rustean` without arguments (or `rustean tui`), the startup flow:
1. **Analyzes your codebase** to detect programming languages
2. **Checks for an existing index** and detects changes
3. **Prompts you for actions** (build, update, skip)
4. **Launches the interactive TUI**

## Startup Sequence

```
┌─────────────────────────────────────┐
│  User runs: rustean                  │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  1. Analyze Codebase                │
│     Detect languages                │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  2. Check Language Support          │
│     Is primary language supported?  │
└─────────────────────────────────────┘
           ↓
      ┌────┴────┐
      │          │
   YES│          │NO
      ↓          ↓
   ┌──┴──┐    ┌──────────────────────┐
   │     │    │ Show "Not Supported" │
   │     │    │ Press key to continue│
   │     │    └──────────────────────┘
   │     │              ↓
   ↓     └──────────────┬────────────────┐
┌──────────────────────┘                │
│  3. Check Index Status                │
│     Does .rustean-index/ exist?            │
└──────────────────────┬────────────────┘
           ┌───────────┴──────────┐
           │                      │
        YES│                      │NO
           ↓                      ↓
    ┌────────────┐      ┌──────────────┐
    │ Index      │      │ No Index     │
    │ Exists     │      │ Exists       │
    └────┬───────┘      └──────┬───────┘
         ↓                     ↓
    ┌────────────┐      ┌──────────────────┐
    │ Detect     │      │ Prompt User:     │
    │ Changes    │      │ Index now? Y/N/Q │
    └────┬───────┘      └──────┬───────────┘
         ↓                     │
    ┌────────────┐        ┌────┴──────────────┐
    │ Has        │        │                   │
    │ Changes?   │        │   ┌───┬───┬───┐   │
    └────┬───────┘        │   │ Y │ N │ Q │   │
         │                └───┴─┬─┴─┬─┴───┘   │
    ┌────┴────┐             ┌──┴┐ │ └──┐     │
    │          │             │  │ │    │     │
   YES│       NO│          Index│ Skip Quit  │
    ↓  ↓       ↓             ↓  │  ↓  ↓    │
    │  │    ┌──────┐         │  │  │  │    │
    │  └───→│ Index│         │  │  │  │    │
    │       │ Up to│         │  │  │  │    │
    │       │ Date │         │  │  │  │    │
    │       └──────┘         │  │  │  │    │
    │         ↓              │  │  │  │    │
    └→ Prompt │              │  │  │  │    │
      for    └──→ Update ←───┘  │  │  │    │
      Update     Index          │  │  │    │
        │          ↓            │  │  │    │
        └──────────┬────────────┘  │  │    │
                   ↓               │  │    │
              ┌─────────┐          │  │    │
              │ Indexing│          │  │    │
              │ Progress│          │  │    │
              │ Display │          │  │    │
              └────┬────┘          │  │    │
                   ↓               │  │    │
              ┌─────────┐          │  │    │
              │ Index   │          │  │    │
              │ Complete│          │  │    │
              └────┬────┘          │  │    │
                   │               │  │    │
                   └───────────────┬──┴────┘
                                   ↓
                          ┌──────────────────┐
                          │ 4. Launch TUI    │
                          │                  │
                          │ User can now:    │
                          │ - Type messages  │
                          │ - Pick files     │
                          │ - Search index   │
                          └──────────────────┘
```

## Detailed Steps

### Step 1: Language Detection

**What happens:**
- Scans your codebase for files by extension
- Identifies programming languages present
- Determines the primary language (most files)
- Checks if primary language is supported

**Code in:** `src/startup.rs::analyze_codebase()`
**Implementation:** `src/indexer/analyzer.rs::CodebaseAnalyzer`

**Result:**
- Primary language identified
- Support status determined
- File counts calculated

### Step 2: Language Support Check

#### Supported Language (Rust)
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

User chooses:
- `Y` → Proceed to build index
- `N` → Skip indexing, continue to TUI
- `Q` → Quit application

#### Unsupported Language (JavaScript)
```
  rustean - Semantic Code Indexer

  Language Not Supported

  Detected primary language: JavaScript

  Currently supported languages:
    - Rust

  No supported files found in this codebase.
  Semantic indexing will be skipped.

  Press any key to continue to the TUI...
```

Then:
- Continues to TUI automatically
- Indexing skipped
- Other features still available

#### Mixed Languages (Python + Rust)
```
  rustean - Semantic Code Indexer

  No code index found for this project.

  Note: Primary language (Python) is not yet supported.
  Will index 15 supported file(s).

  Indexing your codebase enables:
    - Fast symbol search across all files
    - Go-to-definition functionality
    - Find all references to a symbol
    - Semantic code understanding

  Would you like to index your codebase now?

    [Y]es  [N]o  [Q]uit
```

User can:
- Index the Rust files only
- Skip indexing
- Quit

### Step 3: Index Management

#### No Index Exists (First Time)
```
User sees → Prompt to build index (as shown above)
```

#### Index Exists + No Changes
```
System detects: Index is up to date
Action: Skip prompting, proceed to TUI
```

#### Index Exists + Changes Detected
```
  rustean - Semantic Code Indexer

  Changes detected in your codebase!

    + 3 new file(s)
    ~ 2 modified file(s)
    - 1 deleted file(s)

  Would you like to update your index?

    [Y]es  [N]o  [Q]uit

  Press Y, N, or Q:
```

User chooses:
- `Y` → Update index incrementally
- `N` → Keep old index, continue to TUI
- `Q` → Quit

### Step 4: Indexing Progress

When user selects `Y` to index/update:

```
  Indexing codebase...

  ⠏ [████████████████████████░░░░░░░░░░░░░░░░] 60%
  Files: 27/45
  Current: src/indexer/manager.rs
  Elapsed: 1.2s
```

**Displayed:**
- Animated spinner (rotating)
- Progress bar (filled percentage)
- Completion percentage
- Current file count
- Current file being processed
- Elapsed time

**Processing:**
- Scans all source files
- Extracts symbols using tree-sitter
- Builds search index with tantivy
- Performs semantic analysis (if enabled)
- Saves to `.rustean-index/`

### Step 5: TUI Launch

After indexing (or skip), launches interactive interface:

```
  ╔═══════════════════════════════════════════════════╗
  ║  rustean                                           ║
  ║  A semantic code indexer and TUI assistant       ║
  ╚═══════════════════════════════════════════════════╝

  > _

  [File references shown below]

  Debug panel (Press F6 to toggle)
```

## Decision Tree

### Is Index Needed?
```
Does .rustean-index/ exist?
├─ NO → Prompt to build (NEEDS DECISION)
└─ YES → Has changes?
         ├─ NO → Skip, continue to TUI
         └─ YES → Prompt to update (NEEDS DECISION)
```

### User Decision Points

#### First Time Run
```
User sees: "No code index found"
Choice:   Build now? [Y/N/Q]
├─ Y → Index (full scan)
├─ N → Skip (no indexing)
└─ Q → Quit app
```

#### Changes Detected
```
User sees: "Changes detected"
Choice:   Update? [Y/N/Q]
├─ Y → Update (incremental re-index)
├─ N → Keep old (continue with stale)
└─ Q → Quit app
```

#### Language Not Supported
```
User sees: "Language Not Supported"
Choice:   Auto (no decision needed)
├─ Message shown: ~3-5 seconds
├─ Auto-continue: To TUI
└─ No indexing performed
```

## Code Location Reference

| Component | File | Key Functions |
|-----------|------|---|
| Startup flow | `src/startup.rs` | `run_startup()` |
| Language analysis | `src/indexer/analyzer.rs` | `CodebaseAnalyzer::analyze()` |
| Index checking | `src/startup.rs` | `check_index_exists()` |
| Change detection | `src/startup.rs` | `detect_codebase_changes()` |
| Prompts | `src/startup.rs` | `prompt_for_*()` functions |
| Indexing | `src/indexer/manager.rs` | `IndexManager::index_project()` |
| Progress display | `src/startup.rs` | `index_with_progress()` |
| TUI launch | `src/main.rs` | `run_tui()` |

## Environment Variables

| Variable | Effect |
|----------|--------|
| None currently | System uses sensible defaults |

## Performance

| Operation | Time |
|-----------|------|
| Language detection | ~100-500ms |
| Index check | ~10-50ms |
| Change detection | ~50-200ms |
| Full indexing (small) | <1s |
| Full indexing (medium) | 1-5s |
| Full indexing (large) | 5-30s |
| Incremental update | 100-500ms |
| TUI launch | ~100ms |

**Total first run:** ~2-10 seconds (mostly indexing)
**Subsequent runs:** ~100-500ms (if no changes)

## Error Handling

| Error | Message | Result |
|-------|---------|--------|
| No supported files | "Language Not Supported" | TUI launches without indexing |
| Permission denied | Error message | Quits with error |
| Disk full | Indexing error | Quits with error |
| Corrupted index | Auto-recovers | Rebuilds automatically |

## See Also

- [Getting Started](../getting-started/) - First-time setup
- [Language Detection](../semantic-indexer/supported-languages.md) - How language detection works
- [CLI Commands](../cli/) - Available commands
- [TUI Guide](../tui/) - Interactive interface
