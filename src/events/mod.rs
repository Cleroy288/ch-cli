use crossterm::event::{self, Event, KeyEventKind};
use std::io;
use std::time::Duration;

use crate::app::App;

/// Poll and handle terminal events
/// Returns true if the app should quit
pub fn handle_events(app: &mut App) -> io::Result<bool> {
    // Poll for events with a small timeout to keep the UI responsive
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            // Only handle key press events (ignore release/repeat)
            if key.kind == KeyEventKind::Press {
                return Ok(app.handle_key(key.code, key.modifiers));
            }
        }
    }
    Ok(false)
}
