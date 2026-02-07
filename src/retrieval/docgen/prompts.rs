//! Prompt templates for documentation generation.
//!
//! Contains specialized prompts for different symbol types
//! to generate high-quality documentation.

use crate::indexer::SymbolKind;

/// Prompt template for function documentation.
pub const DOC_PROMPT_FUNCTION: &str =
	r#"You are a code documentation expert.
Generate a clear, concise description for this Rust function.

Function name: {name}
Signature: {signature}
Code:
```rust
{code_snippet}
```
{user_comment_section}
Write a 2-4 sentence description explaining:
1. What this function does (its purpose)
2. Key parameters and their roles
3. What it returns
4. Any important side effects or error conditions

Be specific and technical. Use present tense.
Do not repeat the signature.

Description:"#;

/// Prompt template for struct documentation.
pub const DOC_PROMPT_STRUCT: &str =
	r#"You are a code documentation expert.
Generate a clear, concise description for this Rust struct.

Struct name: {name}
Definition:
```rust
{code_snippet}
```
{user_comment_section}
Write a 2-4 sentence description explaining:
1. What this struct represents (its domain concept)
2. Its main fields and their purpose
3. How it's typically used in the codebase
4. Any invariants or constraints

Be specific and technical. Use present tense.

Description:"#;

/// Prompt template for enum documentation.
pub const DOC_PROMPT_ENUM: &str =
	r#"You are a code documentation expert.
Generate a clear, concise description for this Rust enum.

Enum name: {name}
Definition:
```rust
{code_snippet}
```
{user_comment_section}
Write a 2-4 sentence description explaining:
1. What states or variants this enum represents
2. When each variant is used
3. The overall purpose of this enum in the codebase

Be specific and technical. Use present tense.

Description:"#;

/// Prompt template for trait documentation.
pub const DOC_PROMPT_TRAIT: &str =
	r#"You are a code documentation expert.
Generate a clear, concise description for this Rust trait.

Trait name: {name}
Definition:
```rust
{code_snippet}
```
{user_comment_section}
Write a 2-4 sentence description explaining:
1. What capability or behavior this trait defines
2. Its required methods and their purpose
3. What types typically implement this trait
4. Any important constraints or guarantees

Be specific and technical. Use present tense.

Description:"#;

/// Prompt template for impl block documentation.
pub const DOC_PROMPT_IMPL: &str =
	r#"You are a code documentation expert.
Generate a clear, concise description for this Rust impl block.

Impl: {name}
Code:
```rust
{code_snippet}
```
{user_comment_section}
Write a 2-3 sentence description explaining:
1. What functionality this impl block provides
2. Key methods and their purpose
3. Any trait being implemented (if applicable)

Be specific and technical. Use present tense.

Description:"#;

/// Prompt template for method documentation.
pub const DOC_PROMPT_METHOD: &str =
	r#"You are a code documentation expert.
Generate a clear, concise description for this Rust method.

Method name: {name}
Signature: {signature}
Parent type: {parent}
Code:
```rust
{code_snippet}
```
{user_comment_section}
Write a 2-4 sentence description explaining:
1. What this method does for its parent type
2. Key parameters and their roles
3. What it returns
4. Any mutations to self or side effects

Be specific and technical. Use present tense.

Description:"#;

/// Prompt template for constant documentation.
pub const DOC_PROMPT_CONST: &str =
	r#"You are a code documentation expert.
Generate a clear, concise description for this Rust constant.

Constant: {name}
Definition:
```rust
{code_snippet}
```
{user_comment_section}
Write a 1-2 sentence description explaining:
1. What this constant represents
2. Where and why it's used

Be specific and technical.

Description:"#;

/// Prompt template for module documentation.
pub const DOC_PROMPT_MODULE: &str =
	r#"You are a code documentation expert.
Generate a clear, concise description for this Rust module.

Module name: {name}
Contents preview:
```rust
{code_snippet}
```
{user_comment_section}
Write a 2-3 sentence description explaining:
1. What functionality this module provides
2. Its main types and functions
3. How it fits into the overall architecture

Be specific and technical. Use present tense.

Description:"#;

/// Prompt template for type alias documentation.
pub const DOC_PROMPT_TYPE_ALIAS: &str =
	r#"You are a code documentation expert.
Generate a clear, concise description for this Rust type alias.

Type alias: {name}
Definition:
```rust
{code_snippet}
```
{user_comment_section}
Write a 1-2 sentence description explaining:
1. What this type alias represents
2. Why this alias exists (clarity, brevity, abstraction)

Be specific and technical.

Description:"#;

/// Prompt template for macro documentation.
pub const DOC_PROMPT_MACRO: &str =
	r#"You are a code documentation expert.
Generate a clear, concise description for this Rust macro.

Macro name: {name}
Definition:
```rust
{code_snippet}
```
{user_comment_section}
Write a 2-3 sentence description explaining:
1. What code this macro generates
2. Its input parameters/patterns
3. When and why to use this macro

Be specific and technical.

Description:"#;

/// Generic fallback prompt for any symbol.
pub const DOC_PROMPT_GENERIC: &str =
	r#"You are a code documentation expert.
Generate a clear, concise description for this Rust code element.

Name: {name}
Kind: {kind}
Code:
```rust
{code_snippet}
```
{user_comment_section}
Write a 2-3 sentence description explaining what this code element
does and its purpose.

Be specific and technical. Use present tense.

Description:"#;

/// Get the appropriate prompt template for a symbol kind.
pub fn get_prompt_for_kind(kind: SymbolKind) -> &'static str {
	match kind {
		SymbolKind::Function => DOC_PROMPT_FUNCTION,
		SymbolKind::Method => DOC_PROMPT_METHOD,
		SymbolKind::Struct => DOC_PROMPT_STRUCT,
		SymbolKind::Enum => DOC_PROMPT_ENUM,
		SymbolKind::Trait => DOC_PROMPT_TRAIT,
		SymbolKind::Impl => DOC_PROMPT_IMPL,
		SymbolKind::Constant | SymbolKind::Static => {
			DOC_PROMPT_CONST
		}
		SymbolKind::Module => DOC_PROMPT_MODULE,
		SymbolKind::TypeAlias => DOC_PROMPT_TYPE_ALIAS,
		SymbolKind::Macro => DOC_PROMPT_MACRO,
		_ => DOC_PROMPT_GENERIC,
	}
}

/// Format the user comment section for inclusion in prompt.
pub fn format_user_comment_section(
	user_comment: Option<&str>,
) -> String {
	match user_comment {
		Some(comment) if !comment.trim().is_empty() => {
			format!(
				"\nExisting documentation:\n```\n{}\n```\n\
				Enhance and expand upon this existing \
				documentation.\n",
				comment.trim()
			)
		}
		_ => String::new(),
	}
}

/// Build a complete prompt for a doc entry.
pub fn build_prompt(
	kind: SymbolKind,
	name: &str,
	signature: Option<&str>,
	code_snippet: &str,
	user_comment: Option<&str>,
	parent: Option<&str>,
) -> String {
	let template = get_prompt_for_kind(kind);
	let user_comment_section =
		format_user_comment_section(user_comment);

	template
		.replace("{name}", name)
		.replace("{kind}", &kind.to_string())
		.replace("{signature}", signature.unwrap_or("N/A"))
		.replace("{code_snippet}", code_snippet)
		.replace("{user_comment_section}", &user_comment_section)
		.replace("{parent}", parent.unwrap_or("N/A"))
}

