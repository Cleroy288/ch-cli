//! Fast Path Pattern Constants
//!
//! Defines stop words, conceptual patterns, and source code
//! patterns used by the FastPathParser.

/// Stop words to filter out from symbol extraction
pub const STOP_WORDS: &[&str] = &[
	"how", "where", "what", "when", "why", "which", "who",
	"the", "a", "an", "is", "are", "was", "were", "be",
	"been", "does", "do", "did", "has", "have", "had",
	"can", "could", "will", "would", "should", "shall",
	"may", "might", "must", "find", "show", "get",
	"search", "look", "locate", "defined", "used",
	"called", "work", "works", "working", "implement",
];

/// Patterns indicating conceptual/explanatory queries
pub const CONCEPTUAL_PATTERNS: &[&str] = &[
	"how does",
	"how do",
	"how is",
	"how are",
	"what is",
	"what are",
	"what does",
	"what happens",
	"what is the purpose",
	"explain",
	"describe",
	"understand",
	"why does",
	"why is",
	"when to use",
	"difference between",
	"compare",
	"architecture of",
	"design of",
	"flow of",
	"implementation of",
	"logic of",
];

/// Patterns indicating explicit source code requests
pub const SOURCE_CODE_PATTERNS: &[&str] = &[
	"show me the code",
	"show the code",
	"show code for",
	"source code",
	"implementation of",
	"code for",
];

/// Type of pattern that matched a symbol candidate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
	/// CamelCase (e.g., AuthService, BgeEmbedder)
	CamelCase,
	/// snake_case (e.g., parse_config, get_user)
	SnakeCase,
	/// SCREAMING_CASE (e.g., MAX_TOKENS)
	ScreamingCase,
	/// Generic identifier (alphanumeric)
	Identifier,
}

/// A candidate symbol extracted from the query
#[derive(Debug, Clone)]
pub struct SymbolCandidate {
	/// the symbol name
	pub name: String,
	/// the pattern type that matched
	pub pattern_type: PatternType,
	/// confidence that this is a real symbol (0.0 - 1.0)
	pub confidence: f32,
}

/// Query intent classification for fast-path decision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastPathIntent {
	/// Explicit symbol lookup (high confidence fast-path)
	Explicit,
	/// Conceptual/explanatory query (needs LLM)
	Conceptual,
	/// Mixed: has symbols but also conceptual elements
	Mixed,
}

/// Result of fast-path parsing
#[derive(Debug, Clone)]
pub struct FastPathResult {
	/// extracted symbol candidates
	pub symbols: Vec<SymbolCandidate>,
	/// overall confidence score (0.0 - 1.0)
	pub confidence: f32,
	/// detected query intent
	pub intent: FastPathIntent,
	/// whether to use fast-path (skip LLM)
	pub use_fast_path: bool,
}

