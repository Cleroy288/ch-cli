# tcah CLI - Usage Guide

A comprehensive guide to using the tcah CLI with file/folder picker functionality.

---

## Quick Start

```bash
cargo run
```

You'll see:
```
┌─ tcah ──────────────────────────────────────┐
│                                              │
│  ████████  ██████   █████  ██   ██          │
│     ██    ██       ██   ██ ██   ██          │
│     ██    ██       ███████ ███████          │
│     ██    ██       ██   ██ ██   ██          │
│     ██     ██████  ██   ██ ██   ██          │
│                                              │
└──────────────────────────────────────────────┘
┌─ Input ─────────────────────────────────────┐
│Type something... (@ for files/folders...)   │
└──────────────────────────────────────────────┘
┌─ Debug: Parsed Messages ────────────────────┐
│                                              │
│ No messages yet. Type something and press    │
│ Enter!                                       │
│                                              │
└──────────────────────────────────────────────┘
```

---

## Feature 1: Basic Input

Just start typing in the input box!

**Example:**
```
┌─ Input ─────────────────────────────────────┐
│Hello, I'm typing in the CLI!█               │
└──────────────────────────────────────────────┘
```

**Keyboard Controls:**
- Type any character to add text
- `Backspace` - Delete previous character
- `Delete` - Delete character at cursor
- `←` / `→` - Move cursor left/right
- `Home` - Jump to beginning
- `End` - Jump to end

---

## Feature 2: File/Folder Picker

The main feature! Reference files and folders in your input.

### Step 1: Trigger the Picker

Type `@` anywhere in your input:

```
┌─ Input ─────────────────────────────────────┐
│I want to reference @█                        │
└──────────────────────────────────────────────┘
```

**Note:** When picker is active, the debug panel is hidden for clarity.

### Step 2: Choose Type

A picker appears with two options:

```
┌─ Input ─────────────────────────────────────┐
│I want to reference @█                        │
└──────────────────────────────────────────────┘
┌─ Select Type (↑↓ navigate, Enter select) ───┐
│ ▸ folder                                     │
│ ◆ file                                       │
└──────────────────────────────────────────────┘
(Debug panel hidden while picker is active)
```

**Controls:**
- `↑` / `↓` - Navigate between options
- `Enter` - Select highlighted option
- `f` - Quick shortcut for "file"
- `d` - Quick shortcut for "folder"
- `ESC` - Cancel

### Step 3: Search and Select

After choosing a type, start typing to search:

#### Example: Searching for Files

```
┌─ Input ─────────────────────────────────────┐
│I want to reference @█                        │
└──────────────────────────────────────────────┘
┌─ Files (query: 'main') [1/3] ───────────────┐
│ ◆ main.rs                                    │
│ ◆ domain.rs                                  │
│ ◆ remain.txt                                 │
└──────────────────────────────────────────────┘
↑↓ navigate  Enter select  ESC cancel
```

**Controls:**
- **Type** - Filter results in real-time
- `↑` / `↓` - Navigate through results
- `Enter` - Select highlighted file/folder
- `F5` - Refresh/rescan filesystem (get latest files/folders)
- `Backspace` - Go back to type selection (when query is empty)
- `ESC` - Cancel and remove `@`

### Step 4: Selection Inserted

When you press Enter, the selected path is inserted:

```
┌─ Input ─────────────────────────────────────┐
│I want to reference src/main.rs█             │
└──────────────────────────────────────────────┘
```

---

## Complete Workflow Examples

### Example 1: Reference a Configuration File

1. Type: `"Check the config in "`
2. Press: `@`
3. Press: `f` (or navigate to "file" and press Enter)
4. Type: `config`
5. See filtered results:
   ```
   ◆ Cargo.toml
   ◆ .gitignore
   ◆ config.json
   ```
6. Use `↓` to select `config.json`
7. Press `Enter`

**Result:** `"Check the config in config.json"` (with `config.json` visually highlighted in green)

### Example 2: Reference a Folder

1. Type: `"The source code is in "`
2. Press: `@`
3. Press: `d` (quick shortcut for directory)
4. Type: `src`
5. See results:
   ```
   ▸ src
   ▸ src_backup
   ```
6. Press `Enter` (first item selected by default)

**Result:** `"The source code is in src"` (with `src` visually highlighted in cyan)

### Example 3: Multiple References

You can use `@` multiple times:

1. Type: `"Copy "`
2. Press: `@` → `f` → type `readme` → select `README.md`
3. Type: `" to "`
4. Press: `@` → `d` → type `docs` → select `docs/`
5. Result: `"Copy README.md to docs/"` (both `README.md` and `docs` highlighted)

---

## Tips & Tricks

### Tip 1: Quick Selection
If you know the exact filename, just type it quickly:
- `@` → `f` → `main` → `Enter` (selects first match)

### Tip 2: Cancel Anytime
Made a mistake? Press `ESC` to cancel the picker and remove the `@` symbol.

### Tip 3: Empty Query Shows All
When the picker opens, press `Enter` immediately to see ALL files/folders (no filtering).

### Tip 4: Scroll Through Results
The picker automatically scrolls to keep your selection visible, even with hundreds of files.

### Tip 5: Fast Navigation
The picker shows your position: `[3/15]` means you're on item 3 of 15 total results.

### Tip 6: Stay Up-to-Date
Created a new file in another terminal? Press `F5` in the picker to refresh instantly, or just close and reopen with `@` (auto-refreshes).

### Tip 7: Visual Distinction
Selected files/folders are automatically highlighted in your input:
- **Files** - Green background with white bold text
- **Folders** - Cyan background with white bold text
- **Normal text** - Yellow (default input color)

### Tip 8: Clean Paths
Only filenames/foldernames are inserted, not full paths:
- Selecting `/path/to/myproject/src/main.rs` inserts just `main.rs`
- Selecting `/path/to/myproject/docs/` inserts just `docs`
- File extensions are always preserved

---

## What Gets Indexed?

The file scanner automatically:

✅ **Includes:**
- ALL files and folders (no exclusions!)
- Hidden files (starting with `.`)
- `node_modules/` directory
- `target/` directory (Rust build artifacts)
- `.git/` directory
- Nested directories (up to 10 levels deep)
- All file types

**Auto-Refresh:**
- Filesystem is automatically rescanned every time you press `@`
- Press `F5` while picker is open to manually refresh
- Always shows the latest files and folders

**Icons:**
- ▸ Triangle arrow for folders/directories
- ◆ Diamond for files

**UI Behavior:**
- When picker is active (`@` pressed), debug panel is hidden
- Picker uses the full bottom area for clean display
- Debug panel reappears when picker is closed
- No overlap or text mixing

Everything is indexed and searchable!

---

## Visual Key Reference

```
┌─────────────────────────────────────────────────┐
│                  KEYBOARD MAP                   │
├─────────────────────────────────────────────────┤
│                                                 │
│  @           Open file/folder picker (auto-scan)│
│  ESC         Cancel picker / Exit app           │
│  Ctrl+C      Exit application                   │
│  ↑ / ↓       Navigate picker items              │
│  Enter       Select item / Confirm              │
│  F5          Refresh/rescan filesystem          │
│  Backspace   Delete char / Go back in picker    │
│  f           Quick: select "file" mode          │
│  d           Quick: select "folder" mode        │
│  ← / →       Move cursor in input               │
│  Home        Jump to start of input             │
│  End         Jump to end of input               │
│                                                 │
└─────────────────────────────────────────────────┘
```

---

## Troubleshooting

### "No results found"
- Check your spelling
- Try a shorter search query
- Remember: search is case-insensitive
- The file might be beyond 10 levels deep in the directory structure

### Picker not appearing after `@`
- Make sure you actually typed the `@` character
- Check if you're in the input box (cursor should be visible)

### Wrong file selected
- Press `ESC` to cancel
- Use `Backspace` to delete the inserted path
- Try again with a more specific search query

### Performance issues with large projects
- The scanner limits depth to 10 levels
- Filesystem is rescanned every time you press `@` (usually very fast)
- Press `F5` only when needed to avoid unnecessary rescans
- Consider adding filtering logic in `fs.rs` if you have very large directories
- You can exclude specific folders by modifying the `scan_recursive` function

---

## Advanced Usage

### Searching Tips

**Partial matching:** Type any part of the filename
- Query: `main` → Matches: `main.rs`, `domain.rs`, `remain.txt`

**Start of name:** The search checks the entire filename
- Query: `src` → Matches: `src/main.rs`, `resources.txt`

**No wildcards needed:** Just type what you remember
- Query: `rs` → Matches: `main.rs`, `first.rs`, `version.txt`

### Navigation Patterns

**Quick file access:**
```
@ → f → [first letters] → Enter
```

**Browse all folders:**
```
@ → d → Enter (on empty query)
```

**Precise selection:**
```
@ → f → [specific name] → ↓ ↓ ↓ → Enter
```

**Refresh while browsing:**
```
@ → f → [typing...] → F5 (to rescan) → continue browsing
```

---

## Color Guide

The UI uses colors to help you navigate:

- **Cyan** - Title and type chooser (folder/file selection)
- **Green** - Active file/folder picker
- **Yellow** - Your input text
- **White** - Selected items (with highlight)
- **Gray** - Help text and placeholders
- **Black on Cyan/Green** - Currently highlighted selection in picker

**In Input Box:**
- **Yellow** - Normal typed text
- **White on Green** - Selected file names (bold)
- **White on Cyan** - Selected folder names (bold)

**Debug Panel:**
- **Magenta border** - Debug panel frame
- **Cyan** - Section headers
- **Yellow** - Message numbers
- **Green** - File references
- **Cyan** - Folder references
- **Gray** - Metadata and labels

**Icons:**
- **▸** - Folder/Directory (triangle arrow pointing right)
- **◆** - File (diamond shape)

**Layout Behavior:**
- Picker active → Debug panel hidden
- Picker closed → Debug panel visible
- No UI overlap or mixed content

**Visual Example:**
```
Input: "Check the main.rs file in src folder"
                  ^^^^^^^^^        ^^^
                  (green bg)    (cyan bg)
```

---

## Smart Path Insertion

When you select a file or folder from the picker:

### What Gets Inserted:
- ✅ **Filename only** (not the full path)
- ✅ **Foldername only** (not the full path)
- ✅ **File extensions preserved** (`.rs`, `.py`, `.json`, etc.)

### Examples:

| Full Path | What's Inserted |
|-----------|----------------|
| `/Users/me/projects/app/src/main.rs` | `main.rs` |
| `/Users/me/projects/app/config.json` | `config.json` |
| `/Users/me/projects/app/docs/` | `docs` |
| `./target/debug/ch-cli` | `ch-cli` |
| `./node_modules/` | `node_modules` |

### Visual Highlighting:

**Files** appear with **green background**:
```
┌─ Input ────────────────────────────────────┐
│Check main.rs for the code                  │
│      ^^^^^^^^ (white text on green bg)     │
└────────────────────────────────────────────┘
```

**Folders** appear with **cyan background**:
```
┌─ Input ────────────────────────────────────┐
│Look in the src folder                      │
│           ^^^ (white text on cyan bg)      │
└────────────────────────────────────────────┘
```

**Mixed content**:
```
┌─ Input ────────────────────────────────────┐
│Copy main.rs from src to target folder      │
│     ^^^^^^      ^^^    ^^^^^^              │
│    (green)    (cyan)   (cyan)              │
└────────────────────────────────────────────┘
```

---

## Exit the Application

Two ways to exit:

1. **Press `ESC`** - Quick exit
2. **Press `Ctrl+C`** - Standard interrupt

Both show a friendly goodbye message:

```
╔═══════════════════════════════════════════╗
║                                           ║
║          Thanks for using tcah!           ║
║                                           ║
║            See you next time! 👋          ║
║                                           ║
╚═══════════════════════════════════════════╝
```

---

## Need Help?

- Check `README.md` for technical details
- Review the code structure in `src/`
- File an issue if you find bugs

Happy coding! 🚀