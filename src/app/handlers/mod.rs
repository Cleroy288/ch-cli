/// Keyboard input handlers module.
///
/// This module contains separate handlers:
/// - `input_keys`: Key event routing
/// - `input_edit`: Text editing and file reference operations
/// - `cursor_movement`: Cursor left/right movement
/// - `picker_keys`: Picker key event routing
/// - `picker_actions`: Picker action handlers
///
/// By separating handlers, we follow SRP and avoid deep nesting.
mod cursor_movement;
mod input_edit;
mod input_keys;
mod picker_actions;
mod picker_keys;
