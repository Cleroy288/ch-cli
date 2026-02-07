//! Progress bar and spinner text constants.

// -- Spinner frames --
pub const SPINNER_FRAMES: &[&str] = &[
	"\u{280B}", "\u{2819}", "\u{2839}", "\u{2838}",
	"\u{283C}", "\u{2834}", "\u{2826}", "\u{2827}",
	"\u{2807}", "\u{280F}",
];

// -- Indexing progress --
pub const INDEXING_CODEBASE: &str =
	"  Indexing codebase...\n\n";
pub const FILES_LABEL: &str = "  Files:";
pub const CURRENT_LABEL: &str = "  Current:";
pub const ELAPSED_LABEL: &str = "  Elapsed:";

// -- Doc generation progress --
pub const STEP_DAEMON: &str =
	"  Step 2/3: Starting daemon + \
	loading models...\n";
pub const STEP_DOCGEN: &str =
	"  Step 3/3: Generating documentation...\n";
pub const DOCS_LABEL: &str = "  Docs:";
pub const ELAPSED_SKIP: &str =
	"  Elapsed: {}s  (Ctrl+C to skip)";

// -- Incremental progress --
pub const FILES_TO_REINDEX: &str =
	"file(s) to re-index";

// -- Watcher --
pub const WATCHER_PREFIX: &str = "[watcher]";
pub const WATCHER_WATCHING: &str =
	"Watching for changes...";
pub const WATCHER_STOPPED: &str = "Stopped";
pub const WATCHER_REINDEXING: &str =
	"file(s) changed, reindexing...";
