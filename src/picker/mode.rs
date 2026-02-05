/// The mode of the picker.
///
/// Represents the current state of the file/folder picker.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PickerMode {
    /// Not in picker mode
    Inactive,
    /// Choosing between file or folder
    ChoosingType,
    /// Picking a file
    File,
    /// Picking a folder
    Folder,
}
