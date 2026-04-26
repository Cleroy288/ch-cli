use std::path::{Path, PathBuf};

/// Ensure default skill files exist
pub fn provision_defaults() {
	let Some(dir) = commands_dir() else {
		return;
	};
	ensure_file(
		&dir.join("clean-code.md"),
		DEFAULT_CLEAN_CODE_SKILL,
	);
}

/// ~/.claude/commands/
fn commands_dir() -> Option<PathBuf> {
	let home = dirs::home_dir()?;
	let dir = home.join(".claude").join("commands");
	std::fs::create_dir_all(&dir).ok()?;
	Some(dir)
}

/// Write content only if file is missing
fn ensure_file(path: &Path, content: &str) {
	if path.exists() {
		return;
	}
	let _ = std::fs::write(path, content);
}

/// Default clean-code skill content
const DEFAULT_CLEAN_CODE_SKILL: &str = "\
---
name: clean-code
description: Review code against clean code rules
---

You are a strict but pedagogical code reviewer. \
Your mission: take the code provided and verify \
it respects ALL clean code rules.

## Rules to enforce

### Functions
- Max 20 lines per function body
- Each function does ONE thing only
- One abstraction level per function
- Max 3 arguments (use objects for more)
- No boolean/flag parameters

### Naming
- Intention-revealing names
- Zero mental mapping (no single-letter vars \
except i/j/k in short loops)
- Searchable names (no magic numbers)
- Pronounceable names

### Comments
- Code explains itself — rewrite if comment needed
- Delete: paraphrasing comments, banners, \
commented-out code, attribution comments
- Keep: legal headers, public API docs, \
TODO(TICKET), warnings

### Formatting
- Newspaper rule: public/high-level at top, \
private/details below
- Related functions stay close together
- Lines under 100 characters

### DRY
- Same logic twice? Extract shared function
- Rule of three: third copy = mandatory refactor

### Error Handling
- Error handling is one thing (try body = 1 call)
- Exceptions over return codes
- Never return null
- Never pass null

## Output format

For each file with violations:

```
File: full/path/to/file.ext

1. [CATEGORY] violation description
   Rule: which rule
   Why: impact

Refactored code:
(show the fixed version)

Score: X/10 -> Y/10
```
";

/// Short instruction to enforce clean code
pub const CODING_NORM_INSTRUCTION: &str = "\
[CLEAN CODE] When generating or modifying code, \
you MUST first read and apply the /clean-code \
skill rules (available via slash commands). \
Write clean, tested, well-named code. \
Small functions, no magic values, DRY, \
guard clauses first.\n\n";
