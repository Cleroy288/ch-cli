# Message Parsing & Storage System

This document explains how user messages are parsed, stored, and structured in the tcah CLI tool.

---

## Overview

When you press **Enter**, your input is parsed into structured segments that distinguish between:
- **Plain text** - Regular typed content
- **File references** - Files selected via the `@` picker
- **Folder references** - Folders selected via the `@` picker

This structured data is stored in a conversation history for context-aware operations.

---

## Data Structures

### MessageSegment (Enum)

Represents a single piece of a message:

```rust
pub enum MessageSegment {
    Text(String),
    FileReference {
        full_path: String,      // Complete path: "./src/main.rs"
        display_name: String,   // What's shown: "main.rs"
    },
    FolderReference {
        full_path: String,      // Complete path: "./src"
        display_name: String,   // What's shown: "src"
    },
}
```

### UserMessage (Struct)

A complete parsed message:

```rust
pub struct UserMessage {
    pub segments: Vec<MessageSegment>,  // Parsed segments
    pub timestamp: u64,                 // Unix timestamp
    pub raw_input: String,              // Original input
}
```

### ConversationHistory (Struct)

Stores all messages:

```rust
pub struct ConversationHistory {
    messages: VecDeque<UserMessage>,  // FIFO queue
    max_messages: usize,              // Default: 100
}
```

---

## Parsing Process

### Step 1: User Types Message

```
Input: "Check main.rs in src folder"
       ^      ^^^^^^^^    ^^^
       text   file ref    folder ref
```

### Step 2: Press Enter

The parser processes the input:
1. Identifies file/folder references (tracked in `app.file_references`)
2. Splits text between references
3. Creates appropriate `MessageSegment` types
4. Builds a `UserMessage`
5. Adds to `ConversationHistory`

### Step 3: Stored Structure

```rust
UserMessage {
    raw_input: "Check main.rs in src folder",
    segments: [
        Text("Check "),
        FileReference {
            full_path: "./src/main.rs",
            display_name: "main.rs",
        },
        Text(" in "),
        FolderReference {
            full_path: "./src",
            display_name: "src",
        },
        Text(" folder"),
    ],
    timestamp: 1706380800,
}
```

**Note:** Trailing whitespace is automatically trimmed, and whitespace-only text segments are filtered out.

---

## Examples

### Example 1: Plain Text Only

**Input:**
```
"Hello, how are you?"
```

**Parsed:**
```rust
segments: [
    Text("Hello, how are you?")
]
```

**Debug Output:**
```
[0] Text: "Hello, how are you?"
Stats: 0 files, 0 folders
```

---

### Example 2: Single File Reference

**Input:**
```
"Check main.rs"
```

**Parsed:**
```rust
segments: [
    Text("Check "),
    FileReference {
        full_path: "./src/main.rs",
        display_name: "main.rs",
    }
]
```

**Debug Output:**
```
[0] Text: "Check "
[1] File: main.rs → ./src/main.rs
Stats: 1 files, 0 folders
```

---

### Example 3: Multiple References

**Input:**
```
"Copy config.json from src to target folder"
```

**Parsed:**
```rust
segments: [
    Text("Copy "),
    FileReference {
        full_path: "./config.json",
        display_name: "config.json",
    },
    Text(" from "),
    FolderReference {
        full_path: "./src",
        display_name: "src",
    },
    Text(" to "),
    FolderReference {
        full_path: "./target",
        display_name: "target",
    },
    Text(" folder"),
]
```

**Debug Output:**
```
[0] Text: "Copy "
[1] File: config.json → ./config.json
[2] Text: " from "
[3] Folder: src → ./src
[4] Text: " to "
[5] Folder: target → ./target
[6] Text: " folder"
Stats: 1 files, 2 folders
```

---

### Example 4: Complex Message

**Input:**
```
"The main.rs file in src needs the config.json from docs folder"
```

**Parsed:**
```rust
segments: [
    Text("The "),
    FileReference { full_path: "./src/main.rs", display_name: "main.rs" },
    Text(" file in "),
    FolderReference { full_path: "./src", display_name: "src" },
    Text(" needs the "),
    FileReference { full_path: "./docs/config.json", display_name: "config.json" },
    Text(" from "),
    FolderReference { full_path: "./docs", display_name: "docs" },
    Text(" folder"),
]
```

**Debug Output:**
```
[0] Text: "The "
[1] File: main.rs → ./src/main.rs
[2] Text: " file in "
[3] Folder: src → ./src
[4] Text: " needs the "
[5] File: config.json → ./docs/config.json
[6] Text: " from "
[7] Folder: docs → ./docs
[8] Text: " folder"
Stats: 2 files, 2 folders
```

---

## Debug Panel Display

The UI shows parsed messages in real-time:

```
┌─ Debug: Parsed Messages ────────────────────────────────┐
│ 📋 Message History (3 messages)                          │
│                                                           │
│ ─── Message 1 ───                                        │
│ Raw: "Check main.rs in src folder"                       │
│ Parsed Segments:                                         │
│   [0] Text: "Check "                                     │
│   [1] File: main.rs → ./src/main.rs                      │
│   [2] Text: " in "                                       │
│   [3] Folder: src → ./src                                │
│   [4] Text: " folder"                                    │
│   Stats: 1 files, 1 folders                              │
│                                                           │
│ ─── Message 2 ───                                        │
│ Raw: "Copy config.json to docs"                          │
│ Parsed Segments:                                         │
│   [0] Text: "Copy "                                      │
│   [1] File: config.json → ./config.json                  │
│   [2] Text: " to "                                       │
│   [3] Folder: docs → ./docs                              │
│   Stats: 1 files, 1 folders                              │
└───────────────────────────────────────────────────────────┘
```

---

## API Methods

### UserMessage Methods

```rust
// Get reconstructed message text
message.to_string() -> String

// Get debug representation
message.debug_string() -> String

// Count references
message.file_count() -> usize
message.folder_count() -> usize

// Extract paths
message.file_paths() -> Vec<String>
message.folder_paths() -> Vec<String>
```

### ConversationHistory Methods

```rust
// Add message
history.add_message(message)

// Access messages
history.messages() -> &VecDeque<UserMessage>
history.last_message() -> Option<&UserMessage>

// Query
history.len() -> usize
history.is_empty() -> bool

// Management
history.clear()
history.debug_string() -> String
```

---

## Use Cases

### Use Case 1: Code Review Request

**User Input:**
```
"Review the changes in main.rs and check if it follows patterns from utils.rs"
```

**What Gets Stored:**
- Text: "Review the changes in "
- File: `main.rs` (path: `./src/main.rs`)
- Text: " and check if it follows patterns from "
- File: `utils.rs` (path: `./src/utils.rs`)

**Future Use:**
- Read contents of `./src/main.rs`
- Read contents of `./src/utils.rs`
- Perform comparison/analysis
- Provide context-aware feedback

---

### Use Case 2: File Operation Request

**User Input:**
```
"Move test.py from tests to archive folder and update references"
```

**What Gets Stored:**
- Text: "Move "
- File: `test.py` (path: `./tests/test.py`)
- Text: " from "
- Folder: `tests` (path: `./tests`)
- Text: " to "
- Folder: `archive` (path: `./archive`)
- Text: " folder and update references"

**Future Use:**
- Source file: `./tests/test.py`
- Source folder: `./tests`
- Destination: `./archive`
- Action: Move file and update imports/references

---

### Use Case 3: Documentation Request

**User Input:**
```
"Generate documentation for the API in api.rs and save to docs folder"
```

**What Gets Stored:**
- Text: "Generate documentation for the API in "
- File: `api.rs` (path: `./src/api.rs`)
- Text: " and save to "
- Folder: `docs` (path: `./docs`)
- Text: " folder"

**Future Use:**
- Read: `./src/api.rs`
- Analyze API structure
- Generate docs
- Save to: `./docs/`

---

### Use Case 4: Multi-File Context

**User Input:**
```
"Explain how main.rs uses functions from lib.rs and types from types.rs"
```

**What Gets Stored:**
- Text: "Explain how "
- File: `main.rs` (path: `./src/main.rs`)
- Text: " uses functions from "
- File: `lib.rs` (path: `./src/lib.rs`)
- Text: " and types from "
- File: `types.rs` (path: `./src/types.rs`)

**Future Use:**
- Read all three files
- Build context with multiple file contents
- Analyze relationships
- Provide comprehensive explanation

---

## Memory Management

### History Limits

```rust
ConversationHistory {
    max_messages: 100,  // Configurable
}
```

- **Default:** Store last 100 messages
- **FIFO:** Oldest messages dropped when limit reached
- **Memory:** ~1-10KB per message (depending on content)
- **Total:** ~100KB-1MB for full history

### Display Limits

- **Debug Panel:** Shows last 10 messages
- **Full Access:** All messages available via API
- **Overflow Indicator:** "... and N more messages"

---

## Whitespace Handling

### Automatic Trimming

The parser automatically handles whitespace:

**1. Trailing Whitespace Removal**
```
Input: "Check main.rs   "
                    ^^^^ (removed)
Stored: "Check main.rs"
```

**2. Whitespace-Only Segments Filtered**
```
Input: "Check main.rs in src "
                           ^ (space after folder)
Segments created: [Text("Check "), File, Text(" in "), Folder]
                                                       ^^^^^^^ (no trailing space segment)
```

**3. Preserves Internal Whitespace**
```
Input: "Check    main.rs    in    src"
       ^^^^       ^^^^    ^^^^
       (all preserved in Text segments)
```

### Why This Matters

Without filtering:
```
❌ BAD:
  [0] Text: "Check "
  [1] File: main.rs
  [2] Text: " in "
  [3] Folder: src
  [4] Text: "   "  ← Useless whitespace-only segment
```

With filtering:
```
✅ GOOD:
  [0] Text: "Check "
  [1] File: main.rs
  [2] Text: " in "
  [3] Folder: src
  (No whitespace-only segment)
```

### Implementation

```rust
// Trim trailing whitespace from input
self.input = self.input.trim_end().to_string();

// Filter whitespace-only text segments
if !text.trim().is_empty() {
    segments.push(MessageSegment::Text(text));
}
```

---

## File Path Handling

### What's Stored

| User Selects | Display Name | Full Path Stored |
|--------------|--------------|------------------|
| `./src/main.rs` | `main.rs` | `./src/main.rs` |
| `./config.json` | `config.json` | `./config.json` |
| `./tests/` | `tests` | `./tests/` |
| `./docs/guide.md` | `guide.md` | `./docs/guide.md` |
| `./.git/config` | `config` | `./.git/config` |

### Why Store Full Paths?

✅ **Future file operations** - Know exactly where file is
✅ **Context loading** - Read correct file contents
✅ **Disambiguation** - Handle same filename in different folders
✅ **Tool integration** - Pass paths to external tools
✅ **History replay** - Reconstruct exact context later

---

## Integration Points

### Current Implementation

1. **Input Parsing** - `app.rs::parse_and_store_message()`
2. **Storage** - `message.rs::ConversationHistory`
3. **Display** - `ui.rs::render_debug_panel()`

### Future Integration Ideas

```rust
// Load file contents for AI context
fn build_context(message: &UserMessage) -> String {
    let mut context = String::new();
    for path in message.file_paths() {
        let content = fs::read_to_string(path)?;
        context.push_str(&format!("=== {} ===\n{}\n", path, content));
    }
    context
}

// Execute file operations
fn execute_operation(message: &UserMessage) {
    match analyze_intent(message) {
        Intent::Copy { from, to } => { /* copy files */ },
        Intent::Move { from, to } => { /* move files */ },
        Intent::Delete { files } => { /* delete files */ },
        _ => {},
    }
}

// Build search index
fn index_references(history: &ConversationHistory) -> HashMap<String, Vec<usize>> {
    let mut index = HashMap::new();
    for (i, msg) in history.messages().iter().enumerate() {
        for path in msg.file_paths() {
            index.entry(path).or_insert_with(Vec::new).push(i);
        }
    }
    index
}
```

---

## Performance Considerations

### Parsing Speed

- **Simple text:** < 1ms
- **With references:** < 5ms
- **Complex (10+ refs):** < 10ms

### Memory Usage

- **Empty message:** ~100 bytes
- **Text segment:** ~50 bytes + text length
- **File reference:** ~200 bytes (paths + metadata)
- **Typical message:** ~500 bytes - 2KB

### Storage Strategy

- Use `VecDeque` for efficient FIFO operations
- Limit history size to prevent unbounded growth
- Clone-on-demand (no unnecessary copies)
- Lazy string formatting (only when displayed)

---

## Testing Scenarios

### Scenario 1: Sequential References

```
Input: "file1.rs file2.rs file3.rs"
Expected: 3 FileReference segments (no Text between)
```

### Scenario 2: Empty Spaces

```
Input: "Check    main.rs    in    src"
Expected: Preserves exact spacing in Text segments
```

### Scenario 3: Special Characters

```
Input: "Use file-name_v2.final.rs!"
Expected: Handles hyphens, underscores, dots correctly
```

### Scenario 4: Edge Cases

```
Input: ""
Expected: Empty message not stored

Input: "   "
Expected: Whitespace-only message not stored

Input: "@" (cancelled)
Expected: @ symbol removed, no reference created
```

---

## Future Enhancements

### Planned Features

1. **Semantic Analysis**
   - Detect intent (copy, move, delete, review, etc.)
   - Extract action verbs and targets
   - Build command objects

2. **Context Windows**
   - Group related messages
   - Track conversation threads
   - Maintain session state

3. **Export/Import**
   - Save history to JSON/YAML
   - Load previous sessions
   - Share conversations

4. **Search & Filter**
   - Find messages by keyword
   - Filter by file references
   - Query by timestamp range

5. **Undo/Redo**
   - Navigate message history
   - Edit previous messages
   - Re-submit with modifications

---

## Summary

The message parsing system provides:

✅ **Structured Storage** - Text and references separated
✅ **Full Context** - Complete file paths preserved
✅ **Easy Debugging** - Visual representation in UI
✅ **Future-Ready** - Extensible for AI/tool integration
✅ **Efficient** - Fast parsing, bounded memory
✅ **Reliable** - Handles edge cases gracefully

This foundation enables building sophisticated context-aware features for the CLI tool!