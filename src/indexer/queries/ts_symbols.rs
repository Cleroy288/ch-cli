pub const TS_SYMBOLS_QUERY: &str = r#"
; Function declarations (exported or not)
(function_declaration
  name: (identifier) @function.name) @function.def

; Arrow functions assigned to const/let/var
(lexical_declaration
  (variable_declarator
    name: (identifier) @function.name
    value: (arrow_function))) @function.def

; Class declarations (reuse struct kind)
(class_declaration
  name: (type_identifier) @struct.name) @struct.def

; Interface declarations (reuse trait kind)
(interface_declaration
  name: (type_identifier) @trait.name) @trait.def

; Type alias declarations
(type_alias_declaration
  name: (type_identifier) @type.name) @type.def

; Enum declarations
(enum_declaration
  name: (identifier) @enum.name) @enum.def

; Methods inside class bodies
(class_declaration
  name: (type_identifier) @method.parent_type
  body: (class_body
    (method_definition
      name: (property_identifier) @method.name)
        @method.def))

; Class public field definitions
(class_declaration
  name: (type_identifier) @field.parent
  body: (class_body
    (public_field_definition
      name: (property_identifier) @field.name)
        @field.def))
"#;
