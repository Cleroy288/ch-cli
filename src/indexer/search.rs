//! Tantivy-based search engine for the semantic indexer.
//!
//! This module provides full-text search capabilities for indexed symbols,
//! including fuzzy search, filtering by kind, and ranked results.

use std::path::{Path, PathBuf};

use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery, BoostQuery, FuzzyTermQuery, Occur, QueryParser, TermQuery};
use tantivy::schema::*;
use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy, TantivyDocument, Term};

use crate::indexer::symbols::{CodeLocation, DocumentType, Symbol, SymbolKind, Visibility};

/// Error type for search operations
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("Tantivy error: {0}")]
    Tantivy(#[from] tantivy::TantivyError),

    #[error("Query parse error: {0}")]
    QueryParse(#[from] tantivy::query::QueryParserError),

    #[error("Index not found at {0}")]
    IndexNotFound(PathBuf),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Schema field not found: {0}")]
    FieldNotFound(String),
}

pub type SearchResult<T> = std::result::Result<T, SearchError>;

/// A search result with score and symbol information
#[derive(Debug, Clone)]
pub struct SearchHit {
    /// The matched symbol
    pub symbol: Symbol,
    /// Relevance score (higher is better)
    pub score: f32,
}

/// Field handles for the search schema
#[derive(Clone)]
struct SchemaFields {
    symbol_name: Field,
    symbol_kind: Field,
    file_path: Field,
    line: Field,
    column: Field,
    visibility: Field,
    signature: Field,
    fqn: Field,
    parent: Field,
    content: Field,
    document_type: Field, // document type for boost scoring (SourceCode, Documentation, Notes, etc.)
}

impl SchemaFields {
    /// Extract all field handles from the schema
    fn from_schema(schema: &Schema) -> SearchResult<Self> {
        Ok(Self {
            symbol_name: schema
                .get_field("symbol_name")
                .map_err(|_| SearchError::FieldNotFound("symbol_name".to_string()))?,
            symbol_kind: schema
                .get_field("symbol_kind")
                .map_err(|_| SearchError::FieldNotFound("symbol_kind".to_string()))?,
            file_path: schema
                .get_field("file_path")
                .map_err(|_| SearchError::FieldNotFound("file_path".to_string()))?,
            line: schema
                .get_field("line")
                .map_err(|_| SearchError::FieldNotFound("line".to_string()))?,
            column: schema
                .get_field("column")
                .map_err(|_| SearchError::FieldNotFound("column".to_string()))?,
            visibility: schema
                .get_field("visibility")
                .map_err(|_| SearchError::FieldNotFound("visibility".to_string()))?,
            signature: schema
                .get_field("signature")
                .map_err(|_| SearchError::FieldNotFound("signature".to_string()))?,
            fqn: schema
                .get_field("fqn")
                .map_err(|_| SearchError::FieldNotFound("fqn".to_string()))?,
            parent: schema
                .get_field("parent")
                .map_err(|_| SearchError::FieldNotFound("parent".to_string()))?,
            content: schema
                .get_field("content")
                .map_err(|_| SearchError::FieldNotFound("content".to_string()))?,
            document_type: schema
                .get_field("document_type")
                .map_err(|_| SearchError::FieldNotFound("document_type".to_string()))?,
        })
    }
}

/// Build the Tantivy schema for symbol indexing
fn build_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    // Primary search field - tokenized for full-text search
    schema_builder.add_text_field("symbol_name", TEXT | STORED);

    // Filterable fields - not tokenized
    schema_builder.add_text_field("symbol_kind", STRING | STORED);
    schema_builder.add_text_field("file_path", STRING | STORED);
    schema_builder.add_text_field("visibility", STRING | STORED);

    // Numeric fields for location
    schema_builder.add_u64_field("line", STORED | INDEXED);
    schema_builder.add_u64_field("column", STORED | INDEXED);

    // Additional metadata
    schema_builder.add_text_field("signature", TEXT | STORED);
    schema_builder.add_text_field("fqn", TEXT | STORED);
    schema_builder.add_text_field("parent", STRING | STORED);

    // Content field for full-text search on documentation
    schema_builder.add_text_field("content", TEXT | STORED);

    // Document type field for boost scoring (SourceCode, Documentation, Notes, etc.)
    schema_builder.add_text_field("document_type", STRING | STORED);

    schema_builder.build()
}

/// The main search index for symbols
pub struct SearchIndex {
    index: Index,
    #[allow(dead_code)]
    schema: Schema,
    fields: SchemaFields,
    index_path: Option<PathBuf>,
}

impl SearchIndex {
    /// Create a new in-memory search index
    pub fn in_memory() -> SearchResult<Self> {
        let schema = build_schema();
        let index = Index::create_in_ram(schema.clone());
        let fields = SchemaFields::from_schema(&schema)?;

        Ok(Self {
            index,
            schema,
            fields,
            index_path: None,
        })
    }

    /// Create or open a persistent search index at the given path
    pub fn open_or_create<P: AsRef<Path>>(path: P) -> SearchResult<Self> {
        let path = path.as_ref();
        std::fs::create_dir_all(path)?;

        let schema = build_schema();
        let index = if path.join("meta.json").exists() {
            Index::open_in_dir(path)?
        } else {
            Index::create_in_dir(path, schema.clone())?
        };

        let fields = SchemaFields::from_schema(&schema)?;

        Ok(Self {
            index,
            schema,
            fields,
            index_path: Some(path.to_path_buf()),
        })
    }

    /// Get an index writer for adding documents
    pub fn writer(&self, heap_size: usize) -> SearchResult<IndexWriter> {
        Ok(self.index.writer(heap_size)?)
    }

    /// Get an index reader for searching
    pub fn reader(&self) -> SearchResult<IndexReader> {
        Ok(self.index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?)
    }

    /// Index a batch of symbols
    pub fn index_symbols(&self, symbols: &[Symbol]) -> SearchResult<usize> {
        let mut writer = self.writer(50_000_000)?; // 50MB heap
        let count = symbols.len();

        for symbol in symbols {
            let doc = self.symbol_to_doc(symbol);
            writer.add_document(doc)?;
        }

        writer.commit()?;
        Ok(count)
    }

    /// Convert a Symbol to a Tantivy document
    fn symbol_to_doc(&self, symbol: &Symbol) -> TantivyDocument {
        let mut doc = TantivyDocument::new(); // tantivy document to populate

        doc.add_text(self.fields.symbol_name, &symbol.name);
        doc.add_text(self.fields.symbol_kind, &symbol.kind.to_string());
        doc.add_text(
            self.fields.file_path,
            symbol.location.file.to_string_lossy(),
        );
        doc.add_u64(self.fields.line, symbol.location.line as u64);
        doc.add_u64(self.fields.column, symbol.location.column as u64);
        doc.add_text(self.fields.visibility, &symbol.visibility.to_string());

        // Add document type based on file path
        let doc_type = DocumentType::from_path(&symbol.location.file);
        let doc_type_str = match doc_type {
            DocumentType::SourceCode => "SourceCode",
            DocumentType::Documentation => "Documentation",
            DocumentType::Notes => "Notes",
            DocumentType::Benchmark => "Benchmark",
            DocumentType::Test => "Test",
        };
        doc.add_text(self.fields.document_type, doc_type_str);

        if let Some(ref sig) = symbol.signature {
            doc.add_text(self.fields.signature, sig);
        }
        if let Some(ref fqn) = symbol.fqn {
            doc.add_text(self.fields.fqn, fqn);
        }
        if let Some(ref parent) = symbol.parent {
            doc.add_text(self.fields.parent, parent);
        }
        if let Some(ref content) = symbol.content {
            doc.add_text(self.fields.content, content);
        }

        doc
    }

    /// Convert a Tantivy document back to a Symbol
    fn doc_to_symbol(&self, doc: &TantivyDocument) -> Option<Symbol> {
        let name = doc
            .get_first(self.fields.symbol_name)?
            .as_str()?
            .to_string();
        let kind_str = doc.get_first(self.fields.symbol_kind)?.as_str()?;
        let file_path = doc.get_first(self.fields.file_path)?.as_str()?;
        let line = doc.get_first(self.fields.line)?.as_u64()? as usize;
        let column = doc.get_first(self.fields.column)?.as_u64()? as usize;
        let visibility_str = doc.get_first(self.fields.visibility)?.as_str()?;

        let kind = parse_symbol_kind(kind_str)?;
        let visibility = parse_visibility(visibility_str);

        let location = CodeLocation::new(PathBuf::from(file_path), line, column, 0, 0);

        let mut symbol = Symbol::new(name, kind, location).with_visibility(visibility);

        // Optional fields
        if let Some(val) = doc.get_first(self.fields.signature) {
            if let Some(s) = val.as_str() {
                if !s.is_empty() {
                    symbol = symbol.with_signature(s.to_string());
                }
            }
        }
        if let Some(val) = doc.get_first(self.fields.parent) {
            if let Some(s) = val.as_str() {
                if !s.is_empty() {
                    symbol = symbol.with_parent(s.to_string());
                }
            }
        }
        if let Some(val) = doc.get_first(self.fields.content) {
            if let Some(s) = val.as_str() {
                if !s.is_empty() {
                    symbol = symbol.with_content(s.to_string());
                }
            }
        }

        Some(symbol)
    }

    /// Search for symbols by name and content (full-text search)
    pub fn search(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>> {
        let reader = self.reader()?; // index reader for searching
        let searcher = reader.searcher(); // searcher instance

        // Search in both symbol_name and content fields
        let query_parser = QueryParser::for_index(
            &self.index,
            vec![self.fields.symbol_name, self.fields.content],
        );
        let query = query_parser.parse_query(query)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new(); // results to return
        for (score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            if let Some(symbol) = self.doc_to_symbol(&doc) {
                results.push(SearchHit { symbol, score });
            }
        }

        Ok(results)
    }

    /// Search with document type boosting applied at query time
    /// Source code results are boosted higher than documentation/notes
    pub fn search_with_boost(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>> {
        let reader = self.reader()?; // index reader for searching
        let searcher = reader.searcher(); // searcher instance

        // Parse base query for symbol_name and content fields
        let query_parser = QueryParser::for_index(
            &self.index,
            vec![self.fields.symbol_name, self.fields.content],
        );
        let base_query = query_parser.parse_query(query)?;

        // Create boost query for SourceCode documents (highest priority)
        let source_term = Term::from_field_text(self.fields.document_type, "SourceCode");
        let source_query = TermQuery::new(source_term, IndexRecordOption::Basic);

        // Combine: base query + boosted source code query
        // Documents matching both base query AND being SourceCode get additional 50% boost
        let boosted_query = BooleanQuery::new(vec![
            (Occur::Must, Box::new(base_query.box_clone())),
            (Occur::Should, Box::new(BoostQuery::new(
                Box::new(BooleanQuery::new(vec![
                    (Occur::Must, Box::new(base_query)),
                    (Occur::Must, Box::new(source_query)),
                ])),
                0.5, // additional 50% boost for source code matches
            ))),
        ]);

        let top_docs = searcher.search(&boosted_query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new(); // results to return
        for (score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            if let Some(symbol) = self.doc_to_symbol(&doc) {
                results.push(SearchHit { symbol, score });
            }
        }

        Ok(results)
    }

    /// Fuzzy search for symbols (handles typos)
    pub fn fuzzy_search(
        &self,
        term: &str,
        distance: u8,
        limit: usize,
    ) -> SearchResult<Vec<SearchHit>> {
        let reader = self.reader()?;
        let searcher = reader.searcher();

        let term = tantivy::Term::from_field_text(self.fields.symbol_name, term);
        let query = FuzzyTermQuery::new(term, distance, true);

        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            if let Some(symbol) = self.doc_to_symbol(&doc) {
                results.push(SearchHit { symbol, score });
            }
        }

        Ok(results)
    }

    /// Search for symbols of a specific kind
    pub fn search_by_kind(&self, kind: SymbolKind, limit: usize) -> SearchResult<Vec<SearchHit>> {
        let reader = self.reader()?;
        let searcher = reader.searcher();

        let term = tantivy::Term::from_field_text(self.fields.symbol_kind, &kind.to_string());
        let query = tantivy::query::TermQuery::new(term, IndexRecordOption::Basic);

        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            if let Some(symbol) = self.doc_to_symbol(&doc) {
                results.push(SearchHit { symbol, score });
            }
        }

        Ok(results)
    }

    /// Get the total number of indexed documents
    pub fn num_docs(&self) -> SearchResult<u64> {
        let reader = self.reader()?;
        Ok(reader.searcher().num_docs())
    }

    /// Clear all documents from the index
    pub fn clear(&self) -> SearchResult<()> {
        let mut writer = self.writer(50_000_000)?;
        writer.delete_all_documents()?;
        writer.commit()?;
        Ok(())
    }

    /// Delete all documents for a specific file (for incremental updates)
    pub fn delete_file(&self, file_path: &Path) -> SearchResult<()> {
        let mut writer = self.writer(50_000_000)?;
        let term = tantivy::Term::from_field_text(
            self.fields.file_path,
            &file_path.to_string_lossy(),
        );
        writer.delete_term(term);
        writer.commit()?;
        Ok(())
    }

    /// Update symbols for a file: delete existing symbols and add new ones
    pub fn update_file_symbols(&self, file_path: &Path, symbols: &[Symbol]) -> SearchResult<usize> {
        let mut writer = self.writer(50_000_000)?;

        // Delete existing documents for this file
        let term = tantivy::Term::from_field_text(
            self.fields.file_path,
            &file_path.to_string_lossy(),
        );
        writer.delete_term(term);

        // Add new symbols
        let count = symbols.len();
        for symbol in symbols {
            let doc = self.symbol_to_doc(symbol);
            writer.add_document(doc)?;
        }

        writer.commit()?;
        Ok(count)
    }

    /// Batch update symbols for multiple files (much faster than calling update_file_symbols repeatedly)
    ///
    /// This method:
    /// 1. Deletes symbols for all specified files to delete
    /// 2. Adds symbols for all file results
    /// 3. Commits once at the end
    pub fn batch_update(
        &self,
        files_to_delete: &[PathBuf],
        file_results: &[(PathBuf, Vec<Symbol>)],
    ) -> SearchResult<usize> {
        let mut writer = self.writer(50_000_000)?;
        let mut total_count = 0;

        // Delete symbols for deleted/modified files
        for file_path in files_to_delete {
            let term = tantivy::Term::from_field_text(
                self.fields.file_path,
                &file_path.to_string_lossy(),
            );
            writer.delete_term(term);
        }

        // Add new symbols for all files
        for (file_path, symbols) in file_results {
            // Delete existing symbols for this file first
            let term = tantivy::Term::from_field_text(
                self.fields.file_path,
                &file_path.to_string_lossy(),
            );
            writer.delete_term(term);

            // Add new symbols
            for symbol in symbols {
                let doc = self.symbol_to_doc(symbol);
                writer.add_document(doc)?;
            }
            total_count += symbols.len();
        }

        // Single commit at the end
        writer.commit()?;
        Ok(total_count)
    }

    /// Get the index path (if persistent)
    pub fn path(&self) -> Option<&Path> {
        self.index_path.as_deref()
    }
}

/// Parse a SymbolKind from its string representation
fn parse_symbol_kind(s: &str) -> Option<SymbolKind> {
    match s {
        "fn" => Some(SymbolKind::Function),
        "method" => Some(SymbolKind::Method),
        "struct" => Some(SymbolKind::Struct),
        "enum" => Some(SymbolKind::Enum),
        "trait" => Some(SymbolKind::Trait),
        "impl" => Some(SymbolKind::Impl),
        "const" => Some(SymbolKind::Constant),
        "static" => Some(SymbolKind::Static),
        "type" => Some(SymbolKind::TypeAlias),
        "mod" => Some(SymbolKind::Module),
        "macro" => Some(SymbolKind::Macro),
        "variant" => Some(SymbolKind::EnumVariant),
        "field" => Some(SymbolKind::Field),
        "doc" => Some(SymbolKind::DocumentChunk),
        _ => None,
    }
}

/// Parse a Visibility from its string representation
fn parse_visibility(s: &str) -> Visibility {
    match s {
        "pub" => Visibility::Public,
        "pub(crate)" => Visibility::PublicCrate,
        "pub(super)" => Visibility::PublicSuper,
        _ => Visibility::Private,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_symbol(name: &str, kind: SymbolKind) -> Symbol {
        Symbol::new(
            name.to_string(),
            kind,
            CodeLocation::new(PathBuf::from("test.rs"), 1, 0, 0, 10),
        )
    }

    #[test]
    fn test_index_and_search() {
        let index = SearchIndex::in_memory().unwrap();

        let symbols = vec![
            create_test_symbol("handle_events", SymbolKind::Function),
            create_test_symbol("process_input", SymbolKind::Function),
            create_test_symbol("EventHandler", SymbolKind::Struct),
        ];

        index.index_symbols(&symbols).unwrap();

        // Search for "handle"
        let results = index.search("handle", 10).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].symbol.name, "handle_events");
    }

    #[test]
    fn test_fuzzy_search() {
        let index = SearchIndex::in_memory().unwrap();

        let symbols = vec![create_test_symbol("function_name", SymbolKind::Function)];
        index.index_symbols(&symbols).unwrap();

        // Search with typo
        let results = index.fuzzy_search("functon", 2, 10).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_by_kind() {
        let index = SearchIndex::in_memory().unwrap();

        let symbols = vec![
            create_test_symbol("my_func", SymbolKind::Function),
            create_test_symbol("MyStruct", SymbolKind::Struct),
            create_test_symbol("another_func", SymbolKind::Function),
        ];
        index.index_symbols(&symbols).unwrap();

        let results = index.search_by_kind(SymbolKind::Function, 10).unwrap();
        assert_eq!(results.len(), 2);
    }

    /// Helper to create a test symbol at a specific file path
    fn create_test_symbol_with_path(name: &str, kind: SymbolKind, path: &str) -> Symbol {
        Symbol::new(
            name.to_string(),
            kind,
            CodeLocation::new(PathBuf::from(path), 1, 0, 0, 10),
        )
    }

    /// Test that document_type field is correctly indexed and stored
    #[test]
    fn test_document_type_field_indexed() {
        let index = SearchIndex::in_memory().unwrap();

        // Create symbols from different document types
        let symbols = vec![
            create_test_symbol_with_path("source_func", SymbolKind::Function, "src/main.rs"),
            create_test_symbol_with_path("doc_func", SymbolKind::Function, "doc/api.md"),
            create_test_symbol_with_path("notes_func", SymbolKind::Function, "notes/impl.md"),
            create_test_symbol_with_path("test_func", SymbolKind::Function, "tests/unit.rs"),
        ];

        index.index_symbols(&symbols).unwrap();

        // Verify all symbols are indexed
        let num_docs = index.num_docs().unwrap();
        assert_eq!(num_docs, 4);

        // Search should return all matching symbols
        let results = index.search("func", 10).unwrap();
        assert_eq!(results.len(), 4);
    }

    /// Test that search_with_boost prefers source code over documentation/notes
    #[test]
    fn test_search_with_boost_prefers_source_code() {
        let index = SearchIndex::in_memory().unwrap();

        // Create symbols with same name but different document types
        let symbols = vec![
            create_test_symbol_with_path("handle_data", SymbolKind::Function, "notes/impl.md"),
            create_test_symbol_with_path("handle_data", SymbolKind::Function, "src/handler.rs"),
            create_test_symbol_with_path("handle_data", SymbolKind::Function, "doc/api.md"),
        ];

        index.index_symbols(&symbols).unwrap();

        // Boosted search should rank source code higher
        let results = index.search_with_boost("handle_data", 10).unwrap();
        assert!(!results.is_empty());

        // First result should be from source code (src/handler.rs)
        let first_path = results[0].symbol.location.file.to_string_lossy();
        assert!(
            first_path.contains("src/"),
            "Expected source code file first, got: {}",
            first_path
        );
    }
}

