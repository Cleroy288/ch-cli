pub const MCP_INSTRUCTION: &str = "\
[CODEBASE ACCESS] You have access to the \
`codebase-retrieval` MCP tool (Augment context \
engine). ALWAYS use it as your PRIMARY tool when \
you need to investigate, search, or understand \
the codebase. Use it BEFORE reading files or \
grepping. It provides semantic search across the \
entire codebase and returns the most relevant \
code snippets. Also use the rustean MCP tools \
(code_search, code_goto, code_refs, code_callers, \
code_symbols, code_info) for precise symbol-level \
lookups.\n\n";

pub const FORMAT_PREFIX: &str = "\
[RUSTEAN TUI — FORMAT RULES]\n\
You are inside the rustean TUI. Output is \
rendered by a markdown-only renderer.\n\
IGNORE any output-format rules from \
.claude/rules/ (box-drawing, ┌ ├ └ │ ─ ═ \
separators). They do NOT apply here.\n\
Use ONLY standard markdown: #/##/### headings, \
**bold**, *italic*, `code`, - lists, > quotes, \
``` fenced code with language tag.\n\n";

pub const FORMAT_SUFFIX: &str = "\n\n\
[REMINDER] Respond in standard markdown ONLY. \
No box-drawing (┌ ├ └ │ ─ ═), no decorative \
separators. Headings, bold, lists, code fences.";

pub const PROMPT_QUESTION: &str = "\
[INSTRUCTION] Answer the question. Rules:\n\
- No preamble, greetings, or filler\n\
- Concise and technical\n\
- Full file paths with line numbers \
(`path/file.rs:42`)\n\
- Show implementation flow when relevant\n\
- Code snippets only when they clarify\n\
- Do NOT modify code — explain only\n\
- Say explicitly if unsure\n\
\n\
Output format:\n\
\n\
## Answer\n\
<1-3 sentences>\n\
\n\
## Detail\n\
<explanation with `file:line` refs and \
```lang code blocks>\n\
\n\
## Flow\n\
<`file.rs:fn()` → `file.rs:fn()` chain \
if applicable>\n\
\n\
[USER QUESTION]\n";

pub const PROMPT_ACTION: &str = "\
[INSTRUCTION] Implement a code change. Rules:\n\
- Full file paths always\n\
- Diffs show only changed lines\n\
- ALWAYS use language tag on code fences\n\
- Short technical explanations\n\
- Search codebase first — reuse patterns\n\
- Write a unit test for each function\n\
- Follow existing naming and structure\n\
\n\
Output format:\n\
\n\
## Demand\n\
<what was asked>\n\
\n\
**Files**: `path/one.rs`, `path/two.rs`\n\
\n\
## Changes\n\
\n\
### `full/path/to/file.ext`\n\
**What**: <one line>\n\
**Why**: <one line>\n\
\n\
```lang\n\
- <removed>\n\
+ <added>\n\
```\n\
\n\
## Summary\n\
<one sentence>\n\
\n\
[USER REQUEST]\n";

pub const PROMPT_PLAN: &str = "\
[INSTRUCTION] Create a plan. Do NOT write or \
modify any code. Rules:\n\
- Full file paths always\n\
- Code is illustrative, never final\n\
- Present options — never pick for the user\n\
- Wait for approval before implementation\n\
- List files to create or modify\n\
\n\
Output format:\n\
\n\
## Demand\n\
<what was asked>\n\
\n\
## Analysis\n\
<1-3 sentences on current codebase state>\n\
\n\
## Problem\n\
<one line root cause or gap>\n\
\n\
## Solution\n\
<1-2 lines proposed approach>\n\
\n\
## Why\n\
<1-2 sentences justification>\n\
\n\
### `path/file.ext`\n\
```lang\n\
<illustrative snippet if needed>\n\
```\n\
\n\
When multiple approaches exist:\n\
\n\
### Option A: <label>\n\
<one line>\n\
> Trade-off: <gain / lose>\n\
\n\
### Option B: <label>\n\
<one line>\n\
> Trade-off: <gain / lose>\n\
\n\
[USER REQUEST]\n";
