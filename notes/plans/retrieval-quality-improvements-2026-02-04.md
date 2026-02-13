# Retrieval Quality Improvements Plan

**Date:** 2026-02-04
**Status:** Planned
**Goal:** Close the gap between rustean and Augment MCP retrieval quality

---

## Summary

Manual benchmark testing revealed 5 key gaps where Augment MCP outperforms rustean:

1. **Full file content** - Augment returns complete files, rustean returns location pointers
2. **Definition accuracy** - Augment prioritizes definitions, rustean returns usages/docs first
3. **Conceptual queries** - Augment returns source code, rustean returns documentation
4. **Module structure queries** - Augment understands "modules in X", rustean treats as keyword search
5. **Caller/callee tracking** - Augment shows actual call sites, rustean returns docs about callers

---

## Benchmark Results (2026-02-04)

| Query | rustean Result | Augment MCP Result |
|-------|---------------|-------------------|
| "BgeEmbedder struct" | Location pointers | Full 228-line file |
| "how does pipeline work" | 10 doc headers | Full pipeline.rs source |
| "modules in retrieval" | Doc files (#7 has mod) | All mod.rs files |
| "RetrievalError definition" | Doc at #1, enum at #3 | Enum definition at #1 |
| "callers of embed" | Docs about callers | Actual call sites |

---

## Root Cause Analysis

### Issue 1: Basic Search Doesn't Use Intent Detection

**Location:** `src/cli/commands.rs:147-170`

**Current behavior:**
```rust
for (i, hit) in hits.iter().enumerate() {
    println!("  {}. {} {} ({}:{})", i + 1, hit.symbol.kind, hit.symbol.name, file, line);
}
```

**Problem:** No intent detection, no boost application, just raw Tantivy results.

### Issue 2: Intent Boosting Exists But Not Used

**Location:** `src/indexer/symbols.rs:286-303`

The `boost_factor_for_intent()` method exists and works correctly:
- SourceCode gets 2.0x boost for Understand intent
- Documentation gets 0.2x reduction for Understand intent

**Problem:** Basic search command doesn't call this method.

### Issue 3: No Full File Content Option

**Location:** `src/cli/commands.rs`

Search only returns `file:line` pointers. The `ContextExpander` exists but:
- Only works with `--context` flag on semantic search
- Returns snippets, not full files
- Complex integration required

### Issue 4: Definition Not Boosted for Definition Queries

**Location:** `src/indexer/symbols.rs:150-160`

Current `FindDefinition` intent boost:
```rust
QueryIntent::FindDefinition => {
    match self {
        SymbolKind::Struct | SymbolKind::Enum | SymbolKind::Trait => base * 1.1,
        _ => base,
    }
}
```

**Problem:** 1.1x boost is too weak to overcome keyword match scores.

### Issue 5: No Module Structure Query Detection

**Location:** `src/retrieval/query/mod.rs`

No detection for structural queries like:
- "modules in retrieval"
- "what's in the retrieval directory"
- "structure of indexer module"

---

## Proposed Improvements

### Improvement 1: Add Intent Detection to Basic Search

**Priority:** HIGH
**Effort:** LOW
**Files:** `src/cli/commands.rs`, `src/indexer/search.rs`

**Implementation:**

```rust
// In search_command() - add intent detection
pub fn search_command(query: &str, limit: usize, fuzzy: bool, kind: Option<&str>) -> CommandResult {
    // NEW: Detect intent from query keywords
    let intent = crate::retrieval::query::fallback_parse(query).intent;

    // Load index
    let manager = IndexManager::new();
    let index = manager.load_index(".")?;

    // NEW: Search with intent-aware boosting
    let hits = index.search_with_intent(query, limit, &intent)?;

    // Display results...
}
```

```rust
// In SearchIndex - add intent-aware search
pub fn search_with_intent(&self, query: &str, limit: usize, intent: &QueryIntent) -> SearchResult<Vec<SearchHit>> {
    let mut hits = self.search(query, limit * 2)?;  // Get extra candidates

    // Apply intent-aware boost to each hit
    for hit in &mut hits {
        let doc_type = DocumentType::from_path(&hit.symbol.location.file);
        let doc_boost = doc_type.boost_factor_for_intent(intent);
        let kind_boost = hit.symbol.kind.boost_factor_for_intent(intent);
        hit.score *= doc_boost * kind_boost;
    }

    // Re-sort by boosted score
    hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    hits.truncate(limit);

    Ok(hits)
}
```

### Improvement 2: Add `--full` Flag for Full File Content

**Priority:** HIGH
**Effort:** LOW
**Files:** `src/cli/mod.rs`, `src/cli/commands.rs`

**Implementation:**

```rust
// In Commands enum - add full flag
Search {
    query: String,
    #[arg(short, long, default_value = "10")]
    limit: usize,
    #[arg(long, help = "Show full file content instead of location pointers")]
    full: bool,
    // ... other flags
}
```

```rust
// In search_command() - handle full flag
if full {
    for hit in &hits {
        let path = &hit.symbol.location.file;
        println!("\n{}", "=".repeat(60));
        println!("FILE: {}", path.display());
        println!("{}", "=".repeat(60));

        match std::fs::read_to_string(path) {
            Ok(content) => {
                // Limit to 500 lines to avoid overwhelming output
                let lines: Vec<&str> = content.lines().take(500).collect();
                for (i, line) in lines.iter().enumerate() {
                    println!("{:4} | {}", i + 1, line);
                }
                if content.lines().count() > 500 {
                    println!("... (truncated, {} more lines)", content.lines().count() - 500);
                }
            }
            Err(e) => println!("Error reading file: {}", e),
        }
    }
} else {
    // Existing location pointer output
}
```

### Improvement 3: Stronger Definition Boosting

**Priority:** MEDIUM
**Effort:** LOW
**Files:** `src/indexer/symbols.rs`

**Implementation:**

```rust
// Update boost_factor_for_intent for FindDefinition
QueryIntent::FindDefinition => {
    match self {
        // Definition types - strong boost
        SymbolKind::Struct => base * 2.0,
        SymbolKind::Enum => base * 2.0,
        SymbolKind::Trait => base * 2.0,
        SymbolKind::TypeAlias => base * 1.8,

        // Implementations are usages, not definitions - reduce
        SymbolKind::Impl => base * 0.5,

        // Methods/functions are definitions too
        SymbolKind::Function => base * 1.5,
        SymbolKind::Method => base * 1.3,

        // Documentation is not a definition - heavily reduce
        SymbolKind::DocumentChunk => base * 0.2,

        _ => base,
    }
}
```

### Improvement 4: Module Structure Query Detection

**Priority:** MEDIUM
**Effort:** MEDIUM
**Files:** `src/retrieval/query/mod.rs`, `src/cli/commands.rs`

**Implementation:**

```rust
// Add structure query detection
pub fn is_structure_query(query: &str) -> Option<String> {
    let query_lower = query.to_lowercase();

    // Pattern: "modules in X" or "what modules are in X"
    if let Some(idx) = query_lower.find("modules in ") {
        let rest = &query[idx + 11..];
        let dir = rest.split_whitespace().next()?;
        return Some(dir.to_string());
    }

    // Pattern: "what's in X directory"
    if query_lower.contains("what's in") || query_lower.contains("what is in") {
        let words: Vec<&str> = query.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            if *word == "in" && i + 1 < words.len() {
                return Some(words[i + 1].to_string());
            }
        }
    }

    None
}

// Handle structure queries in search
if let Some(directory) = is_structure_query(query) {
    return search_mod_files(&directory);
}

fn search_mod_files(directory: &str) -> CommandResult {
    // Find all mod.rs files in the directory
    let pattern = format!("src/{}/**/mod.rs", directory);
    let files = glob::glob(&pattern)?;

    for file in files {
        println!("Module: {}", file.display());
        // Show pub mod declarations
        let content = std::fs::read_to_string(&file)?;
        for line in content.lines() {
            if line.starts_with("pub mod") || line.starts_with("mod ") {
                println!("  {}", line);
            }
        }
    }
    Ok(())
}
```

---

## Implementation Order

| Phase | Improvement | Impact | Effort | Files Changed |
|-------|-------------|--------|--------|---------------|
| 1 | Intent detection in basic search | HIGH | LOW | commands.rs, search.rs |
| 2 | Full file content (`--full`) | HIGH | LOW | mod.rs, commands.rs |
| 3 | Stronger definition boosting | MEDIUM | LOW | symbols.rs |
| 4 | Module structure queries | MEDIUM | MEDIUM | query/mod.rs, commands.rs |

---

## Testing Plan

After each improvement, test with these queries:

```bash
# Test intent detection (should return code, not docs)
rustean search "how does retrieval pipeline work"

# Test full file content
rustean search "BgeEmbedder struct" --full

# Test definition boosting
rustean search "RetrievalError definition"

# Test module structure
rustean search "modules in retrieval"
```

**Expected Results:**
- Intent detection: Source code files rank above documentation
- Full file: Complete file content displayed
- Definition: Enum/struct definition at #1
- Module structure: mod.rs files with pub mod declarations

---

## Additional Notes

### Daemon Socket Issue

During benchmarking, we discovered a daemon socket concurrency issue:
- Error: "Resource temporarily unavailable (os error 35)"
- Cause: Multiple parallel `--semantic` searches overwhelmed the socket
- Impact: Semantic search fails under concurrent load
- Recommendation: Add connection pooling or queue requests

This is a separate issue from retrieval quality and should be tracked separately.

### Existing Code to Leverage

The codebase already has most building blocks:
- `fallback_parse()` - Detects intent from query keywords
- `boost_factor_for_intent()` - Applies intent-aware boosts
- `DocumentType::from_path()` - Classifies files as source/doc/notes
- `ContextExpander` - Can extract code snippets (for --full enhancement)

The improvements mainly involve connecting these existing pieces to the basic search command.
