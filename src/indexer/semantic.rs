//! Semantic analysis module for name resolution.
//!
//! This module provides:
//! - Definition tracking (where symbols are declared)
//! - Reference tracking (where symbols are used)
//! - "Go to definition" functionality
//! - "Find all references" functionality
//!
//! This is a simplified implementation that can be enhanced with
//! full stack-graphs support in the future.

use std::path::{Path, PathBuf};

use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};

use crate::indexer::symbols::{CodeLocation, Symbol, SymbolKind};

/// A reference to a symbol (where it's used)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolReference {
    /// The name being referenced
    pub name: String,
    /// Location of the reference
    pub location: CodeLocation,
    /// The kind of reference (if known)
    pub context: ReferenceContext,
}

/// Context in which a reference appears
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReferenceContext {
	/// Function/method call
	Call,
	/// Type annotation
	Type,
	/// Field access
	FieldAccess,
	/// Import/use statement
	Import,
	/// Variable/identifier usage
	Identifier,
	/// Unknown context
	Unknown,

	// More specific type contexts
	/// Used as field type: `field: MyType`
	FieldType,
	/// Used as return type: `-> MyType`
	ReturnType,
	/// Used as parameter type: `fn foo(x: MyType)`
	ParameterType,
	/// Used as generic argument: `Vec<MyType>`
	GenericArg,
	/// Used as trait bound: `T: MyTrait`
	TraitBound,
	/// Target of impl: `impl MyTrait for X`
	ImplTarget,
}

impl ReferenceContext {
	/// Check if this is a type-related context
	pub fn is_type_usage(&self) -> bool {
		matches!(
			self,
			Self::Type
				| Self::FieldType
				| Self::ReturnType
				| Self::ParameterType
				| Self::GenericArg
				| Self::TraitBound
				| Self::ImplTarget
		)
	}
}

/// A definition with its location and metadata
#[derive(Debug, Clone)]
pub struct Definition {
    /// The symbol that was defined
    pub symbol: Symbol,
    /// Fully qualified name (e.g., "module::struct::method")
    pub fqn: String,
    /// The scope this definition belongs to
    pub scope: Option<String>,
}

/// Result of resolving a reference
#[derive(Debug, Clone)]
pub struct ResolutionResult {
    /// The reference that was resolved
    pub reference: SymbolReference,
    /// Possible definitions this reference could refer to
    pub definitions: Vec<Definition>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
}

/// The semantic graph for a project
pub struct SemanticGraph {
    /// All definitions indexed by name
    definitions_by_name: FxHashMap<String, Vec<Definition>>,
    /// All definitions indexed by file
    definitions_by_file: FxHashMap<PathBuf, Vec<Definition>>,
    /// All references indexed by name
    references_by_name: FxHashMap<String, Vec<SymbolReference>>,
    /// All references indexed by file
    references_by_file: FxHashMap<PathBuf, Vec<SymbolReference>>,
    /// Scope hierarchy (child -> parent)
    scope_parents: FxHashMap<String, String>,
}

impl SemanticGraph {
    /// Create a new empty semantic graph
    pub fn new() -> Self {
        Self {
            definitions_by_name: FxHashMap::default(),
            definitions_by_file: FxHashMap::default(),
            references_by_name: FxHashMap::default(),
            references_by_file: FxHashMap::default(),
            scope_parents: FxHashMap::default(),
        }
    }

    /// Add symbols from parsing as definitions
    pub fn add_symbols(&mut self, symbols: &[Symbol]) {
        for symbol in symbols {
            self.add_definition(symbol.clone());
        }
    }

    /// Add a single definition
    pub fn add_definition(&mut self, symbol: Symbol) {
        let fqn = self.compute_fqn(&symbol);
        let scope = symbol.parent.clone();
        let file = symbol.location.file.clone();

        let definition = Definition {
            symbol: symbol.clone(),
            fqn: fqn.clone(),
            scope,
        };

        // Index by name
        self.definitions_by_name
            .entry(symbol.name.clone())
            .or_default()
            .push(definition.clone());

        // Index by file
        self.definitions_by_file
            .entry(file)
            .or_default()
            .push(definition);

        // Track scope hierarchy
        if let Some(ref parent) = symbol.parent {
            self.scope_parents.insert(fqn, parent.clone());
        }
    }

    /// Add a reference
    pub fn add_reference(&mut self, reference: SymbolReference) {
        let file = reference.location.file.clone();
        let name = reference.name.clone();

        self.references_by_name
            .entry(name)
            .or_default()
            .push(reference.clone());

        self.references_by_file
            .entry(file)
            .or_default()
            .push(reference);
    }

    /// Compute fully qualified name for a symbol
    fn compute_fqn(&self, symbol: &Symbol) -> String {
        if let Some(ref parent) = symbol.parent {
            format!("{}::{}", parent, symbol.name)
        } else {
            symbol.name.clone()
        }
    }

    /// Find the definition(s) for a symbol name
    pub fn find_definitions(&self, name: &str) -> Vec<&Definition> {
        self.definitions_by_name
            .get(name)
            .map(|defs| defs.iter().collect())
            .unwrap_or_default()
    }

    /// Find all references to a symbol name
    pub fn find_references(&self, name: &str) -> Vec<&SymbolReference> {
        self.references_by_name
            .get(name)
            .map(|refs| refs.iter().collect())
            .unwrap_or_default()
    }

	/// Find where a type is used in signatures, fields, etc.
	/// Returns references that are in type-related contexts
	pub fn find_type_usages(&self, type_name: &str) -> Vec<SymbolReference> {
		self.references_by_name
			.get(type_name)
			.map(|refs| {
				refs.iter()
					.filter(|r| r.context.is_type_usage())
					.cloned()
					.collect()
			})
			.unwrap_or_default()
	}

	/// Find all implementations of a trait
	/// Searches for impl blocks whose signature contains the trait name
	pub fn find_trait_implementations(&self, trait_name: &str) -> Vec<Definition> {
		self.definitions_by_name
			.values()
			.flatten()
			.filter(|def| {
				def.symbol.kind == SymbolKind::Impl
					&& def
						.symbol
						.signature
						.as_ref()
						.map(|s| s.contains(trait_name))
						.unwrap_or(false)
			})
			.cloned()
			.collect()
	}

	/// Find all usages grouped by context type
	/// Returns a map from ReferenceContext to list of references
	pub fn find_usages_by_context(
		&self,
		name: &str,
	) -> std::collections::HashMap<ReferenceContext, Vec<SymbolReference>> {
		let mut grouped: std::collections::HashMap<ReferenceContext, Vec<SymbolReference>> =
			std::collections::HashMap::new(); // map from context to list of references

		if let Some(refs) = self.references_by_name.get(name) {
			for reference in refs {
				grouped
					.entry(reference.context)
					.or_default()
					.push(reference.clone());
			}
		}

		grouped
	}

    /// Find definition at a specific location (for "go to definition")
    pub fn definition_at(&self, file: &Path, line: usize) -> Option<&Definition> {
        self.definitions_by_file.get(file).and_then(|defs| {
            defs.iter().find(|d| d.symbol.location.line == line)
        })
    }

    /// Find all definitions in a file
    pub fn definitions_in_file(&self, file: &Path) -> Vec<&Definition> {
        self.definitions_by_file
            .get(file)
            .map(|defs| defs.iter().collect())
            .unwrap_or_default()
    }

    /// Find all references in a file
    pub fn references_in_file(&self, file: &Path) -> Vec<&SymbolReference> {
        self.references_by_file
            .get(file)
            .map(|refs| refs.iter().collect())
            .unwrap_or_default()
    }

    /// Resolve a reference to its possible definitions
    pub fn resolve(&self, reference: &SymbolReference) -> ResolutionResult {
        let definitions: Vec<Definition> = self
            .find_definitions(&reference.name)
            .into_iter()
            .cloned()
            .collect();

        let confidence = if definitions.len() == 1 {
            1.0
        } else if definitions.is_empty() {
            0.0
        } else {
            // Multiple definitions - lower confidence
            0.5 / definitions.len() as f32 + 0.5
        };

        ResolutionResult {
            reference: reference.clone(),
            definitions,
            confidence,
        }
    }

    /// Validate that a reference actually refers to a specific target symbol
    /// Returns true if the reference resolves to the target definition
    pub fn validate_reference(&self, reference: &SymbolReference, target: &Symbol) -> bool {
        // name must match
        if reference.name != target.name {
            return false;
        }

        // resolve to find all possible definitions
        let resolution = self.resolve(reference);

        // if no definitions found, accept the reference (external symbol)
        if resolution.definitions.is_empty() {
            return true;
        }

        // if only one definition, accept (unambiguous)
        if resolution.definitions.len() == 1 {
            return true;
        }

        // multiple definitions - check if any matches the target location
        // be lenient: match by file only if line doesn't match exactly
        resolution.definitions.iter().any(|def| {
            // exact match
            if def.symbol.location.file == target.location.file
                && def.symbol.location.line == target.location.line
            {
                return true;
            }

            // same file match (for fields/methods that might have different line tracking)
            if def.symbol.location.file == target.location.file
                && def.symbol.kind == target.kind
            {
                return true;
            }

            false
        })
    }

    /// Filter references to only those that resolve to a specific target
    pub fn validated_references(&self, target: &Symbol) -> Vec<&SymbolReference> {
        self.find_references(&target.name)
            .into_iter()
            .filter(|r| self.validate_reference(r, target))
            .collect()
    }

    /// Find all usages of a symbol (definition + references)
    pub fn find_all_usages(&self, name: &str) -> AllUsages {
        let definitions: Vec<CodeLocation> = self
            .find_definitions(name)
            .into_iter()
            .map(|d| d.symbol.location.clone())
            .collect();

        let references: Vec<CodeLocation> = self
            .find_references(name)
            .into_iter()
            .map(|r| r.location.clone())
            .collect();

        AllUsages {
            name: name.to_string(),
            definitions,
            references,
        }
    }

    /// Get statistics about the semantic graph
    pub fn stats(&self) -> SemanticStats {
        SemanticStats {
            total_definitions: self.definitions_by_name.values().map(|v| v.len()).sum(),
            total_references: self.references_by_name.values().map(|v| v.len()).sum(),
            unique_symbols: self.definitions_by_name.len(),
            files_analyzed: self.definitions_by_file.len(),
        }
    }

    /// Get all unique symbol names
    pub fn all_symbol_names(&self) -> Vec<&String> {
        self.definitions_by_name.keys().collect()
    }

    /// Find definitions by kind
    pub fn find_by_kind(&self, kind: SymbolKind) -> Vec<&Definition> {
        self.definitions_by_name
            .values()
            .flatten()
            .filter(|d| d.symbol.kind == kind)
            .collect()
    }
}

impl Default for SemanticGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// All usages of a symbol
#[derive(Debug, Clone)]
pub struct AllUsages {
    /// The symbol name
    pub name: String,
    /// Where the symbol is defined
    pub definitions: Vec<CodeLocation>,
    /// Where the symbol is referenced
    pub references: Vec<CodeLocation>,
}

impl AllUsages {
    /// Total number of usages (definitions + references)
    pub fn total(&self) -> usize {
        self.definitions.len() + self.references.len()
    }
}

/// Statistics about the semantic graph
#[derive(Debug, Clone)]
pub struct SemanticStats {
    /// Total number of definitions
    pub total_definitions: usize,
    /// Total number of references
    pub total_references: usize,
    /// Number of unique symbol names
    pub unique_symbols: usize,
    /// Number of files analyzed
    pub files_analyzed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_symbol(name: &str, kind: SymbolKind, file: &str, line: usize) -> Symbol {
        Symbol::new(
            name.to_string(),
            kind,
            CodeLocation::new(PathBuf::from(file), line, 0, 0, 10),
        )
    }

    #[test]
    fn test_add_and_find_definitions() {
        let mut graph = SemanticGraph::new();

        let symbol = create_test_symbol("my_function", SymbolKind::Function, "src/lib.rs", 10);
        graph.add_definition(symbol);

        let defs = graph.find_definitions("my_function");
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].symbol.name, "my_function");
    }

    #[test]
    fn test_add_and_find_references() {
        let mut graph = SemanticGraph::new();

        let reference = SymbolReference {
            name: "my_function".to_string(),
            location: CodeLocation::new(PathBuf::from("src/main.rs"), 5, 0, 0, 10),
            context: ReferenceContext::Call,
        };
        graph.add_reference(reference);

        let refs = graph.find_references("my_function");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].name, "my_function");
    }

    #[test]
    fn test_find_all_usages() {
        let mut graph = SemanticGraph::new();

        // Add definition
        let symbol = create_test_symbol("process", SymbolKind::Function, "src/lib.rs", 10);
        graph.add_definition(symbol);

        // Add references
        for line in [20, 30, 40] {
            graph.add_reference(SymbolReference {
                name: "process".to_string(),
                location: CodeLocation::new(PathBuf::from("src/main.rs"), line, 0, 0, 7),
                context: ReferenceContext::Call,
            });
        }

        let usages = graph.find_all_usages("process");
        assert_eq!(usages.definitions.len(), 1);
        assert_eq!(usages.references.len(), 3);
        assert_eq!(usages.total(), 4);
    }

    #[test]
    fn test_resolve_reference() {
        let mut graph = SemanticGraph::new();

        // Add definition
        let symbol = create_test_symbol("MyStruct", SymbolKind::Struct, "src/types.rs", 5);
        graph.add_definition(symbol);

        // Create reference
        let reference = SymbolReference {
            name: "MyStruct".to_string(),
            location: CodeLocation::new(PathBuf::from("src/main.rs"), 10, 0, 0, 8),
            context: ReferenceContext::Type,
        };

        let result = graph.resolve(&reference);
        assert_eq!(result.definitions.len(), 1);
        assert_eq!(result.confidence, 1.0);
    }

    #[test]
    fn test_resolve_unresolved_reference() {
        let graph = SemanticGraph::new();

        // Reference to non-existent symbol
        let reference = SymbolReference {
            name: "NonExistent".to_string(),
            location: CodeLocation::new(PathBuf::from("src/main.rs"), 10, 0, 0, 11),
            context: ReferenceContext::Type,
        };

        let result = graph.resolve(&reference);
        assert_eq!(result.definitions.len(), 0);
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn test_resolve_ambiguous_reference() {
        let mut graph = SemanticGraph::new();

        // Add multiple definitions with same name (e.g., "new" in different structs)
        let symbol1 = create_test_symbol("new", SymbolKind::Method, "src/foo.rs", 10);
        let symbol2 = create_test_symbol("new", SymbolKind::Method, "src/bar.rs", 20);
        let symbol3 = create_test_symbol("new", SymbolKind::Method, "src/baz.rs", 30);
        graph.add_definition(symbol1);
        graph.add_definition(symbol2);
        graph.add_definition(symbol3);

        let reference = SymbolReference {
            name: "new".to_string(),
            location: CodeLocation::new(PathBuf::from("src/main.rs"), 5, 0, 0, 3),
            context: ReferenceContext::Call,
        };

        let result = graph.resolve(&reference);
        assert_eq!(result.definitions.len(), 3);
        // Confidence should be lower for ambiguous references
        assert!(result.confidence < 1.0);
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_find_by_kind() {
        let mut graph = SemanticGraph::new();

        // Add various symbol types
        graph.add_definition(create_test_symbol("MyStruct", SymbolKind::Struct, "src/lib.rs", 1));
        graph.add_definition(create_test_symbol("MyEnum", SymbolKind::Enum, "src/lib.rs", 10));
        graph.add_definition(create_test_symbol("my_func", SymbolKind::Function, "src/lib.rs", 20));
        graph.add_definition(create_test_symbol("AnotherStruct", SymbolKind::Struct, "src/lib.rs", 30));
        graph.add_definition(create_test_symbol("MyTrait", SymbolKind::Trait, "src/lib.rs", 40));

        let structs = graph.find_by_kind(SymbolKind::Struct);
        assert_eq!(structs.len(), 2);

        let enums = graph.find_by_kind(SymbolKind::Enum);
        assert_eq!(enums.len(), 1);

        let traits = graph.find_by_kind(SymbolKind::Trait);
        assert_eq!(traits.len(), 1);

        let functions = graph.find_by_kind(SymbolKind::Function);
        assert_eq!(functions.len(), 1);
    }

    #[test]
    fn test_definitions_in_file() {
        let mut graph = SemanticGraph::new();

        // Add symbols in different files
        graph.add_definition(create_test_symbol("foo", SymbolKind::Function, "src/a.rs", 1));
        graph.add_definition(create_test_symbol("bar", SymbolKind::Function, "src/a.rs", 10));
        graph.add_definition(create_test_symbol("baz", SymbolKind::Function, "src/b.rs", 1));

        let defs_a = graph.definitions_in_file(Path::new("src/a.rs"));
        assert_eq!(defs_a.len(), 2);

        let defs_b = graph.definitions_in_file(Path::new("src/b.rs"));
        assert_eq!(defs_b.len(), 1);

        let defs_c = graph.definitions_in_file(Path::new("src/c.rs"));
        assert_eq!(defs_c.len(), 0);
    }

    #[test]
    fn test_references_in_file() {
        let mut graph = SemanticGraph::new();

        // Add references in different files
        graph.add_reference(SymbolReference {
            name: "foo".to_string(),
            location: CodeLocation::new(PathBuf::from("src/main.rs"), 5, 0, 0, 3),
            context: ReferenceContext::Call,
        });
        graph.add_reference(SymbolReference {
            name: "bar".to_string(),
            location: CodeLocation::new(PathBuf::from("src/main.rs"), 10, 0, 0, 3),
            context: ReferenceContext::Call,
        });
        graph.add_reference(SymbolReference {
            name: "baz".to_string(),
            location: CodeLocation::new(PathBuf::from("src/other.rs"), 1, 0, 0, 3),
            context: ReferenceContext::Call,
        });

        let refs_main = graph.references_in_file(Path::new("src/main.rs"));
        assert_eq!(refs_main.len(), 2);

        let refs_other = graph.references_in_file(Path::new("src/other.rs"));
        assert_eq!(refs_other.len(), 1);
    }

    #[test]
    fn test_definition_at_line() {
        let mut graph = SemanticGraph::new();

        graph.add_definition(create_test_symbol("foo", SymbolKind::Function, "src/lib.rs", 10));
        graph.add_definition(create_test_symbol("bar", SymbolKind::Function, "src/lib.rs", 20));

        let def = graph.definition_at(Path::new("src/lib.rs"), 10);
        assert!(def.is_some());
        assert_eq!(def.unwrap().symbol.name, "foo");

        let def = graph.definition_at(Path::new("src/lib.rs"), 20);
        assert!(def.is_some());
        assert_eq!(def.unwrap().symbol.name, "bar");

        // No definition at line 15
        let def = graph.definition_at(Path::new("src/lib.rs"), 15);
        assert!(def.is_none());
    }

    #[test]
    fn test_stats() {
        let mut graph = SemanticGraph::new();

        // Add definitions
        graph.add_definition(create_test_symbol("foo", SymbolKind::Function, "src/a.rs", 1));
        graph.add_definition(create_test_symbol("bar", SymbolKind::Function, "src/b.rs", 1));
        graph.add_definition(create_test_symbol("foo", SymbolKind::Method, "src/c.rs", 1)); // Same name, different file

        // Add references
        graph.add_reference(SymbolReference {
            name: "foo".to_string(),
            location: CodeLocation::new(PathBuf::from("src/main.rs"), 5, 0, 0, 3),
            context: ReferenceContext::Call,
        });

        let stats = graph.stats();
        assert_eq!(stats.total_definitions, 3);
        assert_eq!(stats.unique_symbols, 2); // "foo" and "bar"
        assert_eq!(stats.files_analyzed, 3); // a.rs, b.rs, c.rs
        assert_eq!(stats.total_references, 1);
    }

    #[test]
    fn test_all_symbol_names() {
        let mut graph = SemanticGraph::new();

        graph.add_definition(create_test_symbol("alpha", SymbolKind::Function, "src/lib.rs", 1));
        graph.add_definition(create_test_symbol("beta", SymbolKind::Struct, "src/lib.rs", 10));
        graph.add_definition(create_test_symbol("gamma", SymbolKind::Enum, "src/lib.rs", 20));

        let names = graph.all_symbol_names();
        assert_eq!(names.len(), 3);
        assert!(names.iter().any(|n| *n == "alpha"));
        assert!(names.iter().any(|n| *n == "beta"));
        assert!(names.iter().any(|n| *n == "gamma"));
    }

    #[test]
    fn test_add_symbols_batch() {
        let mut graph = SemanticGraph::new();

        let symbols = vec![
            create_test_symbol("func1", SymbolKind::Function, "src/lib.rs", 1),
            create_test_symbol("func2", SymbolKind::Function, "src/lib.rs", 10),
            create_test_symbol("Struct1", SymbolKind::Struct, "src/lib.rs", 20),
        ];

        graph.add_symbols(&symbols);

        assert_eq!(graph.find_definitions("func1").len(), 1);
        assert_eq!(graph.find_definitions("func2").len(), 1);
        assert_eq!(graph.find_definitions("Struct1").len(), 1);
        assert_eq!(graph.stats().total_definitions, 3);
    }

    #[test]
    fn test_fqn_computation() {
        let mut graph = SemanticGraph::new();

        // Symbol without parent
        let symbol1 = create_test_symbol("standalone", SymbolKind::Function, "src/lib.rs", 1);
        graph.add_definition(symbol1);

        // Symbol with parent
        let mut symbol2 = create_test_symbol("method", SymbolKind::Method, "src/lib.rs", 10);
        symbol2.parent = Some("MyStruct".to_string());
        graph.add_definition(symbol2);

        let defs = graph.find_definitions("standalone");
        assert_eq!(defs[0].fqn, "standalone");

        let defs = graph.find_definitions("method");
        assert_eq!(defs[0].fqn, "MyStruct::method");
    }

    #[test]
    fn test_reference_contexts() {
        let mut graph = SemanticGraph::new();

        // Add references with different contexts
        let contexts = [
            ReferenceContext::Call,
            ReferenceContext::Type,
            ReferenceContext::FieldAccess,
            ReferenceContext::Import,
            ReferenceContext::Identifier,
            ReferenceContext::Unknown,
        ];

        for (i, ctx) in contexts.iter().enumerate() {
            graph.add_reference(SymbolReference {
                name: format!("symbol_{}", i),
                location: CodeLocation::new(PathBuf::from("src/main.rs"), i + 1, 0, 0, 5),
                context: *ctx,
            });
        }

        assert_eq!(graph.stats().total_references, 6);

        // Verify each reference has correct context
        let refs = graph.find_references("symbol_0");
        assert_eq!(refs[0].context, ReferenceContext::Call);

        let refs = graph.find_references("symbol_1");
        assert_eq!(refs[0].context, ReferenceContext::Type);
    }

    #[test]
    fn test_default_trait() {
        let graph = SemanticGraph::default();
        assert_eq!(graph.stats().total_definitions, 0);
        assert_eq!(graph.stats().total_references, 0);
    }

    #[test]
    fn test_all_usages_total() {
        let usages = AllUsages {
            name: "test".to_string(),
            definitions: vec![
                CodeLocation::new(PathBuf::from("a.rs"), 1, 0, 0, 4),
                CodeLocation::new(PathBuf::from("b.rs"), 1, 0, 0, 4),
            ],
            references: vec![
                CodeLocation::new(PathBuf::from("c.rs"), 1, 0, 0, 4),
                CodeLocation::new(PathBuf::from("d.rs"), 1, 0, 0, 4),
                CodeLocation::new(PathBuf::from("e.rs"), 1, 0, 0, 4),
            ],
        };

        assert_eq!(usages.total(), 5);
    }

	#[test]
	fn test_is_type_usage_returns_true_for_type_related_variants() {
		// all type-related variants should return true
		assert!(ReferenceContext::Type.is_type_usage());
		assert!(ReferenceContext::FieldType.is_type_usage());
		assert!(ReferenceContext::ReturnType.is_type_usage());
		assert!(ReferenceContext::ParameterType.is_type_usage());
		assert!(ReferenceContext::GenericArg.is_type_usage());
		assert!(ReferenceContext::TraitBound.is_type_usage());
		assert!(ReferenceContext::ImplTarget.is_type_usage());
	}

	#[test]
	fn test_is_type_usage_returns_false_for_non_type_variants() {
		// non-type-related variants should return false
		assert!(!ReferenceContext::Call.is_type_usage());
		assert!(!ReferenceContext::FieldAccess.is_type_usage());
		assert!(!ReferenceContext::Import.is_type_usage());
		assert!(!ReferenceContext::Identifier.is_type_usage());
		assert!(!ReferenceContext::Unknown.is_type_usage());
	}

	#[test]
	fn test_reference_context_serialization() {
		// test serialization of new variants
		let contexts = [
			ReferenceContext::FieldType,
			ReferenceContext::ReturnType,
			ReferenceContext::ParameterType,
			ReferenceContext::GenericArg,
			ReferenceContext::TraitBound,
			ReferenceContext::ImplTarget,
		];

		for ctx in contexts {
			// serialize to JSON
			let serialized = serde_json::to_string(&ctx).expect("serialization failed");
			// deserialize back
			let deserialized: ReferenceContext =
				serde_json::from_str(&serialized).expect("deserialization failed");
			// verify round-trip
			assert_eq!(ctx, deserialized);
		}
	}

	#[test]
	fn test_reference_context_deserialization_from_string() {
		// test deserialization from known string values
		let test_cases = [
			("\"FieldType\"", ReferenceContext::FieldType),
			("\"ReturnType\"", ReferenceContext::ReturnType),
			("\"ParameterType\"", ReferenceContext::ParameterType),
			("\"GenericArg\"", ReferenceContext::GenericArg),
			("\"TraitBound\"", ReferenceContext::TraitBound),
			("\"ImplTarget\"", ReferenceContext::ImplTarget),
		];

		for (json_str, expected) in test_cases {
			let deserialized: ReferenceContext =
				serde_json::from_str(json_str).expect("deserialization failed");
			assert_eq!(deserialized, expected);
		}
	}

	#[test]
	fn test_find_type_usages() {
		// test that find_type_usages returns only type-related references
		let mut graph = SemanticGraph::new();

		// add type-related references for "MyType"
		graph.add_reference(SymbolReference {
			name: "MyType".to_string(),
			location: CodeLocation::new(PathBuf::from("src/lib.rs"), 10, 0, 0, 6),
			context: ReferenceContext::Type,
		});
		graph.add_reference(SymbolReference {
			name: "MyType".to_string(),
			location: CodeLocation::new(PathBuf::from("src/lib.rs"), 20, 0, 0, 6),
			context: ReferenceContext::FieldType,
		});
		graph.add_reference(SymbolReference {
			name: "MyType".to_string(),
			location: CodeLocation::new(PathBuf::from("src/lib.rs"), 30, 0, 0, 6),
			context: ReferenceContext::ReturnType,
		});
		graph.add_reference(SymbolReference {
			name: "MyType".to_string(),
			location: CodeLocation::new(PathBuf::from("src/lib.rs"), 40, 0, 0, 6),
			context: ReferenceContext::ParameterType,
		});

		// add non-type references for "MyType"
		graph.add_reference(SymbolReference {
			name: "MyType".to_string(),
			location: CodeLocation::new(PathBuf::from("src/lib.rs"), 50, 0, 0, 6),
			context: ReferenceContext::Call,
		});
		graph.add_reference(SymbolReference {
			name: "MyType".to_string(),
			location: CodeLocation::new(PathBuf::from("src/lib.rs"), 60, 0, 0, 6),
			context: ReferenceContext::Identifier,
		});

		// find_type_usages should return only the 4 type-related references
		let type_usages = graph.find_type_usages("MyType");
		assert_eq!(type_usages.len(), 4);

		// verify all returned references have type-related contexts
		for usage in &type_usages {
			assert!(usage.context.is_type_usage());
		}

		// non-existent type should return empty vec
		let empty = graph.find_type_usages("NonExistent");
		assert!(empty.is_empty());
	}

	#[test]
	fn test_find_trait_implementations() {
		// test that find_trait_implementations finds impl blocks for a trait
		let mut graph = SemanticGraph::new();

		// create impl block with signature containing trait name
		let mut impl_symbol1 = create_test_symbol("", SymbolKind::Impl, "src/lib.rs", 10);
		impl_symbol1.signature = Some("impl MyTrait for Foo".to_string());
		graph.add_definition(impl_symbol1);

		// another impl for the same trait
		let mut impl_symbol2 = create_test_symbol("", SymbolKind::Impl, "src/lib.rs", 50);
		impl_symbol2.signature = Some("impl MyTrait for Bar".to_string());
		graph.add_definition(impl_symbol2);

		// impl for a different trait
		let mut impl_symbol3 = create_test_symbol("", SymbolKind::Impl, "src/lib.rs", 100);
		impl_symbol3.signature = Some("impl OtherTrait for Baz".to_string());
		graph.add_definition(impl_symbol3);

		// inherent impl (no trait)
		let mut impl_symbol4 = create_test_symbol("", SymbolKind::Impl, "src/lib.rs", 150);
		impl_symbol4.signature = Some("impl Qux".to_string());
		graph.add_definition(impl_symbol4);

		// impl without signature
		let impl_symbol5 = create_test_symbol("", SymbolKind::Impl, "src/lib.rs", 200);
		graph.add_definition(impl_symbol5);

		// find implementations of MyTrait
		let impls = graph.find_trait_implementations("MyTrait");
		assert_eq!(impls.len(), 2);

		// verify signatures contain MyTrait
		for imp in &impls {
			assert!(imp.symbol.signature.as_ref().unwrap().contains("MyTrait"));
		}

		// find implementations of OtherTrait
		let other_impls = graph.find_trait_implementations("OtherTrait");
		assert_eq!(other_impls.len(), 1);

		// non-existent trait should return empty vec
		let empty = graph.find_trait_implementations("NonExistentTrait");
		assert!(empty.is_empty());
	}

	#[test]
	fn test_find_usages_by_context() {
		// test that find_usages_by_context groups references correctly
		let mut graph = SemanticGraph::new();

		// add references with various contexts for "foo"
		graph.add_reference(SymbolReference {
			name: "foo".to_string(),
			location: CodeLocation::new(PathBuf::from("src/a.rs"), 1, 0, 0, 3),
			context: ReferenceContext::Call,
		});
		graph.add_reference(SymbolReference {
			name: "foo".to_string(),
			location: CodeLocation::new(PathBuf::from("src/a.rs"), 10, 0, 0, 3),
			context: ReferenceContext::Call,
		});
		graph.add_reference(SymbolReference {
			name: "foo".to_string(),
			location: CodeLocation::new(PathBuf::from("src/b.rs"), 5, 0, 0, 3),
			context: ReferenceContext::Type,
		});
		graph.add_reference(SymbolReference {
			name: "foo".to_string(),
			location: CodeLocation::new(PathBuf::from("src/c.rs"), 20, 0, 0, 3),
			context: ReferenceContext::Import,
		});
		graph.add_reference(SymbolReference {
			name: "foo".to_string(),
			location: CodeLocation::new(PathBuf::from("src/c.rs"), 25, 0, 0, 3),
			context: ReferenceContext::Import,
		});
		graph.add_reference(SymbolReference {
			name: "foo".to_string(),
			location: CodeLocation::new(PathBuf::from("src/c.rs"), 30, 0, 0, 3),
			context: ReferenceContext::Import,
		});

		// group by context
		let grouped = graph.find_usages_by_context("foo");

		// verify grouping
		assert_eq!(grouped.len(), 3); // Call, Type, Import
		assert_eq!(grouped.get(&ReferenceContext::Call).unwrap().len(), 2);
		assert_eq!(grouped.get(&ReferenceContext::Type).unwrap().len(), 1);
		assert_eq!(grouped.get(&ReferenceContext::Import).unwrap().len(), 3);

		// non-existent symbol should return empty map
		let empty = graph.find_usages_by_context("NonExistent");
		assert!(empty.is_empty());
	}
}

