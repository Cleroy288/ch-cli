use clap::{Parser, Subcommand};

/// rustean: A semantic code indexer and TUI assistant
#[derive(Parser, Debug, Default)]
#[command(name = "rustean")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Index {
        /// Path to the project root
        #[arg(short, long, default_value = ".")]
        path: String,

        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    Search {
        /// The search query
        query: String,

        /// Maximum number of results to show
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Use fuzzy matching
        #[arg(short, long)]
        fuzzy: bool,

        #[arg(short, long)]
        kind: Option<String>,

        /// Show full file content for results
        #[arg(long)]
        full: bool,
    },

    /// Jump to symbol definition
    Goto {
        /// The symbol name to find
        symbol: String,
    },

    Refs {
        /// The symbol name to find references for
        symbol: String,

        /// Include the definition location
        #[arg(short, long)]
        include_definition: bool,
    },

    /// List all symbols in a file
    Symbols {
        /// Path to the file
        #[arg(short, long)]
        file: Option<String>,

        #[arg(short, long)]
        kind: Option<String>,
    },

    /// Show index statistics
    Stats,

    /// Launch the interactive TUI
    Tui,

    /// Manage interaction memory
    Memory {
        #[command(subcommand)]
        action: MemoryAction,
    },

    #[command(name = "mcp-server")]
    McpServer,

    /// Show detailed information about a symbol
    Info {
        /// The symbol name to look up
        symbol: String,

        /// Show source code snippet
        #[arg(short, long)]
        code: bool,

        /// Show callers
        #[arg(long)]
        callers: bool,

        /// Show callees
        #[arg(long)]
        callees: bool,

        /// Show all references
        #[arg(short, long)]
        refs: bool,

        /// Show everything
        #[arg(short, long)]
        all: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum MemoryAction {
    /// Show recent interactions
    Show {
        /// Maximum entries to show
        #[arg(short, long, default_value = "20")]
        limit: usize,

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
    Add {
        /// User input text
        #[arg(short, long)]
        input: String,
        /// AI response text
        #[arg(short, long)]
        response: String,
        #[arg(long, default_value = "answer")]
        response_type: String,
    },
}
