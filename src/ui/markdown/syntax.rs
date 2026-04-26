use std::sync::LazyLock;

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

static SYNTAX_SET: LazyLock<SyntaxSet> =
	LazyLock::new(SyntaxSet::load_defaults_newlines);

static THEME_SET: LazyLock<ThemeSet> =
	LazyLock::new(ThemeSet::load_defaults);

const THEME_NAME: &str = "base16-eighties.dark";

/// Falls back to plain text if language is unknown.
pub fn highlight_lines(
	lang: &str,
	code: &str,
) -> Vec<Line<'static>> {
	let syntax = find_syntax(lang);
	let theme = &THEME_SET.themes[THEME_NAME];
	let mut hlt =
		HighlightLines::new(syntax, theme);
	LinesWithEndings::from(code)
		.map(|line| {
			Line::from(
				highlight_one_line(&mut hlt, line),
			)
		})
		.collect()
}

/// name → token → alias → extension → plain text
fn find_syntax(
	lang: &str,
) -> &'static syntect::parsing::SyntaxReference {
	let set = &*SYNTAX_SET;
	if lang.is_empty() {
		return set.find_syntax_plain_text();
	}
	let lower = lang.to_ascii_lowercase();
	resolve_name(&lower, set)
		.or_else(|| set.find_syntax_by_token(&lower))
		.or_else(|| {
			let alias = resolve_alias(&lower)?;
			set.find_syntax_by_token(alias)
		})
		.or_else(|| {
			set.find_syntax_by_extension(&lower)
		})
		.unwrap_or_else(|| {
			set.find_syntax_plain_text()
		})
}

/// Resolves fence tags to source-only syntaxes.
///
/// Some syntect syntaxes (e.g. PHP) default to an
/// HTML-embedded variant that needs `<?php` tags.
/// Code fences contain pure source — map to the
/// source-only syntax by name.
fn resolve_name<'a>(
	lang: &str,
	set: &'a SyntaxSet,
) -> Option<&'a syntect::parsing::SyntaxReference> {
	let name = match lang {
		"php" => "PHP Source",
		_ => return None,
	};
	set.find_syntax_by_name(name)
}

/// Maps lowercased tokens to syntect-known tokens.
/// `lang` must already be ASCII-lowercased.
fn resolve_alias(
	lang: &str,
) -> Option<&'static str> {
	match lang {
		"typescript" | "ts" | "tsx" | "jsx" => {
			Some("js")
		}
		"bash" | "shell" | "zsh" => Some("sh"),
		"python" => Some("py"),
		"golang" => Some("go"),
		"yml" => Some("yaml"),
		"jsonc" => Some("json"),
		"c++" => Some("cpp"),
		"csharp" | "c#" => Some("cs"),
		_ => None,
	}
}

fn highlight_one_line(
	highlighter: &mut HighlightLines<'_>,
	line: &str,
) -> Vec<Span<'static>> {
	let set = &*SYNTAX_SET;
	let Ok(ranges) =
		highlighter.highlight_line(line, set)
	else {
		return vec![Span::from(line.to_owned())];
	};

	ranges
		.into_iter()
		.map(|(style, text)| {
			let c = style.foreground;
			let clean =
				text.trim_end_matches(['\r', '\n']);
			Span::styled(
				clean.to_owned(),
				Style::new().fg(Color::Rgb(
					c.r, c.g, c.b,
				)),
			)
		})
		.collect()
}
