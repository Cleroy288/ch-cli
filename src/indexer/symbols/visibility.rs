//! Visibility modifiers for code symbols.

use std::fmt;

/// Visibility of a symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Visibility {
	/// Public (pub)
	Public,
	/// Public within crate (pub(crate))
	PublicCrate,
	/// Public within super module (pub(super))
	PublicSuper,
	/// Private (default)
	#[default]
	Private,
}

impl fmt::Display for Visibility {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		let label = match self {
			Visibility::Public => "pub",
			Visibility::PublicCrate => "pub(crate)",
			Visibility::PublicSuper => "pub(super)",
			Visibility::Private => "",
		};
		write!(fmt, "{}", label)
	}
}
