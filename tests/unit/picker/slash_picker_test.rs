//! Tests for slash picker state transitions

use rustean::picker::{Picker, PickerMode};

/// activate_slash sets SlashCommand mode
#[test]
fn activate_slash_sets_mode() {
    // Arrange
    let mut picker = Picker::default();

    // Act
    picker.activate_slash(0);

    // Assert
    assert_eq!(
        *picker.mode(),
        PickerMode::SlashCommand
    );
    assert!(picker.is_active());
    assert_eq!(picker.trigger_position(), 0);
}

/// activate_slash clears query
#[test]
fn activate_slash_clears_query() {
    // Arrange
    let mut picker = Picker::default();
    picker.activate(0);
    picker.push_query('x');

    // Act
    picker.activate_slash(0);

    // Assert
    assert_eq!(picker.query(), "");
}

/// activate_slash_arg transitions to SlashArg
#[test]
fn activate_slash_arg_sets_mode() {
    // Arrange
    let mut picker = Picker::default();
    picker.activate_slash(0);

    // Act
    picker.activate_slash_arg("model");

    // Assert
    assert_eq!(
        *picker.mode(),
        PickerMode::SlashArg {
            command: "model".to_string()
        }
    );
}

/// back_to_slash_command returns to SlashCommand
#[test]
fn back_to_slash_command_works() {
    // Arrange
    let mut picker = Picker::default();
    picker.activate_slash(0);
    picker.activate_slash_arg("model");
    picker.push_query('h');

    // Act
    picker.back_to_slash_command();

    // Assert
    assert_eq!(
        *picker.mode(),
        PickerMode::SlashCommand
    );
    assert_eq!(picker.query(), "");
}

/// is_slash_mode true for SlashCommand
#[test]
fn is_slash_mode_true_for_command() {
    // Arrange
    let mut picker = Picker::default();
    picker.activate_slash(0);

    // Assert
    assert!(picker.is_slash_mode());
}

/// is_slash_mode true for SlashArg
#[test]
fn is_slash_mode_true_for_arg() {
    // Arrange
    let mut picker = Picker::default();
    picker.activate_slash(0);
    picker.activate_slash_arg("model");

    // Assert
    assert!(picker.is_slash_mode());
}

/// is_slash_mode false for Browse
#[test]
fn is_slash_mode_false_for_browse() {
    // Arrange
    let mut picker = Picker::default();
    picker.activate(0);

    // Assert
    assert!(!picker.is_slash_mode());
}
