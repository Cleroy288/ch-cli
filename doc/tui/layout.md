# UI Layout & Behavior

This document explains how the UI is organized and how different panels interact.

---

## Layout Structure

The application uses a vertical layout with three main sections:

```
┌─────────────────────────────────────────────────┐
│                                                 │
│  ████████  ██████   █████  ██   ██             │  ← Title (7 lines)
│     ██    ██       ██   ██ ██   ██             │
│     ██    ██       ███████ ███████             │
│     ██    ██       ██   ██ ██   ██             │
│     ██     ██████  ██   ██ ██   ██             │
│                                                 │
├─────────────────────────────────────────────────┤
│ Your input here...                              │  ← Input Box (3 lines)
├─────────────────────────────────────────────────┤
│                                                 │
│ Debug Panel or Picker                           │  ← Dynamic Area (fills remaining)
│ (mutually exclusive)                            │
│                                                 │
└─────────────────────────────────────────────────┘
```

---

## Section Details

### 1. Title Area (Fixed: 7 lines)
- ASCII art "rustean" logo
- Cyan colored, bold
- Centered with border
- Always visible

### 2. Input Box (Fixed: 3 lines)
- User text input
- Cursor visible
- File/folder references highlighted:
  - Files: White on green background (bold)
  - Folders: White on cyan background (bold)
  - Normal text: Yellow
- Always visible

### 3. Dynamic Bottom Area (Fills remaining space)
**Shows ONE of the following:**

#### A. Debug Panel (When picker is NOT active)
```
┌─ Debug: Parsed Messages ────────────────────┐
│ 📋 Message History (2 messages)              │
│                                              │
│ ─── Message 1 ───                           │
│ Raw: "Check main.rs"                        │
│ Parsed Segments:                            │
│   [0] Text: "Check "                        │
│   [1] File: main.rs → ./src/main.rs         │
│   Stats: 1 files, 0 folders                 │
│                                              │
│ ─── Message 2 ───                           │
│ Raw: "Look in src"                          │
│ Parsed Segments:                            │
│   [0] Text: "Look in "                      │
│   [1] Folder: src → ./src                   │
│   Stats: 0 files, 1 folders                 │
└──────────────────────────────────────────────┘
```

#### B. Picker (When `@` is pressed)
```
┌─ Input ─────────────────────────────────────┐
│Check @█                                      │
└──────────────────────────────────────────────┘
┌─ Select Type ───────────────────────────────┐
│ ▸ folder                                     │
│ ◆ file                                       │
└──────────────────────────────────────────────┘
```

OR

```
┌─ Input ─────────────────────────────────────┐
│Check @█                                      │
└──────────────────────────────────────────────┘
┌─ Files (query: 'main') [1/3] ───────────────┐
│ ◆ main.rs                                    │
│ ◆ domain.rs                                  │
│ ◆ remain.txt                                 │
└──────────────────────────────────────────────┘
↑↓ navigate  Enter select  F5 refresh  ESC cancel
```

---

## UI State Machine

### State 1: Normal (Default)
- **Visible:** Title + Input + Debug Panel
- **Cursor:** Blinking in input box
- **Actions:** Type, backspace, arrow keys, Enter

### State 2: Picker Active (After `@`)
- **Visible:** Title + Input + Picker
- **Hidden:** Debug Panel (completely replaced by picker)
- **Cursor:** Hidden (picker has focus)
- **Actions:** Navigate picker, select, ESC to cancel

### State 3: Message Submitted (After Enter)
- **Action:** Parse input → Store message → Clear input
- **Result:** Returns to State 1
- **Effect:** Debug panel updates with new message

---

## Mutual Exclusion

**Important:** Debug Panel and Picker are **NEVER** shown at the same time.

```rust
if app.picker().is_active() {
    render_picker(frame, input_area, debug_area, app);
} else {
    render_debug_panel(frame, debug_area, app);
    set_cursor(frame, input_area, app);
}
```

This ensures:
- ✅ No UI overlap
- ✅ No text mixing
- ✅ Clean, professional appearance
- ✅ Clear separation of concerns

---

## Color Scheme

### Title Area
- **Border & Text:** Cyan
- **Style:** Bold
- **Background:** None (terminal default)

### Input Box
- **Border:** White
- **Normal text:** Yellow
- **File references:** White on Green (bold)
- **Folder references:** White on Cyan (bold)
- **Placeholder:** Dark Gray

### Debug Panel
- **Border:** Magenta
- **Headers:** Cyan (bold)
- **Message numbers:** Yellow (bold)
- **Labels:** Gray
- **File references:** Green (bold)
- **Folder references:** Cyan (bold)
- **Paths:** Gray
- **Content:** White

### Picker
- **Border (Type chooser):** Cyan
- **Border (File/Folder list):** Green
- **Selected item:** Black on Cyan/Green (bold)
- **Unselected items:** White
- **Help text:** Various colors (Cyan, Green, Yellow, Red)

---

## Responsive Behavior

### Window Size Changes
- Title: Fixed 7 lines
- Input: Fixed 3 lines
- Dynamic area: Adjusts to remaining space

### Content Overflow

#### Debug Panel
- Shows last 10 messages
- Indicator if more messages exist: "... and N more messages"
- Scrollable in future versions

#### Picker
- Type chooser: Fixed 2 options (no scroll needed)
- File/Folder list: Auto-scrolls to keep selection visible
- Max height: 15 items visible at once
- Navigation with ↑↓ keys

---

## Keyboard Focus

### When Debug Panel is Visible
- **Focus:** Input box
- **Cursor:** Visible, blinking
- **Keys:** All typing keys work

### When Picker is Active
- **Focus:** Picker
- **Cursor:** Hidden (picker controls focus)
- **Keys:** ↑↓ for navigation, Enter to select, ESC to cancel

---

## Rendering Order

1. **Clear frame**
2. **Render title** (always)
3. **Render input** (always)
4. **Conditional render:**
   - If picker active → Render picker
   - Else → Render debug panel + set cursor

This order ensures:
- No z-index conflicts
- Proper layering
- Clean transitions

---

## Best Practices

### ✅ Do
- Keep picker and debug panel mutually exclusive
- Use full bottom area for active component
- Maintain consistent color scheme
- Provide visual feedback for all states

### ❌ Don't
- Show picker and debug panel simultaneously
- Overlap UI elements
- Mix content from different panels
- Leave cursor visible when picker is active

---

## Example Transitions

### Normal → Picker
```
User types: @
Action: Hide debug panel, show picker
Result: Clean picker interface
```

### Picker → Normal
```
User presses: ESC or Enter
Action: Hide picker, show debug panel
Result: Return to normal view
```

### Message Submit
```
User presses: Enter
Action: Parse → Store → Clear input → Update debug
Result: Debug panel shows new message
```

---

## Future Enhancements

Potential improvements:
- **Split view:** Debug panel + picker side-by-side (wide terminals)
- **Tabs:** Switch between debug, history, help panels
- **Resize:** Adjustable panel heights
- **Themes:** Customizable color schemes
- **Transparency:** Semi-transparent overlays

---

## Summary

The UI maintains a clean, professional appearance by:

✅ **Mutual exclusion** - Debug and picker never overlap
✅ **Clear states** - Visual feedback for each mode
✅ **Consistent colors** - Magenta debug, Cyan/Green picker
✅ **Responsive** - Adapts to terminal size
✅ **User-friendly** - No confusion, clear separation

This design ensures users can focus on their task without UI clutter or confusion.