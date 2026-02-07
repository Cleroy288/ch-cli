//! Tree-sitter query for extracting Rust symbol definitions.

/// Tree-sitter query for extracting Rust symbols.
///
/// This query captures:
/// - Function definitions (with visibility and name)
/// - Struct definitions
/// - Enum definitions
/// - Trait definitions
/// - Impl blocks
/// - Constants and statics
/// - Type aliases
/// - Modules
/// - Macro definitions
pub const RUST_SYMBOLS_QUERY: &str = r#"
; Function definitions
(function_item
  (visibility_modifier)? @function.visibility
  name: (identifier) @function.name) @function.def

; Struct definitions
(struct_item
  (visibility_modifier)? @struct.visibility
  name: (type_identifier) @struct.name) @struct.def

; Enum definitions
(enum_item
  (visibility_modifier)? @enum.visibility
  name: (type_identifier) @enum.name) @enum.def

; Trait definitions
(trait_item
  (visibility_modifier)? @trait.visibility
  name: (type_identifier) @trait.name) @trait.def

; Impl blocks - capture the type being implemented
(impl_item
  type: (type_identifier) @impl.type) @impl.def

; Impl blocks with trait
(impl_item
  trait: (type_identifier) @impl.trait
  type: (type_identifier) @impl.for_type) @impl.trait_def

; Constant definitions
(const_item
  (visibility_modifier)? @const.visibility
  name: (identifier) @const.name) @const.def

; Static definitions
(static_item
  (visibility_modifier)? @static.visibility
  name: (identifier) @static.name) @static.def

; Type alias definitions
(type_item
  (visibility_modifier)? @type.visibility
  name: (type_identifier) @type.name) @type.def

; Module definitions
(mod_item
  (visibility_modifier)? @mod.visibility
  name: (identifier) @mod.name) @mod.def

; Macro definitions (macro_rules!)
(macro_definition
  name: (identifier) @macro.name) @macro.def

; Methods inside impl blocks
(impl_item
  type: (type_identifier) @method.parent_type
  body: (declaration_list
    (function_item
      (visibility_modifier)? @method.visibility
      name: (identifier) @method.name) @method.def))

; Enum variants
(enum_item
  name: (type_identifier) @variant.parent
  body: (enum_variant_list
    (enum_variant
      name: (identifier) @variant.name) @variant.def))

; Struct fields
(struct_item
  name: (type_identifier) @field.parent
  body: (field_declaration_list
    (field_declaration
      (visibility_modifier)? @field.visibility
      name: (field_identifier) @field.name) @field.def))
"#;
