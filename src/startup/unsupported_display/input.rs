use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyEvent, KeyEventKind},
    terminal,
};

/// Wait for a key press before continuing
pub fn wait_for_keypress() -> io::Result<()> {
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
    Ok(())
}
