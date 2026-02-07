# Plan: Local LLM Documentation Generator

## Summary

Add a background documentation generator that uses a local LLM to create descriptions for all code symbols, storing both user comments and LLM-generated docs in the vector database for enhanced RAG retrieval.

---

## Current State (What Already Exists)

| Component | Status | Location |
|-----------|--------|----------|
| Text generation LLM | ✅ Exists | `src/retrieval/query/llm.rs` (Phi3Model) |
| GPU detection | ✅ Exists | `src/retrieval/models/device.rs` |
| Vector store | ✅ Exists | `src/retrieval/hybrid/vector_store.rs` |
| Symbol extraction | ✅ Exists | `src/indexer/parser.rs`, `symbols.rs` |
| File modification tracking | ✅ Exists | `src/indexer/state.rs` |
| Daemon architecture | ✅ Exists | `src/retrieval/daemon/server.rs` |
| Cross-references | ✅ Exists | `src/indexer/semantic.rs` (SemanticGraph) |

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         DAEMON (Background)                              │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────────────────┐  │
│  │ BgeEmbedder  │    │  Phi3Model   │    │    DocGenerator          │  │
│  │ (embeddings) │    │  (existing)  │    │    (NEW)                 │  │
│  └──────────────┘    └──────────────┘    └──────────────────────────┘  │
│         │                   │                        │                   │
│         │                   │                        │                   │
│         ▼                   ▼                        ▼                   │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                      DocStore (NEW)                               │  │
│  │  ┌────────────────────────────────────────────────────────────┐  │  │
│  │  │  symbol_id | user_comment | llm_doc | references | status  │  │  │
│  │  └────────────────────────────────────────────────────────────┘  │  │
│  │                              │                                    │  │
│  │                              ▼                                    │  │
│  │                    VectorStore (enhanced)                         │  │
│  │                    - code embedding                               │  │
│  │                    - doc embedding (NEW)                          │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         RETRIEVAL PIPELINE                               │
├─────────────────────────────────────────────────────────────────────────┤
│  1. Check DocStore.status == Ready                                       │
│  2. If ready: search code embeddings + doc embeddings                    │
│  3. If not ready: search code embeddings only (graceful degradation)     │
│  4. Return results with: code + user_comment + llm_doc + references      │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Data Model

### DocEntry (NEW)

```rust
/// Documentation entry for a code symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocEntry {
    /// unique identifier (hash of file_path + symbol_name + line)
    pub id: String,
    /// symbol name
    pub name: String,
    /// symbol kind (Function, Struct, Impl, etc.)
    pub kind: SymbolKind,
    /// file path
    pub file_path: PathBuf,
    /// line number
    pub line: usize,
    /// user-written doc comment (/// or //!)
    pub user_comment: Option<String>,
    /// LLM-generated documentation
    pub llm_doc: Option<String>,
    /// function/method signature
    pub signature: Option<String>,
    /// source code snippet (for context)
    pub code_snippet: String,
    /// places where this symbol is referenced
    pub references: Vec<ReferenceLocation>,
    /// cross-reference links to other symbols
    pub links: SymbolLinks,
    /// generation status
    pub status: DocStatus,
    /// last modification time of source file
    pub source_mtime: u64,
    /// embedding vector for the documentation
    pub doc_embedding: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceLocation {
    /// file where reference occurs
    pub file_path: PathBuf,
    /// line number
    pub line: usize,
    /// surrounding code context (3-5 lines)
    pub context: String,
    /// what kind of reference (call, import, type usage, etc.)
    pub ref_kind: ReferenceKind,
    /// module path (e.g., "retrieval::hybrid::embedding")
    pub module_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceKind {
    /// function/method call
    Call,
    /// type usage (in signature, field, etc.)
    TypeUsage,
    /// import/use statement
    Import,
    /// trait implementation
    TraitImpl,
    /// derive macro
    Derive,
    /// field access
    FieldAccess,
    /// external crate dependency
    ExternalCrate,
}

/// Cross-reference links between symbols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolLinks {
    /// symbols this one depends on (calls, uses types from)
    pub depends_on: Vec<String>,
    /// symbols that depend on this one
    pub depended_by: Vec<String>,
    /// parent module/struct/impl
    pub parent: Option<String>,
    /// child symbols (for modules, structs, impls)
    pub children: Vec<String>,
    /// related external crates
    pub external_deps: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DocStatus {
    /// not yet processed
    Pending,
    /// LLM is generating doc
    Generating,
    /// doc generation complete
    Ready,
    /// generation failed
    Failed,
}
```

### DocStore (NEW)

```rust
/// Storage for generated documentation
pub struct DocStore {
    /// all documentation entries
    entries: HashMap<String, DocEntry>,
    /// path for persistence
    store_path: PathBuf,
    /// overall generation status
    generation_complete: bool,
}
```

---

## Implementation Phases

### Phase 1: DocStore Module (Foundation)

**Files to create:**
- `src/retrieval/docgen/mod.rs` - module exports
- `src/retrieval/docgen/entry.rs` - DocEntry, DocStatus, ReferenceLocation
- `src/retrieval/docgen/store.rs` - DocStore implementation

**DocStore methods:**
```rust
impl DocStore {
    /// Create new store for project
    pub fn new(project_path: &Path) -> Self;

    /// Load from disk
    pub fn load(project_path: &Path) -> Result<Self>;

    /// Save to disk
    pub fn save(&self) -> Result<()>;

    /// Add or update entry
    pub fn upsert(&mut self, entry: DocEntry);

    /// Get entry by ID
    pub fn get(&self, id: &str) -> Option<&DocEntry>;

    /// Get all pending entries
    pub fn get_pending(&self) -> Vec<&DocEntry>;

    /// Check if all docs are ready
    pub fn is_ready(&self) -> bool;

    /// Get entries that need regeneration (file modified)
    pub fn get_stale(&self, current_mtimes: &HashMap<PathBuf, u64>) -> Vec<String>;

    /// Search docs by embedding
    pub fn search(&self, query_embedding: &[f32], limit: usize) -> Vec<&DocEntry>;
}
```

**Persistence:**
- Location: `.ch-index/docs.json`
- Format: JSON (same as other persistence)

---

### Phase 2: DocGenerator Module (LLM Integration)

**Files to create:**
- `src/retrieval/docgen/generator.rs` - DocGenerator
- `src/retrieval/docgen/prompts.rs` - prompt templates

**DocGenerator:**
```rust
/// Generates documentation using local LLM
pub struct DocGenerator {
    /// the LLM model (reuse Phi3Model or use smaller model)
    model: Option<DocLlm>,
    /// batch size for generation
    batch_size: usize,
}

impl DocGenerator {
    /// Create with default model
    pub fn new() -> Result<Self>;

    /// Create with specific model ID
    pub fn with_model(model_id: &str) -> Result<Self>;

    /// Generate doc for a single symbol
    pub fn generate(&mut self, entry: &mut DocEntry) -> Result<()>;

    /// Generate docs for batch of symbols
    pub fn generate_batch(&mut self, entries: &mut [DocEntry]) -> Result<usize>;

    /// Check if model is loaded
    pub fn is_ready(&self) -> bool;
}
```

**Prompt Template:**
```rust
pub const DOC_GENERATION_PROMPT: &str = r#"You are a code documentation assistant. Generate a clear, concise description for this {kind}.

Name: {name}
Signature: {signature}
Code:
```rust
{code_snippet}
```

Existing comment: {user_comment}

Write a 1-3 sentence description explaining:
1. What this {kind} does
2. Its purpose in the codebase
3. Key parameters/fields (if any)

Description:"#;
```

**Model Options (lightweight, fast):**
| Model | Size | Speed | Quality |
|-------|------|-------|---------|
| Phi-3-mini (existing) | 3.8B | ~20 tok/s | Good |
| Qwen2.5-Coder-1.5B | 1.5B | ~40 tok/s | Good for code |
| DeepSeek-Coder-1.3B | 1.3B | ~50 tok/s | Excellent for code |
| CodeGemma-2B | 2B | ~35 tok/s | Good |

**Recommendation:** Start with existing Phi3Model, add option to use smaller model later.

---

### Phase 3: Daemon Integration (Background Processing)

**Files to modify:**
- `src/retrieval/daemon/server.rs` - add DocGenerator to daemon
- `src/retrieval/daemon/protocol.rs` - add doc-related requests

**New daemon state:**
```rust
pub struct ModelDaemon {
    // ... existing fields ...

    /// documentation generator
    doc_generator: Option<DocGenerator>,
    /// documentation store per project
    doc_stores: HashMap<PathBuf, DocStore>,
    /// background generation task handle
    doc_gen_task: Option<JoinHandle<()>>,
}
```

**New protocol messages:**
```rust
pub enum DaemonRequest {
    // ... existing ...

    /// Start documentation generation for project
    StartDocGen { project_path: PathBuf },
    /// Get documentation status
    DocStatus { project_path: PathBuf },
    /// Get documentation for symbol
    GetDoc { project_path: PathBuf, symbol_id: String },
    /// Search documentation
    SearchDocs { project_path: PathBuf, query: String, limit: usize },
}

pub enum DaemonResponse {
    // ... existing ...

    /// Documentation generation status
    DocGenStatus {
        total: usize,
        completed: usize,
        pending: usize,
        is_ready: bool,
    },
    /// Documentation entry
    Doc(Option<DocEntry>),
    /// Documentation search results
    DocResults(Vec<DocEntry>),
}
```

**Background generation flow:**
```
1. Daemon receives StartDocGen request
2. Spawn background task:
   a. Load/create DocStore
   b. Get all symbols from IndexManager
   c. Create DocEntry for each symbol (with user_comment, references)
   d. For each pending entry:
      - Call DocGenerator.generate()
      - Update entry status
      - Save periodically (every 10 entries)
   e. Mark generation_complete = true
3. Respond to status queries while generating
```

---

### Phase 4: Comprehensive Cross-Referencing

**Goal:** Build a complete graph of symbol relationships.

**Data to extract:**
```
For each symbol:
├── depends_on: what it calls/uses
│   ├── internal functions called
│   ├── types used in signature/body
│   ├── traits implemented
│   └── external crate functions/types
├── depended_by: what uses it
│   ├── functions that call it
│   ├── structs that use it as field type
│   └── modules that import it
├── parent: containing module/struct/impl
├── children: nested items (for modules, impls)
└── external_deps: crates from Cargo.toml
```

**Integration with SemanticGraph:**
```rust
impl DocStore {
    /// Build cross-references from semantic graph
    pub fn build_links(&mut self, graph: &SemanticGraph, symbols: &[Symbol]) {
        for entry in self.entries.values_mut() {
            // Get all references TO this symbol
            if let Some(refs) = graph.get_references(&entry.name) {
                for ref_loc in refs {
                    entry.references.push(ReferenceLocation {
                        file_path: ref_loc.file.clone(),
                        line: ref_loc.line,
                        context: self.extract_context(&ref_loc, 3), // 3 lines context
                        ref_kind: classify_reference(&ref_loc),
                        module_path: get_module_path(&ref_loc.file),
                    });

                    // Add to depended_by
                    if let Some(caller) = find_containing_symbol(&ref_loc, symbols) {
                        entry.links.depended_by.push(caller);
                    }
                }
            }

            // Get symbols this one CALLS/USES
            if let Some(calls) = graph.get_outgoing_refs(&entry.name) {
                for call in calls {
                    entry.links.depends_on.push(call.name.clone());
                }
            }

            // External crate detection from imports
            entry.links.external_deps = extract_external_deps(&entry.code_snippet);
        }
    }
}
```

**Context extraction:**
```rust
/// Extract surrounding lines for context
fn extract_context(location: &CodeLocation, lines_around: usize) -> String {
    let content = fs::read_to_string(&location.file).ok()?;
    let lines: Vec<&str> = content.lines().collect();

    let start = location.line.saturating_sub(lines_around);
    let end = (location.line + lines_around).min(lines.len());

    lines[start..end].join("\n")
}
```

---

### Phase 5: File Modification Detection

**Files to modify:**
- `src/retrieval/docgen/store.rs` - add sync_with_index

**Integration with IndexState:**
```rust
impl DocStore {
    /// Sync with index state, mark stale entries
    pub fn sync_with_index(&mut self, index_state: &IndexState) {
        for (path, file_state) in &index_state.files {
            let mtime = file_state.mtime;

            // find all entries for this file
            for entry in self.entries.values_mut() {
                if entry.file_path == *path && entry.source_mtime < mtime {
                    // file was modified, mark for regeneration
                    entry.status = DocStatus::Pending;
                    entry.llm_doc = None;
                    entry.doc_embedding = None;
                }
            }
        }
    }
}
```

**Trigger conditions:**
1. **On daemon start**: Check all files, regenerate stale
2. **On index refresh**: Sync DocStore with IndexState
3. **On explicit request**: `ch-cli docs --regenerate`

---

### Phase 6: Pipeline Integration

**Files to modify:**
- `src/retrieval/agent/pipeline.rs` - integrate doc search
- `src/retrieval/hybrid/mod.rs` - add doc embeddings to search

**Enhanced search flow:**
```rust
impl RetrievalPipeline {
    pub fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        // 1. Check if docs are ready
        let docs_ready = self.doc_store.map(|s| s.is_ready()).unwrap_or(false);

        // 2. Embed query
        let query_embedding = self.embed_query(query)?;

        // 3. Search code (existing)
        let code_results = self.hybrid_search.search(&query_embedding, limit)?;

        // 4. Search docs (if ready)
        let doc_results = if docs_ready {
            self.doc_store.search(&query_embedding, limit)?
        } else {
            vec![]
        };

        // 5. Merge results with RRF
        let merged = self.fuse_results(code_results, doc_results)?;

        // 6. Enrich with doc content
        for result in &mut merged {
            if let Some(doc) = self.doc_store.get(&result.symbol_id) {
                result.user_comment = doc.user_comment.clone();
                result.llm_doc = doc.llm_doc.clone();
                result.references = doc.references.clone();
            }
        }

        Ok(merged)
    }
}
```

**SearchResult enhancement:**
```rust
pub struct SearchResult {
    // ... existing fields ...

    /// user-written documentation
    pub user_comment: Option<String>,
    /// LLM-generated documentation
    pub llm_doc: Option<String>,
    /// places where this symbol is used
    pub references: Vec<ReferenceLocation>,
}
```

---

### Phase 7: CLI Commands

**New commands:**
```bash
# Start doc generation (runs in background via daemon)
ch-cli docs generate

# Check generation status
ch-cli docs status

# View doc for specific symbol
ch-cli docs show <symbol_name>

# Regenerate all docs
ch-cli docs regenerate

# Search docs
ch-cli docs search "authentication flow"
```

---

## File Summary

| File | Action | Description |
|------|--------|-------------|
| `src/retrieval/docgen/mod.rs` | NEW | Module exports |
| `src/retrieval/docgen/entry.rs` | NEW | DocEntry, DocStatus, ReferenceKind, SymbolLinks |
| `src/retrieval/docgen/store.rs` | NEW | DocStore implementation |
| `src/retrieval/docgen/generator.rs` | NEW | DocGenerator with LLM |
| `src/retrieval/docgen/prompts.rs` | NEW | Prompt templates |
| `src/retrieval/docgen/linker.rs` | NEW | Cross-reference link builder |
| `src/retrieval/daemon/server.rs` | MODIFY | Add doc generation |
| `src/retrieval/daemon/protocol.rs` | MODIFY | Add doc requests |
| `src/retrieval/agent/pipeline.rs` | MODIFY | Integrate doc search |
| `src/retrieval/hybrid/mod.rs` | MODIFY | Add doc embeddings |
| `src/cli/commands.rs` | MODIFY | Add docs commands |

---

## Performance Considerations

### Generation Speed
| Scenario | Estimate |
|----------|----------|
| 300 symbols, Phi-3 @ 20 tok/s | ~5-8 minutes |
| 300 symbols, 1.5B model @ 40 tok/s | ~3-4 minutes |
| Incremental (10 changed files) | ~30 seconds |

### Memory Usage
| Component | RAM |
|-----------|-----|
| Phi-3 model | ~4GB |
| 1.5B model | ~2GB |
| DocStore (1000 entries) | ~10MB |
| Doc embeddings (1000 × 384) | ~1.5MB |

### Disk Usage
| Component | Size |
|-----------|------|
| docs.json (1000 entries) | ~2-5MB |
| doc_vectors.json | ~2MB |

---

## Success Criteria

1. **Functional:**
   - [ ] Docs generated for all functions, structs, impls, enums, traits
   - [ ] User comments preserved alongside LLM docs
   - [ ] References tracked for each symbol
   - [ ] Regeneration on file modification
   - [ ] Graceful degradation when docs not ready

2. **Performance:**
   - [ ] Background generation doesn't block queries
   - [ ] Incremental updates < 1 minute for small changes
   - [ ] Search includes docs when ready

3. **Integration:**
   - [ ] Works with existing daemon architecture
   - [ ] Uses existing GPU detection
   - [ ] Stores in .ch-index alongside other data

---

## Implementation Order

1. **Phase 1** (DocStore) - foundation types and storage
2. **Phase 2** (DocGenerator) - LLM integration for doc generation
3. **Phase 3** (Daemon integration) - background processing
4. **Phase 4** (Cross-referencing) - build complete symbol graph
5. **Phase 5** (File modification) - incremental updates
6. **Phase 6** (Pipeline integration) - use docs in search
7. **Phase 7** (CLI commands) - user interface

---

## User Decisions

1. **Model choice**: ✅ Start with Phi-3 (existing)
2. **Scope**: ✅ Generate docs for ALL symbol types (functions, structs, enums, traits, impls, methods, constants, etc.)
3. **References depth**: ✅ Include surrounding code context (not just file:line)
4. **Cross-references**: ✅ Include links between folders, functions, external crates - as complete as possible
