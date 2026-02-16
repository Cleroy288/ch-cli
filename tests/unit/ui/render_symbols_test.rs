//! Tests for ui::components::picker::render_symbols

use rustean::indexer::symbols::SymbolKind;
use rustean::ui::components::picker::render_symbols::{
    symbol_kind_icon,
};

/// Function kind maps to function icon
#[test]
fn icon_function_returns_f() {
    assert_eq!(symbol_kind_icon(SymbolKind::Function), "ƒ");
}

/// Method kind shares icon with Function
#[test]
fn icon_method_returns_f() {
    assert_eq!(symbol_kind_icon(SymbolKind::Method), "ƒ");
}

/// Struct kind maps to S
#[test]
fn icon_struct_returns_s() {
    assert_eq!(symbol_kind_icon(SymbolKind::Struct), "S");
}

/// Enum kind maps to E
#[test]
fn icon_enum_returns_e() {
    assert_eq!(symbol_kind_icon(SymbolKind::Enum), "E");
}

/// All SymbolKind variants produce non-empty icons
#[test]
fn all_variants_have_non_empty_icon() {
    // Arrange
    let all_kinds = vec![
        SymbolKind::Function,
        SymbolKind::Method,
        SymbolKind::Struct,
        SymbolKind::Enum,
        SymbolKind::Trait,
        SymbolKind::Impl,
        SymbolKind::Constant,
        SymbolKind::Static,
        SymbolKind::TypeAlias,
        SymbolKind::Module,
        SymbolKind::Macro,
        SymbolKind::EnumVariant,
        SymbolKind::Field,
        SymbolKind::DocumentChunk,
    ];

    // Act + Assert
    for kind in all_kinds {
        let icon = symbol_kind_icon(kind);
        assert!(
            !icon.is_empty(),
            "{:?} has empty icon",
            kind,
        );
    }
}
