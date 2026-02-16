//! CLI module for semantic code indexer commands.
//!
//! This module provides command-line interface for:
//! - `index` - Build/rebuild the code index
//! - `search` - Search for symbols
//! - `goto` - Jump to symbol definition
//! - `refs` - Find all references to a symbol
//! - `tui` - Launch the interactive TUI (default)

use clap::{Parser, Subcommand};

pub mod commands;

/// rustean: A semantic code indexer and TUI assistant
#[derive(Parser, Debug, Default)]
#[command(name = "rustean")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available CLI commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build or rebuild the code index for the current project
    Index {
        /// Path to the project root (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: String,

        /// Enable semantic analysis for name resolution
        #[arg(short, long)]
        semantic: bool,

        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Search for symbols in the codebase
    Search {
        /// The search query
        query: String,

        /// Maximum number of results to show
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Use fuzzy matching
        #[arg(short, long)]
        fuzzy: bool,

        /// Filter by symbol kind (function, struct, enum, trait, etc.)
        #[arg(short, long)]
        kind: Option<String>,

        /// Use semantic (hybrid) search combining keywords
        /// and embeddings
        #[arg(long)]
        semantic: bool,

        /// Expand results with contextual information
        /// (parent, callers, callees)
        #[arg(short, long)]
        context: bool,

        /// Rerank results using cross-encoder for better
        /// relevance
        #[arg(short, long)]
        rerank: bool,

        /// Show full file content for results
        #[arg(long)]
        full: bool,
    },

    /// Manage the ML model daemon
    Daemon {
        #[command(subcommand)]
        action: DaemonAction,
    },

    /// Generate embeddings for all indexed symbols
    Embed {
        /// Path to the project root (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: String,

        /// Force re-embedding all symbols
        #[arg(short, long)]
        force: bool,
    },

    /// Jump to symbol definition (go to definition)
    Goto {
        /// The symbol name to find
        symbol: String,
    },

    /// Find all references to a symbol
    Refs {
        /// The symbol name to find references for
        symbol: String,

        /// Include the definition location
        #[arg(short, long)]
        include_definition: bool,
    },

    /// List all symbols in a file
    Symbols {
        /// Path to the file (defaults to current file)
        #[arg(short, long)]
        file: Option<String>,

        /// Filter by symbol kind
        #[arg(short, long)]
        kind: Option<String>,
    },

    /// Show index statistics
    Stats,

    /// Launch the interactive TUI
    Tui,

    /// Run the full agentic retrieval pipeline
    Retrieve {
        /// Natural language query
        query: String,

        /// Maximum number of results
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Maximum tokens in output
        #[arg(short, long, default_value = "8000")]
        max_tokens: usize,

        /// Disable query expansion (LLM interpretation)
        #[arg(long)]
        no_expand: bool,

        /// Disable reranking
        #[arg(long)]
        no_rerank: bool,

        /// Disable context expansion
        #[arg(long)]
        no_context: bool,

        /// Output raw XML (for LLM consumption)
        #[arg(long)]
        xml: bool,

        /// Use structured output with separate code, doc, notes sections
        #[arg(long)]
        structured: bool,

        /// RRF score threshold for relevance filtering (default: 0.015)
        #[arg(long, default_value = "0.015")]
        threshold: f32,

        /// Minimum results per content type (default: 1)
        #[arg(long, default_value = "1")]
        min_results: usize,
    },

    /// Manage LLM-generated documentation
    Docs {
        #[command(subcommand)]
        action: DocsAction,
    },

    /// Manage interaction memory
    /// (show, search, add, stats)
    Memory {
        #[command(subcommand)]
        action: MemoryAction,
    },

    /// Run MCP stdio server for Claude Code
    #[command(name = "mcp-server")]
    McpServer,

    /// Show detailed information about a symbol
    /// (definition, references, callers)
    Info {
        /// The symbol name to look up
        symbol: String,

        /// Show source code snippet
        #[arg(short, long)]
        code: bool,

        /// Show callers (who calls this symbol)
        #[arg(long)]
        callers: bool,

        /// Show callees (what this symbol calls)
        #[arg(long)]
        callees: bool,

        /// Show all references
        #[arg(short, long)]
        refs: bool,

        /// Show everything (code + callers + callees + refs)
        #[arg(short, long)]
        all: bool,
    },
}

/// Daemon subcommand actions
#[derive(Subcommand, Debug)]
pub enum DaemonAction {
    /// Start the model daemon
    Start,
    /// Stop the model daemon
    Stop,
    /// Show daemon status
    Status,
    /// Restart the daemon
    Restart,
    /// Run the daemon in foreground (internal use)
    Run {
        /// Socket path
        #[arg(long)]
        socket: Option<String>,
    },
}

/// Memory subcommand actions
#[derive(Subcommand, Debug)]
pub enum MemoryAction {
    /// Show recent interactions
    Show {
        /// Maximum entries to show
        #[arg(short, long, default_value = "20")]
        limit: usize,

        /// Filter by session ID
        #[arg(short, long)]
        session: Option<String>,
    },
    /// Search stored interactions
    Search {
        /// Search query
        query: String,
        /// Maximum results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// Show memory statistics
    Stats,
    /// Add a new interaction manually
    Add {
        /// User input text
        #[arg(short, long)]
        input: String,
        /// AI response text
        #[arg(short, long)]
        response: String,
        /// Response type (answer, question, code_change)
        #[arg(long, default_value = "answer")]
        response_type: String,
    },
}

/// Docs subcommand actions
#[derive(Subcommand, Debug)]
pub enum DocsAction {
    /// Generate documentation for all symbols
    Generate {
        /// Force regeneration even if docs exist
        #[arg(short, long)]
        force: bool,
    },
    /// Show documentation generation status
    Status,
    /// Show documentation for a symbol
    Show {
        /// Symbol name to look up
        symbol: String,
    },
    /// Search documentation
    Search {
        /// Search query
        query: String,
        /// Maximum results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
}


