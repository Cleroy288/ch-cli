//! Startup flow for ch-cli.
//!
//! Handles index checking, user prompting, and progress display during indexing.

use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};

use crate::indexer::crawler::Crawler;
use crate::indexer::state::IndexState;
use crate::indexer::{
    ChangeSet, CodebaseAnalysis, CodebaseAnalyzer, DetectedLanguage, IndexManager, IndexResult,
    Language,
};

/// Result of the startup check
pub enum StartupAction {
    /// User wants to index the codebase (full index)
    Index,
    /// User wants to update the index (incremental)
    Update(ChangeSet),
    /// User declined indexing
    Skip,
    /// Index already exists and is up to date
    UpToDate,
    /// User wants to quit
    Quit,
}

/// Check if an index exists for the current directory
pub fn check_index_exists() -> bool {
    IndexManager::has_index(".")
}

/// Analyze the codebase to detect its primary language
pub fn analyze_codebase() -> CodebaseAnalysis {
    let analyzer = CodebaseAnalyzer::new();
    analyzer.analyze(".")
}

/// Display a message when the primary language is not supported for indexing
pub fn display_unsupported_language_message(
    primary_lang: DetectedLanguage,
    supported_count: usize,
    total_count: usize,
) -> io::Result<()> {
    let mut stdout = io::stdout();

    // Clear screen and show message
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    println!();
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("  ch-cli"),
        ResetColor,
        Print(" - Semantic Code Indexer\n\n")
    )?;

    // Warning header
    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("  Language Not Supported\n\n"),
        ResetColor
    )?;

    // Detected language info
    execute!(
        stdout,
        SetForegroundColor(Color::White),
        Print("  Detected primary language: "),
        SetForegroundColor(Color::Cyan),
        Print(format!("{}\n\n", primary_lang.display_name())),
        ResetColor
    )?;

    // Supported languages list
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("  Currently supported languages:\n"),
        ResetColor
    )?;

    for lang in Language::all_supported() {
        execute!(
            stdout,
            SetForegroundColor(Color::Green),
            Print(format!("    - {}\n", lang.display_name())),
            ResetColor
        )?;
    }
    println!();

    // File stats if relevant
    if supported_count > 0 {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print(format!(
                "  Found {} supported file(s) out of {} total source files.\n",
                supported_count, total_count
            )),
            Print("  Indexing will be limited to supported files only.\n\n"),
            ResetColor
        )?;
    } else {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("  No supported files found in this codebase.\n"),
            Print("  Semantic indexing will be skipped.\n\n"),
            ResetColor
        )?;
    }

    // Continue message
    execute!(
        stdout,
        SetForegroundColor(Color::White),
        Print("  Press any key to continue to the TUI...\n"),
        ResetColor
    )?;
    stdout.flush()?;

    // Wait for key press
    terminal::enable_raw_mode()?;
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent {
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                break;
            }
        }
    }
    terminal::disable_raw_mode()?;

    // Clear screen before continuing
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    Ok(())
}

/// Check if the codebase has changes since last index
pub fn detect_codebase_changes() -> Option<ChangeSet> {
    if !IndexState::exists(".".as_ref()) {
        return None;
    }

    // Load existing state
    let state = match IndexState::load(".".as_ref()) {
        Ok(s) => s,
        Err(_) => return None,
    };

    // Discover current files
    let crawler = Crawler::new();
    let current_files: Vec<PathBuf> = crawler.discover_files(".");

    // Detect changes
    let changes = state.detect_changes(&current_files);

    if changes.has_changes() {
        Some(changes)
    } else {
        None
    }
}

/// Prompt the user to decide whether to update the index (changes detected)
pub fn prompt_for_update(changes: &ChangeSet) -> io::Result<StartupAction> {
    let mut stdout = io::stdout();

    // Clear screen and show prompt
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    // Display the prompt
    println!();
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("  ch-cli"),
        ResetColor,
        Print(" - Semantic Code Indexer\n\n")
    )?;

    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("  Changes detected in your codebase!\n\n"),
        ResetColor
    )?;

    // Show change summary
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;

    if !changes.added.is_empty() {
        execute!(
            stdout,
            SetForegroundColor(Color::Green),
            Print(format!("    + {} new file(s)\n", changes.added.len())),
        )?;
    }
    if !changes.modified.is_empty() {
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("    ~ {} modified file(s)\n", changes.modified.len())),
        )?;
    }
    if !changes.deleted.is_empty() {
        execute!(
            stdout,
            SetForegroundColor(Color::Red),
            Print(format!("    - {} deleted file(s)\n", changes.deleted.len())),
        )?;
    }

    execute!(stdout, ResetColor, Print("\n"))?;

    execute!(
        stdout,
        Print("  Would you like to update your index?\n\n"),
        SetForegroundColor(Color::White),
        Print("    ["),
        SetForegroundColor(Color::Green),
        Print("Y"),
        SetForegroundColor(Color::White),
        Print("]es  "),
        Print("["),
        SetForegroundColor(Color::Red),
        Print("N"),
        SetForegroundColor(Color::White),
        Print("]o  "),
        Print("["),
        SetForegroundColor(Color::Yellow),
        Print("Q"),
        SetForegroundColor(Color::White),
        Print("]uit\n\n"),
        ResetColor
    )?;

    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("  Press Y, N, or Q: "),
        ResetColor
    )?;
    stdout.flush()?;

    // Enable raw mode to capture single key press
    terminal::enable_raw_mode()?;

    let result = loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                        break StartupAction::Update(changes.clone());
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        break StartupAction::Skip;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        break StartupAction::Quit;
                    }
                    _ => {}
                }
            }
        }
    };

    terminal::disable_raw_mode()?;
    println!();

    Ok(result)
}

/// Prompt the user to decide whether to index the codebase
pub fn prompt_for_indexing() -> io::Result<StartupAction> {
    let mut stdout = io::stdout();

    // Clear screen and show prompt
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    // Display the prompt
    println!();
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("  ch-cli"),
        ResetColor,
        Print(" - Semantic Code Indexer\n\n")
    )?;

    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("  No code index found for this project.\n\n"),
        ResetColor
    )?;

    println!("  Indexing your codebase enables:");
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print("    - Fast symbol search across all files\n"),
        Print("    - Go-to-definition functionality\n"),
        Print("    - Find all references to a symbol\n"),
        Print("    - Semantic code understanding\n\n"),
        ResetColor
    )?;

    execute!(
        stdout,
        Print("  Would you like to index your codebase now?\n\n"),
        SetForegroundColor(Color::White),
        Print("    ["),
        SetForegroundColor(Color::Green),
        Print("Y"),
        SetForegroundColor(Color::White),
        Print("]es  "),
        Print("["),
        SetForegroundColor(Color::Red),
        Print("N"),
        SetForegroundColor(Color::White),
        Print("]o  "),
        Print("["),
        SetForegroundColor(Color::Yellow),
        Print("Q"),
        SetForegroundColor(Color::White),
        Print("]uit\n\n"),
        ResetColor
    )?;

    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("  Press Y, N, or Q: "),
        ResetColor
    )?;
    stdout.flush()?;

    // Enable raw mode to capture single key press
    terminal::enable_raw_mode()?;

    let result = loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                        break StartupAction::Index;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        break StartupAction::Skip;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        break StartupAction::Quit;
                    }
                    _ => {}
                }
            }
        }
    };

    terminal::disable_raw_mode()?;
    println!();

    Ok(result)
}

/// Display a progress bar during indexing (like Augment/Auggie style)
pub fn index_with_progress() -> io::Result<Option<IndexResult>> {
    let mut stdout = io::stdout();

    // Clear screen
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    println!();
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("  Indexing codebase...\n\n"),
        ResetColor
    )?;

    // Progress tracking
    let files_processed = Arc::new(AtomicUsize::new(0));
    let total_files = Arc::new(AtomicUsize::new(0));
    let current_file = Arc::new(std::sync::Mutex::new(String::new()));
    let indexing_done = Arc::new(AtomicBool::new(false));

    let files_processed_clone = files_processed.clone();
    let total_files_clone = total_files.clone();
    let current_file_clone = current_file.clone();
    let indexing_done_clone = indexing_done.clone();

    // Start indexing in a separate thread
    let handle = std::thread::spawn(move || {
        let manager = IndexManager::new()
            .with_persistence()
            .with_semantic_analysis()
            .with_reference_extraction()
            .on_progress(move |current, total, path| {
                files_processed_clone.store(current, Ordering::SeqCst);
                total_files_clone.store(total, Ordering::SeqCst);
                if let Ok(mut file) = current_file_clone.lock() {
                    *file = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();
                }
            });

        let result = manager.index_project(".");
        indexing_done_clone.store(true, Ordering::SeqCst);
        result
    });

    // Animation frames for spinner
    let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let mut spinner_idx = 0;
    let start_time = Instant::now();

    // Progress bar width
    let bar_width = 40;

    // Display loop
    while !indexing_done.load(Ordering::SeqCst) {
        let processed = files_processed.load(Ordering::SeqCst);
        let total = total_files.load(Ordering::SeqCst).max(1);
        let current = current_file.lock().map(|f| f.clone()).unwrap_or_default();

        // Calculate progress
        let progress = (processed as f64 / total as f64).min(1.0);
        let filled = (progress * bar_width as f64) as usize;
        let empty = bar_width - filled;

        // Build progress bar
        let bar: String = format!(
            "{}{}",
            "█".repeat(filled),
            "░".repeat(empty)
        );

        // Spinner
        let spinner = spinner_frames[spinner_idx % spinner_frames.len()];
        spinner_idx += 1;

        // Move cursor and clear line
        execute!(
            stdout,
            cursor::MoveTo(0, 3),
            terminal::Clear(ClearType::CurrentLine)
        )?;

        // Display progress bar
        execute!(
            stdout,
            SetForegroundColor(Color::Green),
            Print(format!("  {} ", spinner)),
            SetForegroundColor(Color::Cyan),
            Print("["),
            SetForegroundColor(Color::Green),
            Print(&bar),
            SetForegroundColor(Color::Cyan),
            Print("]"),
            SetForegroundColor(Color::White),
            Print(format!(" {:.0}%", progress * 100.0)),
            ResetColor
        )?;

        // Display file count
        execute!(
            stdout,
            cursor::MoveTo(0, 5),
            terminal::Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("  Files: {}/{}", processed, total)),
            ResetColor
        )?;

        // Display current file (truncated if too long)
        let display_file = if current.len() > 50 {
            format!("...{}", &current[current.len() - 47..])
        } else {
            current
        };

        execute!(
            stdout,
            cursor::MoveTo(0, 6),
            terminal::Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("  Current: {}", display_file)),
            ResetColor
        )?;

        // Elapsed time
        let elapsed = start_time.elapsed().as_secs();
        execute!(
            stdout,
            cursor::MoveTo(0, 7),
            terminal::Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("  Elapsed: {}s", elapsed)),
            ResetColor
        )?;

        stdout.flush()?;

        // Small delay to avoid excessive CPU usage
        std::thread::sleep(Duration::from_millis(80));
    }

    // Wait for indexing thread to complete
    let result = handle.join().map_err(|_| {
        io::Error::new(io::ErrorKind::Other, "Indexing thread panicked")
    })?;

    // Clear the progress display
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(ClearType::All)
    )?;

    match result {
        Ok(index_result) => Ok(Some(index_result)),
        Err(_) => Ok(None),
    }
}

/// Run the complete startup flow
pub fn run_startup() -> io::Result<Option<IndexResult>> {
    // Step 1: Analyze codebase language
    let analysis = analyze_codebase();

    // Step 2: Check if primary language is supported
    if !analysis.is_primary_supported && !analysis.has_supported_files() {
        // No supported files at all - show message and skip indexing
        if let Some(primary) = analysis.primary_language {
            display_unsupported_language_message(primary, 0, analysis.total_source_files)?;
        }
        return Ok(None); // Skip indexing, continue to TUI
    }

    // Step 3: Check if index already exists
    if check_index_exists() {
        // Index exists - check for changes
        if let Some(changes) = detect_codebase_changes() {
            // Changes detected, prompt for update
            match prompt_for_update(&changes)? {
                StartupAction::Update(_) => {
                    // User wants to update - run incremental index
                    index_with_progress_incremental(&changes)
                }
                StartupAction::Skip => {
                    // User declined, continue with stale index
                    Ok(None)
                }
                StartupAction::Quit => {
                    // User wants to quit
                    Err(io::Error::new(io::ErrorKind::Interrupted, "User quit"))
                }
                _ => Ok(None),
            }
        } else {
            // No changes, index is up to date
            Ok(None)
        }
    } else {
        // No index exists, prompt for initial indexing
        // Show partial support note if primary language isn't supported but we have some supported files
        let show_partial_note =
            !analysis.is_primary_supported && analysis.has_supported_files();

        match prompt_for_indexing_with_analysis(show_partial_note, &analysis)? {
            StartupAction::Index => {
                // User wants to index
                index_with_progress()
            }
            StartupAction::Skip => {
                // User declined, continue without indexing
                Ok(None)
            }
            StartupAction::Quit => {
                // User wants to quit
                Err(io::Error::new(io::ErrorKind::Interrupted, "User quit"))
            }
            _ => Ok(None),
        }
    }
}

/// Prompt for indexing with optional language analysis info
pub fn prompt_for_indexing_with_analysis(
    show_partial_note: bool,
    analysis: &CodebaseAnalysis,
) -> io::Result<StartupAction> {
    let mut stdout = io::stdout();

    // Clear screen and show prompt
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    // Display the prompt
    println!();
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("  ch-cli"),
        ResetColor,
        Print(" - Semantic Code Indexer\n\n")
    )?;

    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("  No code index found for this project.\n\n"),
        ResetColor
    )?;

    // Show partial support note if applicable
    if show_partial_note {
        if let Some(primary) = analysis.primary_language {
            execute!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print(format!(
                    "  Note: Primary language ({}) is not yet supported.\n",
                    primary.display_name()
                )),
                ResetColor
            )?;
        }

        let supported_count = analysis.supported_file_count();
        if supported_count > 0 {
            execute!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("  Will index {} supported file(s).\n\n", supported_count)),
                ResetColor
            )?;
        }
    }

    println!("  Indexing your codebase enables:");
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print("    - Fast symbol search across all files\n"),
        Print("    - Go-to-definition functionality\n"),
        Print("    - Find all references to a symbol\n"),
        Print("    - Semantic code understanding\n\n"),
        ResetColor
    )?;

    execute!(
        stdout,
        Print("  Would you like to index your codebase now?\n\n"),
        SetForegroundColor(Color::White),
        Print("    ["),
        SetForegroundColor(Color::Green),
        Print("Y"),
        SetForegroundColor(Color::White),
        Print("]es  "),
        Print("["),
        SetForegroundColor(Color::Red),
        Print("N"),
        SetForegroundColor(Color::White),
        Print("]o  "),
        Print("["),
        SetForegroundColor(Color::Yellow),
        Print("Q"),
        SetForegroundColor(Color::White),
        Print("]uit\n\n"),
        ResetColor
    )?;

    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("  Press Y, N, or Q: "),
        ResetColor
    )?;
    stdout.flush()?;

    // Enable raw mode to capture single key press
    terminal::enable_raw_mode()?;

    let result = loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                        break StartupAction::Index;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        break StartupAction::Skip;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        break StartupAction::Quit;
                    }
                    _ => {}
                }
            }
        }
    };

    terminal::disable_raw_mode()?;
    println!();

    Ok(result)
}

/// Display a progress bar during incremental indexing
pub fn index_with_progress_incremental(changes: &ChangeSet) -> io::Result<Option<IndexResult>> {
    let mut stdout = io::stdout();

    // Clear screen
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    println!();
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("  Updating index...\n\n"),
        ResetColor
    )?;

    // Show what we're updating
    let files_to_process = changes.added.len() + changes.modified.len();
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  {} file(s) to re-index\n\n", files_to_process)),
        ResetColor
    )?;

    // Progress tracking
    let files_processed = Arc::new(AtomicUsize::new(0));
    let total_files = Arc::new(AtomicUsize::new(0));
    let current_file = Arc::new(std::sync::Mutex::new(String::new()));
    let indexing_done = Arc::new(AtomicBool::new(false));

    let files_processed_clone = files_processed.clone();
    let total_files_clone = total_files.clone();
    let current_file_clone = current_file.clone();
    let indexing_done_clone = indexing_done.clone();

    // Start indexing in a separate thread
    let handle = std::thread::spawn(move || {
        let manager = IndexManager::new()
            .with_persistence()
            .with_semantic_analysis()
            .with_reference_extraction()
            .on_progress(move |current, total, path| {
                files_processed_clone.store(current, Ordering::SeqCst);
                total_files_clone.store(total, Ordering::SeqCst);
                if let Ok(mut file) = current_file_clone.lock() {
                    *file = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();
                }
            });

        // Incremental index - will only process changed files
        let result = manager.index_project(".");
        indexing_done_clone.store(true, Ordering::SeqCst);
        result
    });

    // Animation frames for spinner
    let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let mut spinner_idx = 0;
    let start_time = Instant::now();

    // Progress bar width
    let bar_width = 40;

    // Display loop
    while !indexing_done.load(Ordering::SeqCst) {
        let processed = files_processed.load(Ordering::SeqCst);
        let total = total_files.load(Ordering::SeqCst).max(1);
        let current = current_file.lock().map(|f| f.clone()).unwrap_or_default();

        // Calculate progress
        let progress = (processed as f64 / total as f64).min(1.0);
        let filled = (progress * bar_width as f64) as usize;
        let empty = bar_width - filled;

        // Build progress bar
        let bar: String = format!(
            "{}{}",
            "█".repeat(filled),
            "░".repeat(empty)
        );

        // Spinner
        let spinner = spinner_frames[spinner_idx % spinner_frames.len()];
        spinner_idx += 1;

        // Move cursor and clear line
        execute!(
            stdout,
            cursor::MoveTo(0, 5),
            terminal::Clear(ClearType::CurrentLine)
        )?;

        // Display progress bar
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("  {} ", spinner)),
            SetForegroundColor(Color::Cyan),
            Print("["),
            SetForegroundColor(Color::Yellow),
            Print(&bar),
            SetForegroundColor(Color::Cyan),
            Print("]"),
            SetForegroundColor(Color::White),
            Print(format!(" {:.0}%", progress * 100.0)),
            ResetColor
        )?;

        // Display file count
        execute!(
            stdout,
            cursor::MoveTo(0, 7),
            terminal::Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("  Files: {}/{}", processed, total)),
            ResetColor
        )?;

        // Display current file (truncated if too long)
        let display_file = if current.len() > 50 {
            format!("...{}", &current[current.len() - 47..])
        } else {
            current
        };

        execute!(
            stdout,
            cursor::MoveTo(0, 8),
            terminal::Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("  Current: {}", display_file)),
            ResetColor
        )?;

        // Elapsed time
        let elapsed = start_time.elapsed().as_secs();
        execute!(
            stdout,
            cursor::MoveTo(0, 9),
            terminal::Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("  Elapsed: {}s", elapsed)),
            ResetColor
        )?;

        stdout.flush()?;

        // Small delay to avoid excessive CPU usage
        std::thread::sleep(Duration::from_millis(80));
    }

    // Wait for indexing thread to complete
    let result = handle.join().map_err(|_| {
        io::Error::new(io::ErrorKind::Other, "Indexing thread panicked")
    })?;

    // Clear the progress display
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(ClearType::All)
    )?;

    match result {
        Ok(index_result) => Ok(Some(index_result)),
        Err(_) => Ok(None),
    }
}
