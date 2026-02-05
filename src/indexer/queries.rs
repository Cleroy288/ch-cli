//! Tree-sitter query definitions for different languages.
//!
//! This module contains the S-expression queries used to extract
//! symbols from parsed ASTs.

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

/// Query for extracting doc comments (separate query for efficiency)
pub const RUST_DOC_COMMENTS_QUERY: &str = r#"
; Line doc comments
(line_comment) @doc.line

; Block doc comments
(block_comment) @doc.block
"#;

/// Tree-sitter query for extracting Rust references (symbol usages).
///
/// This query captures:
/// - Function/method calls
/// - Type references
/// - Identifiers (variable usages)
/// - Field accesses
pub const RUST_REFERENCES_QUERY: &str = r#"
; Function calls - capture the function name being called
(call_expression
  function: (identifier) @call.name)

; Method calls - capture the method name
(call_expression
  function: (field_expression
    field: (field_identifier) @method_call.name))

; Type references in annotations (e.g., let x: SomeType)
(type_identifier) @type_ref.name

; Field access expressions (e.g., obj.field)
(field_expression
  field: (field_identifier) @field_access.name)

; Identifiers in expressions (variable references)
(identifier) @ident.name

; Scoped identifiers (e.g., module::function)
(scoped_identifier
  name: (identifier) @scoped.name)
"#;

