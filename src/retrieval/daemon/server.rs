//! Model Daemon Server
//!
//! The daemon server loads ML models and handles requests from clients.
//! It listens on a Unix socket and processes embedding/reranking requests.

use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Instant, SystemTime};

use super::protocol::{
	deserialize_request, serialize_response, CachedSearchResult, DaemonRequest, DaemonResponse,
	DaemonStatus, DeviceStatus, DocEntryResponse,
};
use crate::indexer::{IndexManager, SemanticGraph, Symbol};
use crate::retrieval::docgen::{DocGenerator, DocLinker, DocStore};
use crate::retrieval::hybrid::{BgeEmbedder, HybridSearch};
use crate::retrieval::models::{get_device_info, DeviceInfo};
use crate::retrieval::query::QueryInterpreter;
use crate::retrieval::rerank::BgeReranker;
use crate::retrieval::{RetrievalConfig, RetrievalError, RetrievalResult};

/// Maximum number of projects to cache in daemon memory
const MAX_CACHED_PROJECTS: usize = 5;

/// A cached project with index data for fast retrieval
struct CachedProject {
	/// all indexed symbols
	symbols: Vec<Symbol>,
	/// semantic graph for context expansion
	graph: Option<SemanticGraph>,
	/// hybrid search instance
	hybrid: HybridSearch,
	/// when the project was indexed (unix timestamp)
	last_indexed: u64,
}

/// The model daemon that handles ML requests
pub struct ModelDaemon {
	/// path to the Unix socket
	socket_path: std::path::PathBuf,
	/// configuration
	config: RetrievalConfig,
	/// start time for uptime calculation
	start_time: Instant,
	/// flag to signal shutdown
	shutdown: Arc<AtomicBool>,
	/// embedding model (loaded on startup)
	embedder: Option<BgeEmbedder>,
	/// reranker model (loaded on startup)
	reranker: Option<BgeReranker>,
	/// query interpreter with LLM (loaded on startup)
	interpreter: QueryInterpreter,
	/// detected compute device information
	device_info: DeviceInfo,
	/// cached project indices (project_path -> CachedProject)
	project_cache: HashMap<PathBuf, CachedProject>,
	/// documentation stores per project (project_path -> DocStore)
	doc_stores: HashMap<PathBuf, DocStore>,
	/// documentation generator (uses LLM)
	doc_generator: Option<DocGenerator>,
}

impl ModelDaemon {
	/// Create a new daemon with default config
	pub fn new() -> Self {
		let config = RetrievalConfig::default();
		Self {
			socket_path: config.socket_path.clone(),
			config,
			start_time: Instant::now(),
			shutdown: Arc::new(AtomicBool::new(false)),
			embedder: None,
			reranker: None,
			interpreter: QueryInterpreter::new(),
			device_info: get_device_info(),
			project_cache: HashMap::new(),
			doc_stores: HashMap::new(),
			doc_generator: None,
		}
	}

	/// Create a new daemon with custom socket path
	pub fn with_socket_path(socket_path: impl AsRef<Path>) -> Self {
		let config = RetrievalConfig::default();
		Self {
			socket_path: socket_path.as_ref().to_path_buf(),
			config,
			start_time: Instant::now(),
			shutdown: Arc::new(AtomicBool::new(false)),
			embedder: None,
			reranker: None,
			interpreter: QueryInterpreter::new(),
			device_info: get_device_info(),
			project_cache: HashMap::new(),
			doc_stores: HashMap::new(),
			doc_generator: None,
		}
	}

	/// Write PID file
	fn write_pid_file(&self) -> RetrievalResult<()> {
		let pid_file = self.socket_path.with_extension("pid");
		let pid = std::process::id();
		fs::write(&pid_file, pid.to_string())?;
		Ok(())
	}

	/// Get list of loaded models
	fn loaded_models(&self) -> Vec<String> {
		let mut models = Vec::new();
		if self.embedder.is_some() {
			models.push(format!("{} (embeddings)", self.config.embedding_model));
		}
		if self.reranker.is_some() {
			models.push(format!("{} (reranker)", self.config.reranker_model));
		}
		if self.interpreter.has_llm() {
			models.push(format!("{} (query expansion)", self.config.expansion_model));
		}
		models
	}

	/// Handle status request - returns daemon status info
	fn handle_status(&self) -> DaemonResponse {
		DaemonResponse::Status(DaemonStatus {
			running: true,
			pid: Some(std::process::id()),
			loaded_models: self.loaded_models(),
			device: DeviceStatus {
				device_type: self.device_info.device_type.to_string(),
				device_name: self.device_info.device_name.clone(),
				memory_mb: self.device_info.memory_mb,
			},
			uptime_secs: self.start_time.elapsed().as_secs(),
		})
	}

	/// Handle a single client request
	fn handle_request(&mut self, request: DaemonRequest) -> DaemonResponse {
		match request {
			DaemonRequest::Ping => DaemonResponse::Pong,

			DaemonRequest::Status => self.handle_status(),

			DaemonRequest::Shutdown => {
				self.shutdown.store(true, Ordering::SeqCst);
				DaemonResponse::Ok
			}

			DaemonRequest::Embed { texts } => {
				match &self.embedder {
					Some(embedder) => {
						match embedder.embed_batch(&texts) {
							Ok(embeddings) => DaemonResponse::Embeddings(embeddings),
							Err(e) => DaemonResponse::Error(format!("embedding error: {}", e)),
						}
					}
					None => DaemonResponse::Error("embedding model not loaded".to_string()),
				}
			}

			DaemonRequest::Rerank { query, documents } => {
				match &self.reranker {
					Some(reranker) => {
						match reranker.score_batch(&query, &documents) {
							Ok(scores) => DaemonResponse::Scores(scores),
							Err(e) => DaemonResponse::Error(format!("reranking error: {}", e)),
						}
					}
					None => {
						// fallback: return dummy scores based on position
						let scores: Vec<f32> = documents
							.iter()
							.enumerate()
							.map(|(i, _)| 1.0 / (i as f32 + 1.0))
							.collect();
						DaemonResponse::Scores(scores)
					}
				}
			}

			DaemonRequest::Expand { query } => {
				// use interpreter to expand query (LLM or fallback)
				let spec = self.interpreter.interpret(&query);
				DaemonResponse::SearchSpec(spec)
			}

			DaemonRequest::IndexProject { project_path, force } => {
				self.handle_index_project(&project_path, force)
			}

			DaemonRequest::SearchProject { project_path, query, limit } => {
				self.handle_search_project(&project_path, &query, limit)
			}

			DaemonRequest::ProjectStatus { project_path } => {
				self.handle_project_status(&project_path)
			}

			DaemonRequest::EvictProject { project_path } => {
				self.handle_evict_project(&project_path)
			}

			DaemonRequest::StartDocGen { project_path, force } => {
				self.handle_start_doc_gen(&project_path, force)
			}

			DaemonRequest::DocGenStatus { project_path } => {
				self.handle_doc_gen_status(&project_path)
			}

			DaemonRequest::GetDoc { project_path, symbol_name } => {
				self.handle_get_doc(&project_path, &symbol_name)
			}

			DaemonRequest::SearchDocs { project_path, query, limit } => {
				self.handle_search_docs(&project_path, &query, limit)
			}
		}
	}

	/// Handle IndexProject request - index a project and cache in memory
	fn handle_index_project(&mut self, project_path: &str, force: bool) -> DaemonResponse {
		let path = PathBuf::from(project_path); // project path
		let canonical = path.canonicalize().unwrap_or(path.clone()); // normalized path

		// Check if already cached and not forcing
		if !force {
			if let Some(cached) = self.project_cache.get(&canonical) {
				return DaemonResponse::ProjectIndexed {
					symbol_count: cached.symbols.len(),
					cached: true,
					index_time_ms: 0,
				};
			}
		}

		// Index the project with persistence
		let start = Instant::now();
		let manager = IndexManager::new()
			.with_persistence()
			.with_semantic_analysis();

		let result = match manager.index_project(&canonical) {
			Ok(r) => r,
			Err(e) => return DaemonResponse::Error(format!("indexing error: {}", e)),
		};

		// Build hybrid search
		let mut hybrid = match HybridSearch::new() {
			Ok(h) => h,
			Err(e) => return DaemonResponse::Error(format!("hybrid search error: {}", e)),
		};

		if let Err(e) = hybrid.index_symbols(&result.symbols) {
			return DaemonResponse::Error(format!("index symbols error: {}", e));
		}

		let symbol_count = result.symbols.len();
		let index_time_ms = start.elapsed().as_millis() as u64;

		// Get current timestamp
		let now = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.map(|d| d.as_secs())
			.unwrap_or(0);

		// Cache in memory
		self.project_cache.insert(
			canonical,
			CachedProject {
				symbols: result.symbols,
				graph: result.semantic_graph,
				hybrid,
				last_indexed: now,
			},
		);

		// LRU eviction if needed
		self.evict_lru_if_needed();

		DaemonResponse::ProjectIndexed {
			symbol_count,
			cached: false,
			index_time_ms,
		}
	}

	/// Handle SearchProject request - search a cached project
	fn handle_search_project(&self, project_path: &str, query: &str, limit: usize) -> DaemonResponse {
		let path = PathBuf::from(project_path); // project path
		let canonical = path.canonicalize().unwrap_or(path); // normalized path

		let cached = match self.project_cache.get(&canonical) {
			Some(c) => c,
			None => return DaemonResponse::Error("Project not cached".to_string()),
		};

		// Perform hybrid search
		let results = match cached.hybrid.search(query, limit) {
			Ok(r) => r,
			Err(e) => return DaemonResponse::Error(format!("search error: {}", e)),
		};

		// Convert to CachedSearchResult
		let cached_results: Vec<CachedSearchResult> = results
			.into_iter()
			.map(|r| CachedSearchResult {
				symbol_name: r.symbol.name,
				symbol_kind: r.symbol.kind.to_string(),
				file_path: r.symbol.location.file.display().to_string(),
				line: r.symbol.location.line,
				score: r.rrf_score,
				rerank_score: r.rerank_score,
			})
			.collect();

		DaemonResponse::SearchResults(cached_results)
	}

	/// Handle ProjectStatus request - check if a project is cached
	fn handle_project_status(&self, project_path: &str) -> DaemonResponse {
		let path = PathBuf::from(project_path); // project path
		let canonical = path.canonicalize().unwrap_or(path); // normalized path

		match self.project_cache.get(&canonical) {
			Some(cached) => DaemonResponse::ProjectCacheStatus {
				cached: true,
				symbol_count: cached.symbols.len(),
				last_indexed: cached.last_indexed,
			},
			None => DaemonResponse::ProjectCacheStatus {
				cached: false,
				symbol_count: 0,
				last_indexed: 0,
			},
		}
	}

	/// Handle EvictProject request - remove a project from cache
	fn handle_evict_project(&mut self, project_path: &str) -> DaemonResponse {
		let path = PathBuf::from(project_path); // project path
		let canonical = path.canonicalize().unwrap_or(path); // normalized path

		if self.project_cache.remove(&canonical).is_some() {
			eprintln!("[daemon] Evicted project from cache: {:?}", canonical);
			DaemonResponse::Ok
		} else {
			DaemonResponse::Error("Project not in cache".to_string())
		}
	}

	/// Handle StartDocGen request - start documentation generation
	fn handle_start_doc_gen(&mut self, project_path: &str, force: bool) -> DaemonResponse {
		let path = PathBuf::from(project_path);
		let canonical = path.canonicalize().unwrap_or(path.clone());

		// load or create doc store
		let store = if let Some(existing) = self.doc_stores.get_mut(&canonical) {
			if !force && existing.is_ready() {
				let stats = existing.stats();
				return DaemonResponse::DocGenStatus {
					total: stats.total,
					completed: stats.ready,
					pending: stats.pending,
					is_ready: true,
				};
			}
			existing
		} else {
			let store = DocStore::load(&canonical).unwrap_or_else(|_| DocStore::new(&canonical));
			self.doc_stores.insert(canonical.clone(), store);
			self.doc_stores.get_mut(&canonical).unwrap()
		};

		// get symbols from project cache or index
		let symbols = if let Some(cached) = self.project_cache.get(&canonical) {
			cached.symbols.clone()
		} else {
			// index the project first
			let manager = IndexManager::new()
				.with_persistence()
				.with_semantic_analysis()
				.with_reference_extraction();

			match manager.index_project(&canonical) {
				Ok(result) => result.symbols,
				Err(e) => return DaemonResponse::Error(format!("indexing error: {}", e)),
			}
		};

		// populate store with symbols
		store.populate_from_symbols(&symbols);

		// get pending IDs for generation
		let pending_ids = store.get_pending_ids();
		let total = store.len();
		let pending_count = pending_ids.len();

		// initialize doc generator if needed
		if self.doc_generator.is_none() {
			eprintln!("[daemon] Initializing doc generator...");
			match DocGenerator::with_default_model() {
				Ok(gen) => {
					eprintln!("[daemon] Doc generator ready");
					self.doc_generator = Some(gen);
				}
				Err(e) => {
					eprintln!("[daemon] Failed to load doc generator: {}", e);
					return DaemonResponse::Error(format!("doc generator error: {}", e));
				}
			}
		}

		// generate docs for pending entries (synchronous for now)
		if let Some(ref mut generator) = self.doc_generator {
			let batch_size = 10; // process in batches
			let mut generated = 0;

			for chunk in pending_ids.chunks(batch_size) {
				match generator.generate_for_ids(store, chunk) {
					Ok(count) => generated += count,
					Err(e) => eprintln!("[daemon] Doc gen error: {}", e),
				}

				// save periodically
				if let Err(e) = store.save() {
					eprintln!("[daemon] Failed to save doc store: {}", e);
				}
			}

			eprintln!("[daemon] Generated {} docs for {}", generated, canonical.display());
		}

		// build cross-references if we have the semantic graph
		if let Some(cached) = self.project_cache.get(&canonical) {
			if let Some(ref graph) = cached.graph {
				let linker = DocLinker::new();
				linker.build_links(store, graph, &cached.symbols);
			}
		}

		// final save
		if let Err(e) = store.save() {
			eprintln!("[daemon] Failed to save doc store: {}", e);
		}

		let stats = store.stats();
		DaemonResponse::DocGenStatus {
			total,
			completed: stats.ready,
			pending: pending_count.saturating_sub(stats.ready),
			is_ready: stats.is_complete,
		}
	}

	/// Handle DocGenStatus request - get documentation generation status
	fn handle_doc_gen_status(&self, project_path: &str) -> DaemonResponse {
		let path = PathBuf::from(project_path);
		let canonical = path.canonicalize().unwrap_or(path);

		match self.doc_stores.get(&canonical) {
			Some(store) => {
				let stats = store.stats();
				DaemonResponse::DocGenStatus {
					total: stats.total,
					completed: stats.ready,
					pending: stats.pending,
					is_ready: stats.is_complete,
				}
			}
			None => {
				// try to load from disk
				match DocStore::load(&canonical) {
					Ok(store) => {
						let stats = store.stats();
						DaemonResponse::DocGenStatus {
							total: stats.total,
							completed: stats.ready,
							pending: stats.pending,
							is_ready: stats.is_complete,
						}
					}
					Err(_) => DaemonResponse::DocGenStatus {
						total: 0,
						completed: 0,
						pending: 0,
						is_ready: false,
					},
				}
			}
		}
	}

	/// Handle GetDoc request - get documentation for a specific symbol
	fn handle_get_doc(&self, project_path: &str, symbol_name: &str) -> DaemonResponse {
		let path = PathBuf::from(project_path);
		let canonical = path.canonicalize().unwrap_or(path);

		let store = match self.doc_stores.get(&canonical) {
			Some(s) => s,
			None => {
				// try to load from disk
				match DocStore::load(&canonical) {
					Ok(s) => {
						return match s.get_by_name(symbol_name) {
							Some(entry) => DaemonResponse::Doc(Some(doc_entry_to_response(entry))),
							None => DaemonResponse::Doc(None),
						};
					}
					Err(_) => return DaemonResponse::Doc(None),
				}
			}
		};

		match store.get_by_name(symbol_name) {
			Some(entry) => DaemonResponse::Doc(Some(doc_entry_to_response(entry))),
			None => DaemonResponse::Doc(None),
		}
	}

	/// Handle SearchDocs request - search documentation
	fn handle_search_docs(&self, project_path: &str, query: &str, limit: usize) -> DaemonResponse {
		let path = PathBuf::from(project_path);
		let canonical = path.canonicalize().unwrap_or(path);

		let store = match self.doc_stores.get(&canonical) {
			Some(s) => s,
			None => {
				// try to load from disk
				match DocStore::load(&canonical) {
					Ok(s) => {
						let results: Vec<DocEntryResponse> = s
							.search_by_name(query)
							.into_iter()
							.take(limit)
							.map(doc_entry_to_response)
							.collect();
						return DaemonResponse::DocResults(results);
					}
					Err(_) => return DaemonResponse::DocResults(Vec::new()),
				}
			}
		};

		let results: Vec<DocEntryResponse> = store
			.search_by_name(query)
			.into_iter()
			.take(limit)
			.map(doc_entry_to_response)
			.collect();

		DaemonResponse::DocResults(results)
	}

	/// Evict oldest project if cache exceeds limit
	fn evict_lru_if_needed(&mut self) {
		while self.project_cache.len() > MAX_CACHED_PROJECTS {
			// Find oldest project by last_indexed timestamp
			let oldest = self
				.project_cache
				.iter()
				.min_by_key(|(_, v)| v.last_indexed)
				.map(|(k, _)| k.clone());

			if let Some(path) = oldest {
				eprintln!("[daemon] Evicting LRU project: {:?}", path);
				self.project_cache.remove(&path);
			} else {
				break;
			}
		}
	}

	/// Handle a client connection
	fn handle_client(&mut self, stream: UnixStream) -> RetrievalResult<()> {
		let mut reader = BufReader::new(&stream);
		let mut writer = &stream;

		loop {
			let mut line = String::new();
			match reader.read_line(&mut line) {
				Ok(0) => break, // client disconnected
				Ok(_) => {
					// parse request
					let request = match deserialize_request(line.trim().as_bytes()) {
						Ok(r) => r,
						Err(e) => {
							let resp = DaemonResponse::Error(format!("parse error: {}", e));
							let bytes = serialize_response(&resp).unwrap();
							writer.write_all(&bytes)?;
							continue;
						}
					};

					// handle request
					let response = self.handle_request(request);
					let bytes = serialize_response(&response)
						.map_err(|e| RetrievalError::DaemonCommunication(e.to_string()))?;
					writer.write_all(&bytes)?;
					writer.flush()?;

					// check for shutdown
					if self.shutdown.load(Ordering::SeqCst) {
						break;
					}
				}
				Err(e) => {
					eprintln!("[daemon] read error: {}", e);
					break;
				}
			}
		}

		Ok(())
	}

	/// Run the daemon (blocking)
	pub fn run(&mut self) -> RetrievalResult<()> {
		// ensure parent directory exists
		if let Some(parent) = self.socket_path.parent() {
			fs::create_dir_all(parent)?;
		}

		// remove stale socket
		let _ = fs::remove_file(&self.socket_path);

		// write PID file
		self.write_pid_file()?;

		// create Unix socket listener BEFORE loading models
		// this allows clients to connect early (they'll get "loading" responses)
		let listener = UnixListener::bind(&self.socket_path)?;
		eprintln!("[daemon] Listening on {:?}", self.socket_path);
		eprintln!("[daemon] Loading models (socket ready for status/ping)...");

		// set socket to non-blocking for model loading phase
		listener.set_nonblocking(true)?;

		// load models while accepting ping/status requests
		self.load_models_with_listener(&listener)?;

		// accept connections (models are now loaded)
		loop {
			match listener.accept() {
				Ok((stream, _)) => {
					stream.set_nonblocking(false)?;
					if let Err(e) = self.handle_client(stream) {
						eprintln!("[daemon] client error: {}", e);
					}
				}
				Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
					// no pending connection, check shutdown flag
					if self.shutdown.load(Ordering::SeqCst) {
						eprintln!("[daemon] Shutting down...");
						break;
					}
					std::thread::sleep(std::time::Duration::from_millis(100));
				}
				Err(e) => {
					eprintln!("[daemon] accept error: {}", e);
				}
			}
		}

		// cleanup
		let _ = fs::remove_file(&self.socket_path);
		let _ = fs::remove_file(self.socket_path.with_extension("pid"));

		Ok(())
	}

	/// Load models while accepting ping/status requests on the listener
	fn load_models_with_listener(&mut self, listener: &UnixListener) -> RetrievalResult<()> {
		// helper to check for and handle ping/status requests
		let check_requests = |daemon: &mut ModelDaemon, listener: &UnixListener| {
			if let Ok((stream, _)) = listener.accept() {
				let _ = stream.set_nonblocking(false);
				// only handle ping/status during loading
				if let Err(e) = daemon.handle_client_limited(stream) {
					eprintln!("[daemon] client error during loading: {}", e);
				}
			}
		};

		// Log device info
		eprintln!(
			"[daemon] Using device: {} ({})",
			self.device_info.device_type, self.device_info.device_name
		);
		if let Some(mem) = self.device_info.memory_mb {
			eprintln!("[daemon] GPU memory: {} MB", mem);
		}

		check_requests(self, listener);

		// Load embedding model
		eprintln!("[daemon] Loading embedding model: {}", self.config.embedding_model);
		match BgeEmbedder::from_model_id(&self.config.embedding_model) {
			Ok(embedder) => {
				eprintln!("[daemon] Embedding model loaded (dim={})", embedder.dim());
				self.embedder = Some(embedder);
			}
			Err(e) => {
				eprintln!("[daemon] Failed to load embedding model: {}", e);
				eprintln!("[daemon] Running in degraded mode (no embeddings)");
			}
		}

		check_requests(self, listener);

		// Load reranker model
		eprintln!("[daemon] Loading reranker model: {}", self.config.reranker_model);
		match BgeReranker::from_model_id(&self.config.reranker_model) {
			Ok(reranker) => {
				eprintln!("[daemon] Reranker model loaded");
				self.reranker = Some(reranker);
			}
			Err(e) => {
				eprintln!("[daemon] Failed to load reranker model: {}", e);
				eprintln!("[daemon] Running in degraded mode (no reranking)");
			}
		}

		check_requests(self, listener);

		// Load LLM for query expansion (this is the slow one)
		eprintln!(
			"[daemon] Loading query expansion model: {}",
			self.config.expansion_model
		);
		match QueryInterpreter::with_model_id(&self.config.expansion_model) {
			Ok(interp) => {
				eprintln!("[daemon] Query expansion model loaded");
				self.interpreter = interp;
			}
			Err(e) => {
				eprintln!("[daemon] Failed to load expansion model: {}", e);
				eprintln!("[daemon] Running in degraded mode (no query expansion)");
			}
		}

		eprintln!("[daemon] All models loaded, ready for requests");
		Ok(())
	}

	/// Handle client with limited commands (only ping/status during loading)
	fn handle_client_limited(&mut self, mut stream: UnixStream) -> RetrievalResult<()> {
		let mut reader = BufReader::new(stream.try_clone()?);
		let mut line = String::new();

		if reader.read_line(&mut line)? == 0 {
			return Ok(());
		}

		let request: DaemonRequest = deserialize_request(line.trim().as_bytes())
			.map_err(|e| RetrievalError::DaemonCommunication(format!("deserialize: {}", e)))?;

		let response = match request {
			DaemonRequest::Ping => DaemonResponse::Pong,
			DaemonRequest::Status => self.handle_status(),
			DaemonRequest::Shutdown => {
				self.shutdown.store(true, Ordering::SeqCst);
				DaemonResponse::Ok
			}
			_ => DaemonResponse::Error("Daemon is still loading models".to_string()),
		};

		let bytes = serialize_response(&response)
			.map_err(|e| RetrievalError::DaemonCommunication(format!("serialize: {}", e)))?;
		stream.write_all(&bytes)?;
		stream.flush()?;

		Ok(())
	}
}

impl Default for ModelDaemon {
	fn default() -> Self {
		Self::new()
	}
}

/// Convert a DocEntry to DocEntryResponse for IPC.
fn doc_entry_to_response(entry: &crate::retrieval::docgen::DocEntry) -> DocEntryResponse {
	DocEntryResponse {
		name: entry.name.clone(),
		kind: entry.kind.to_string(),
		file_path: entry.file_path.display().to_string(),
		line: entry.line,
		user_comment: entry.user_comment.clone(),
		llm_doc: entry.llm_doc.clone(),
		signature: entry.signature.clone(),
		depends_on: entry.links.depends_on.clone(),
		depended_by: entry.links.depended_by.clone(),
		external_deps: entry.links.external_deps.clone(),
		status: entry.status.to_string(),
	}
}
