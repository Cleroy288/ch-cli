use crate::indexer::parser::RustParser;
use crate::indexer::symbols::Symbol;
use crate::message::MessageSegment;

/// source code for each SymbolReference segment.
///
/// Called at message-send time (lazy extraction).
pub fn resolve_symbols(
    segments: &mut [MessageSegment],
) {
    for seg in segments.iter_mut() {
        attach_source_code(seg);
    }
}

/// Attach source code to a SymbolReference segment
fn attach_source_code(seg: &mut MessageSegment) {
    let MessageSegment::SymbolReference {
        full_path,
        symbol_path,
        source_code,
        ..
    } = seg
    else {
        return;
    };
    if source_code.is_some() {
        return;
    }

    let code = extract_source(full_path, symbol_path);
    *source_code = code;
}

/// Extract source code for a symbol from its file
fn extract_source(
    file_path: &str,
    symbol_path: &str,
) -> Option<String> {
    let mut parser = RustParser::new().ok()?;
    let symbols = parser.parse_file(file_path).ok()?;
    let symbol = find_symbol(&symbols, symbol_path)?;
    read_symbol_source(file_path, symbol)
}

/// Find a symbol by its path (e.g. "Struct::method")
fn find_symbol<'a>(
    symbols: &'a [Symbol],
    path: &str,
) -> Option<&'a Symbol> {
    let parts: Vec<&str> = path.split("::").collect();
    let leaf = parts.last()?;
    let parent = if parts.len() > 1 {
        Some(parts[parts.len() - 2])
    } else {
        None
    };

    symbols.iter().find(|sym| {
        sym.name == *leaf
            && sym.parent.as_deref() == parent
    })
}

fn read_symbol_source(
    file_path: &str,
    symbol: &Symbol,
) -> Option<String> {
    let source = std::fs::read_to_string(file_path).ok()?;
    let offset = symbol.location.byte_offset;
    let length = symbol.location.byte_length;
    if length == 0 {
        return None;
    }
    let end = offset + length;
    if end > source.len() {
        return None;
    }
    Some(source[offset..end].to_string())
}
