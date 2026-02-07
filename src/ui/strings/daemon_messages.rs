//! Daemon status and lifecycle message constants.

// -- Lifecycle --
pub const STARTING: &str =
	"Starting model daemon...";
pub const STARTED: &str = "Daemon started";
pub const STOPPING: &str = "Stopping daemon";
pub const STOPPED: &str = "Daemon stopped";
pub const NOT_RUNNING: &str =
	"Daemon is not running";
pub const ALREADY_RUNNING: &str =
	"Daemon is already running";
pub const MAY_NOT_STARTED: &str =
	"Warning: Daemon may not have started \
	correctly";
pub const MODELS_LOADING: &str =
	"Models are loading in the background...";

// -- Status display --
pub const STATUS_RUNNING: &str =
	"Daemon Status: Running";
pub const STATUS_NOT_RUNNING: &str =
	"Daemon Status: Not running";
pub const STATUS_UNREACHABLE: &str =
	"Daemon Status: Running (but unreachable)";
pub const LOADED_MODELS: &str = "Loaded Models:";
pub const NO_MODELS: &str = "  (none)";

// -- Internal daemon logs --
pub const LOG_PREFIX: &str = "[daemon]";
pub const LOG_LISTENING: &str = "Listening on";
pub const LOG_LOADING_MODELS: &str =
	"Loading models (socket ready for \
	status/ping)...";
pub const LOG_ALL_MODELS_LOADED: &str =
	"All models loaded, ready for requests";
pub const LOG_SHUTTING_DOWN: &str =
	"Shutting down...";
pub const LOG_DEGRADED_NO_EMBED: &str =
	"Running in degraded mode (no embeddings)";
pub const LOG_DEGRADED_NO_RERANK: &str =
	"Running in degraded mode (no reranking)";
pub const LOG_DEGRADED_NO_EXPAND: &str =
	"Running in degraded mode \
	(no query expansion)";
