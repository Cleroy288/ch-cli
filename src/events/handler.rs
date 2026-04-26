use crossterm::event::{
	self, Event, KeyEventKind, MouseEventKind,
};
use std::io;
use std::time::Duration;

use crate::app::App;

/// Poll and handle terminal events
/// Returns true if the app should quit
pub fn handle_events(
	app: &mut App,
) -> io::Result<bool> {
	if !event::poll(
		Duration::from_millis(100),
	)? {
		return Ok(false);
	}
	match event::read()? {
		Event::Key(key)
			if key.kind == KeyEventKind::Press =>
		{
			Ok(app.handle_key(
				key.code, key.modifiers,
			))
		}
		Event::Mouse(mouse) => {
			handle_mouse_event(app, &mouse);
			Ok(false)
		}
		Event::Paste(data) => {
			app.handle_paste(&data);
			Ok(false)
		}
		_ => Ok(false),
	}
}

/// Route mouse events to app handlers
fn handle_mouse_event(
	app: &mut App,
	mouse: &crossterm::event::MouseEvent,
) {
	match mouse.kind {
		MouseEventKind::Down(
			crossterm::event::MouseButton::Left,
		) => {
			app.handle_mouse(
				mouse.column, mouse.row,
			);
		}
		MouseEventKind::ScrollUp => {
			app.handle_scroll_up();
		}
		MouseEventKind::ScrollDown => {
			app.handle_scroll_down();
		}
		_ => {}
	}
}
