#[derive(Debug, Clone)]
pub enum MessageSegment {
    Text(String),
    FileReference {
        full_path: String,
        display_name: String,
    },
    FolderReference {
        full_path: String,
        display_name: String,
    },
    SymbolReference {
        full_path: String,
        display_name: String,
        symbol_path: String,
        source_code: Option<String>,
    },
}

impl MessageSegment {
    pub fn debug_string(&self) -> String {
        match self {
            MessageSegment::Text(text) => format!("Text: \"{}\"", text),
            MessageSegment::FileReference {
                full_path,
                display_name,
            } => format!(
                "File: {} ({})",
                display_name, full_path
            ),
            MessageSegment::FolderReference {
                full_path,
                display_name,
            } => format!(
                "Folder: {} ({})",
                display_name, full_path
            ),
            MessageSegment::SymbolReference {
                display_name,
                symbol_path,
                ..
            } => format!(
                "Symbol: {} ({})",
                display_name, symbol_path
            ),
        }
    }

    pub fn display_text(&self) -> String {
        match self {
            MessageSegment::Text(text) => text.clone(),
            MessageSegment::FileReference {
                display_name, ..
            } => display_name.clone(),
            MessageSegment::FolderReference {
                display_name, ..
            } => display_name.clone(),
            MessageSegment::SymbolReference {
                display_name, ..
            } => display_name.clone(),
        }
    }
}
