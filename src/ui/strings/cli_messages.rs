//! CLI command output message constants.
//!
//! All user-facing text from CLI commands.
//! Format strings use named placeholders for clarity.

// -- Search --
pub const NO_RESULTS: &str = "No results found";
pub const SEARCH_RESULTS_HEADER: &str =
	"Search results";
pub const NO_CALLERS: &str = "No callers found";
pub const NO_CALLEES: &str =
	"No function calls found";
pub const NO_REFS: &str = "No references found";
pub const CALLER_TIP: &str =
	"Tip: Make sure the symbol name is exact \
	(case-sensitive)";

// -- Index --
pub const INDEX_STATS_HEADER: &str =
	"Index Statistics:";
pub const SEMANTIC_HEADER: &str =
	"Semantic Analysis:";
pub const SEARCH_INDEX_HEADER: &str =
	"Building search index...";
pub const SEMANTIC_ENABLED: &str =
	"   Semantic analysis: enabled";

// -- Symbols --
pub const NO_SYMBOLS: &str = "No symbols found";

// -- Info --
pub const SIGNATURE_HEADER: &str = "Signature:";
pub const DOC_HEADER: &str = "Documentation:";
pub const SOURCE_HEADER: &str = "Source Code:";
pub const CALLERS_HEADER: &str = "Callers";
pub const CALLEES_HEADER: &str = "Callees";
pub const REFS_HEADER: &str = "References";
pub const DEF_NOT_FOUND: &str =
	"(Definition not found)";

// -- Docs --
pub const DOC_STATUS_HEADER: &str =
	"Documentation Status:";
pub const DOC_READY: &str =
	"Documentation is ready!";
pub const DOC_RUNNING: &str =
	"Generation running in background...";
pub const DOC_NOT_STARTED: &str =
	"Generation not started. \
	Run 'ch-cli docs generate'.";
pub const DOC_GENERATE_STARTED: &str =
	"Documentation generation started \
	in background.";
pub const DOC_CHECK_PROGRESS: &str =
	"Use 'ch-cli docs status' to check progress.";
pub const DOC_NO_DOCS: &str =
	"No documentation found.";
pub const DOC_RUN_GENERATE: &str =
	"Run 'ch-cli docs generate' to start.";

// -- Embed --
pub const CONNECTING_DAEMON: &str =
	"Connecting to daemon...";
pub const DAEMON_READY: &str = "Daemon is ready";
pub const DAEMON_STARTED_LOADING: &str =
	"Daemon started, waiting for models to load...";
pub const RUN_DAEMON_FIRST: &str =
	"Run 'ch-cli daemon start' first and wait \
	for models to load";
pub const EMBEDDING_SYMBOLS: &str =
	"Generating embeddings...";

// -- Retrieve --
pub const INIT_PIPELINE: &str =
	"Initializing retrieval pipeline...";
pub const RESULTS_HEADER: &str =
	"RETRIEVAL RESULTS";
pub const CONTEXT_HEADER: &str = "CONTEXT OUTPUT";
pub const MORE_RESULTS: &str =
	"(More results available - increase \
	--limit to see more)";

// -- Warnings --
pub const WARN_DAEMON_NOT_READY: &str =
	"Warning: Daemon not ready";
pub const WARN_CONTEXT_NEEDS_SEMANTIC: &str =
	"Warning: Context expansion requires \
	semantic analysis.";
pub const MAKE_SURE_DAEMON: &str =
	"Make sure the daemon is running: \
	ch-cli daemon start";
