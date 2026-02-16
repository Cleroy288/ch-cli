//! Model Daemon Server
//!
//! The daemon server loads ML models and handles
//! requests from clients over a Unix socket.
//!
//! Module structure:
//! - `types`: CachedProject struct and constants
//! - `lifecycle`: Daemon creation and startup
//! - `model_loader`: Model loading during startup
//! - `client_handler`: Request dispatch
//! - `project_handlers`: Project indexing and search
//! - `doc_handlers`: Documentation request handlers
//! - `doc_generation`: Background doc generation
//! - `doc_gen_batch`: Batch extraction and LLM inference
//! - `utils`: Cache management utilities

mod client_handler;
mod constructors;
mod doc_gen_batch;
mod doc_gen_helpers;
mod doc_gen_template;
mod doc_generation;
#[doc(hidden)]
pub mod doc_handlers;
mod doc_handlers_helpers;
mod doc_lookup;
mod doc_search;
mod doc_status;
#[doc(hidden)]
pub mod lifecycle;
mod lifecycle_utils;
mod model_loader;
mod project_handlers;
mod request_handlers;
#[doc(hidden)]
pub mod types;
#[doc(hidden)]
pub mod utils;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use types::{
	CachedProject, SharedDocGenerator, SharedDocStores,
};

use crate::retrieval::hybrid::embedder_trait::Embedder;
use crate::retrieval::models::{
	get_device_info, DeviceInfo,
};
use crate::retrieval::query::QueryInterpreter;
use crate::retrieval::rerank::BgeReranker;
use crate::retrieval::{RetrievalConfig, RetrievalResult};

/// Progress tracking for background doc generation
#[derive(Debug, Clone, Default)]
pub struct DocGenProgress {
	/// total entries to process
	pub total: usize,
	/// completed entries
	pub completed: usize,
	/// failed entries
	pub failed: usize,
	/// whether generation is currently running
	pub is_running: bool,
}

/// The model daemon that handles ML requests
pub struct ModelDaemon {
	/// path to the Unix socket
	#[doc(hidden)]
	pub socket_path: PathBuf,
	/// configuration
	pub(crate) config: RetrievalConfig,
	/// start time for uptime calculation
	pub(crate) start_time: Instant,
	/// flag to signal shutdown
	pub(crate) shutdown: Arc<AtomicBool>,
	/// embedding model (loaded on startup)
	#[doc(hidden)]
	pub embedder: Option<Box<dyn Embedder>>,
	/// reranker model (loaded on startup)
	#[doc(hidden)]
	pub reranker: Option<BgeReranker>,
	/// query interpreter with LLM
	pub(crate) interpreter: QueryInterpreter,
	/// detected compute device information
	pub(crate) device_info: DeviceInfo,
	/// cached project indices
	#[doc(hidden)]
	pub project_cache:
		HashMap<PathBuf, CachedProject>,
	/// doc stores per project (shared w/ bg thread)
	pub(crate) doc_stores: SharedDocStores,
	/// doc generator (shared w/ bg thread)
	pub(crate) doc_generator: SharedDocGenerator,
	/// bg doc gen progress (shared w/ bg thread)
	pub(crate) doc_gen_progress:
		Arc<Mutex<DocGenProgress>>,
	/// handle to bg doc gen thread
	pub(crate) doc_gen_thread:
		Option<std::thread::JoinHandle<()>>,
	/// flag to cancel bg doc generation
	pub(crate) doc_gen_cancel: Arc<AtomicBool>,
}

impl ModelDaemon {
	/// Create a new daemon with default config
	pub fn new() -> Self {
		constructors::new_daemon()
	}

	/// Create with custom socket path
	pub fn with_socket_path(
		socket_path: impl AsRef<Path>,
	) -> Self {
		constructors::with_socket_path(socket_path)
	}

	/// Internal constructor with explicit config
	pub(crate) fn new_with_config(
		socket_path: PathBuf,
		config: RetrievalConfig,
	) -> Self {
		Self {
			socket_path,
			config,
			start_time: Instant::now(),
			shutdown: Arc::new(AtomicBool::new(false)),
			embedder: None,
			reranker: None,
			interpreter: QueryInterpreter::new(),
			device_info: get_device_info(),
			project_cache: HashMap::new(),
			doc_stores: Arc::new(Mutex::new(
				HashMap::new(),
			)),
			doc_generator: Arc::new(Mutex::new(None)),
			doc_gen_progress: Arc::new(Mutex::new(
				DocGenProgress::default(),
			)),
			doc_gen_thread: None,
			doc_gen_cancel: Arc::new(AtomicBool::new(
				false,
			)),
		}
	}

	/// Run the daemon (blocking)
	pub fn run(&mut self) -> RetrievalResult<()> {
		lifecycle::run(self)
	}
}

impl Default for ModelDaemon {
	fn default() -> Self {
		Self::new()
	}
}
