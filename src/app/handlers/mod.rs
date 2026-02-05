/// Keyboard input handlers module.
///
/// This module contains separate handlers for different input contexts:
/// - `input`: Regular input handling (typing, cursor movement, enter)
/// - `picker`: Picker-specific input handling (file/folder selection)
///
/// By separating handlers, we follow the Single Responsibility Principle
/// and avoid deep nesting in a single large function.
pub mod input;
pub mod picker;
