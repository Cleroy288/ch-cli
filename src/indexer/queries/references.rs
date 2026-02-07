//! Tree-sitter query for extracting Rust references (symbol usages).

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

; Scoped function calls - capture the function name (e.g., module::function())
(call_expression
  function: (scoped_identifier
    name: (identifier) @call.name))

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
