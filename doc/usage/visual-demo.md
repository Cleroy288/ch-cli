# Visual Demo - File Reference Styling

This document demonstrates the new visual styling feature for selected files and folders.

---

## Feature Overview

When you select a file or folder using the `@` picker, it gets:
1. **Inserted as just the filename/foldername** (not the full path)
2. **Visually highlighted** with a colored background
3. **Tracked internally** with the full path for future use

---

## Color Scheme

### Files: Green Background
- **Background:** Green
- **Text:** White (Bold)
- **Example:** `main.rs`, `config.json`, `README.md`

### Folders: Cyan Background
- **Background:** Cyan
- **Text:** White (Bold)
- **Example:** `src`, `docs`, `target`

### Normal Text: Yellow
- **Background:** None
- **Text:** Yellow
- **Example:** Regular typed text

---

## Before & After Examples

### Example 1: Single File Reference

**Before Selection:**
```
┌─ Input ─────────────────────────────────────┐
│Check the @█                                  │
└──────────────────────────────────────────────┘
```

**After Selecting `/path/to/project/src/main.rs`:**
```
┌─ Input ─────────────────────────────────────┐
│Check the main.rs█                            │
│         ████████                             │
│         (green background, white bold text)  │
└──────────────────────────────────────────────┘
```

---

### Example 2: Single Folder Reference

**Before Selection:**
```
┌─ Input ─────────────────────────────────────┐
│Look in @█                                    │
└──────────────────────────────────────────────┘
```

**After Selecting `/path/to/project/src/`:**
```
┌─ Input ─────────────────────────────────────┐
│Look in src█                                  │
│        ███                                   │
│        (cyan background, white bold text)    │
└──────────────────────────────────────────────┘
```

---

### Example 3: Multiple References

**After Multiple Selections:**
```
┌─ Input ─────────────────────────────────────┐
│Copy main.rs from src to target directory    │
│     ███████     ███    ██████               │
│     (green)    (cyan)  (cyan)               │
└──────────────────────────────────────────────┘
```

**Breakdown:**
- `main.rs` - File (green background)
- `src` - Folder (cyan background)
- `target` - Folder (cyan background)
- `Copy`, `from`, `to`, `directory` - Normal text (yellow)

---

### Example 4: Complex Sentence

**Input with mixed content:**
```
┌─ Input ─────────────────────────────────────┐
│The config.json file is in the docs folder   │
│    ███████████             ████             │
│    (green bg)              (cyan bg)        │
└──────────────────────────────────────────────┘
```

---

## Path Simplification Examples

### Original Path → Inserted Text

| Full Path in Filesystem | What You See in Input |
|------------------------|----------------------|
| `./src/main.rs` | `main.rs` ✅ |
| `./src/lib.rs` | `lib.rs` ✅ |
| `./target/debug/app` | `app` ✅ |
| `./docs/README.md` | `README.md` ✅ |
| `./tests/integration/` | `integration` ✅ |
| `./node_modules/package/` | `package` ✅ |
| `./.git/config` | `config` ✅ |
| `./Cargo.toml` | `Cargo.toml` ✅ |

**Key Points:**
- ✅ Only the **last component** of the path is shown
- ✅ File **extensions are preserved** (`.rs`, `.json`, `.md`, etc.)
- ✅ Full path is **tracked internally** for future use
- ✅ Input box stays **clean and readable**

---

## Live Workflow Demo

### Step-by-Step Visual Flow

#### Step 1: Start Typing
```
┌─ Input ─────────────────────────────────────┐
│Check █                                       │
└──────────────────────────────────────────────┘
```

#### Step 2: Trigger Picker
```
┌─ Input ─────────────────────────────────────┐
│Check @█                                      │
└──────────────────────────────────────────────┘
┌─ Select Type ───────────────────────────────┐
│ ▸ folder                                     │
│ ◆ file                                       │
└──────────────────────────────────────────────┘
```

#### Step 3: Select File Type
```
┌─ Input ─────────────────────────────────────┐
│Check @█                                      │
└──────────────────────────────────────────────┘
┌─ Files (query: '') [1/25] ──────────────────┐
│ ◆ main.rs                                    │
│ ◆ lib.rs                                     │
│ ◆ app.rs                                     │
│ ◆ config.json                                │
│ ...                                          │
└──────────────────────────────────────────────┘
```

#### Step 4: Search and Select
```
┌─ Input ─────────────────────────────────────┐
│Check @█                                      │
└──────────────────────────────────────────────┘
┌─ Files (query: 'main') [1/1] ───────────────┐
│ ◆ main.rs                                    │
└──────────────────────────────────────────────┘
```

#### Step 5: Result (File Inserted & Styled)
```
┌─ Input ─────────────────────────────────────┐
│Check main.rs█                                │
│      ████████                                │
│      (white on green, bold)                  │
└──────────────────────────────────────────────┘
```

#### Step 6: Continue Typing
```
┌─ Input ─────────────────────────────────────┐
│Check main.rs in █                            │
│      ████████                                │
│      (green bg)                              │
└──────────────────────────────────────────────┘
```

#### Step 7: Add Folder Reference
```
┌─ Input ─────────────────────────────────────┐
│Check main.rs in @█                           │
│      ████████                                │
└──────────────────────────────────────────────┘
```

#### Step 8: Final Result
```
┌─ Input ─────────────────────────────────────┐
│Check main.rs in src folder█                 │
│      ████████    ███                         │
│      (green)    (cyan)                       │
└──────────────────────────────────────────────┘
```

---

## Comparison: Old vs New

### Old Behavior (Before Update)
```
User selects: /path/to/project/src/main.rs
Input shows: /path/to/project/src/main.rs
              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
              (all yellow, full path, cluttered)
```

### New Behavior (After Update)
```
User selects: /path/to/project/src/main.rs
Input shows:  main.rs
              ████████
              (white on green, filename only, clean)
```

**Benefits:**
- ✅ **Cleaner** - Only shows what matters
- ✅ **Highlighted** - Easy to spot file references
- ✅ **Organized** - Full path tracked internally
- ✅ **Readable** - Less clutter in the input box

---

## Technical Details

### Color Codes

| Element | Foreground | Background | Modifier |
|---------|-----------|------------|----------|
| File reference | White | Green | Bold |
| Folder reference | White | Cyan | Bold |
| Normal text | Yellow | None | None |
| Placeholder | Dark Gray | None | None |

### Data Structure

Each file/folder reference tracks:
```rust
FileReference {
    start: 6,                           // Position in input
    end: 14,                            // End position
    full_path: "/path/to/src/main.rs", // Complete path
    display_name: "main.rs",            // What's shown
    is_dir: false,                      // File or folder?
}
```

### Rendering Logic

1. **Parse input** - Find all file references
2. **Split text** - Divide into normal text and references
3. **Apply styles** - Color each segment appropriately
4. **Render** - Display styled line in UI

---

## Use Cases

### Use Case 1: Documentation
```
Input: "See main.rs and config.json for details"
       ████ ████████    █████████████
       file           file

Perfect for referencing multiple files in documentation!
```

### Use Case 2: Issue Tracking
```
Input: "Bug in auth.rs related to users table"
            █████████
            file

Quickly reference the problematic file!
```

### Use Case 3: File Organization
```
Input: "Move test.py from tests to archive folder"
            ███████      █████    ███████
            file         folder   folder

Track file movements clearly!
```

### Use Case 4: Code Review
```
Input: "Check components/Button.tsx and styles/theme.css"
            ███████████████████████    ████████████████
            file                       file

Reference files across different directories!
```

---

## Keyboard Interaction

### Editing Behavior

**Deleting Characters:**
- Backspace/Delete work normally
- File references update positions automatically
- If reference is partially deleted, highlighting adjusts

**Moving Cursor:**
- Arrow keys move through text normally
- Cursor can be placed inside file references
- Visual highlighting persists

**Adding Text:**
- Type anywhere in the input
- File references maintain their positions
- New text appears in normal yellow color

---

## Edge Cases Handled

### Case 1: Same Filename, Different Paths
```
Selected: /project/src/utils.rs    → Inserts: utils.rs (green)
Selected: /project/tests/utils.rs  → Inserts: utils.rs (green)

Both show as "utils.rs" but tracked with different full paths!
```

### Case 2: Files Without Extensions
```
Selected: /project/Makefile  → Inserts: Makefile (green)
Selected: /project/LICENSE   → Inserts: LICENSE (green)
```

### Case 3: Hidden Files
```
Selected: /project/.gitignore  → Inserts: .gitignore (green)
Selected: /project/.env        → Inserts: .env (green)
```

### Case 4: Very Long Names
```
Selected: /project/very-long-filename-that-might-be-problematic.rs
Inserts:  very-long-filename-that-might-be-problematic.rs
          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
          (green background, wraps if needed)
```

---

## Best Practices

### ✅ Do:
- Use `@` picker for referencing files/folders
- Let the visual highlighting guide you
- Keep file references clear and distinct
- Use descriptive surrounding text

### ❌ Don't:
- Manually type filenames (use picker instead)
- Edit file reference names directly
- Remove partial file references (delete the whole thing)

---

## Future Enhancements

Potential future features:
- Click on highlighted reference to open picker for editing
- Hover to show full path in tooltip
- Export references as structured data
- Auto-complete based on selected files
- Link multiple files as dependencies

---

## Whitespace Handling

### Automatic Cleaning

The parser automatically handles trailing whitespace:

**Before (with trailing spaces):**
```
Input: "Check main.rs in src    "
                            ^^^^ (trailing spaces)
```

**After parsing:**
```
Segments:
  [0] Text: "Check "
  [1] File: main.rs → ./src/main.rs
  [2] Text: " in "
  [3] Folder: src → ./src
  (No whitespace-only segment added)
```

**Benefits:**
- ✅ No useless whitespace segments
- ✅ Clean, meaningful data
- ✅ Accurate statistics
- ✅ Better for AI processing

### Examples

**Example 1: Trailing spaces removed**
```
Input:  "Look at config.json   "
Stored: "Look at config.json"
         (spaces trimmed)
```

**Example 2: No whitespace-only segments**
```
Input:  "Copy main.rs to src "
Parsed: [Text("Copy "), File(main.rs), Text(" to "), Folder(src)]
        (No empty text segment at end)
```

**Example 3: Internal whitespace preserved**
```
Input:  "Check    main.rs    in    src"
Parsed: [Text("Check    "), File, Text("    in    "), Folder]
        (Internal spaces kept intact)
```

---

## Summary

The visual styling feature provides:

🎨 **Visual Clarity**
- Files: Green background
- Folders: Cyan background
- Normal text: Yellow

📝 **Smart Display**
- Only filename/foldername shown
- Full path tracked internally
- Extensions preserved

🧹 **Smart Parsing**
- Trailing whitespace removed
- Whitespace-only segments filtered
- Internal spacing preserved

✨ **Better UX**
- Easy to spot references
- Clean, uncluttered input
- Professional appearance
- Accurate message structure

This makes the CLI tool more intuitive and powerful for referencing files and folders in your project!