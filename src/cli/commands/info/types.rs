#[allow(clippy::struct_excessive_bools)]
/// Boolean sections for the info command
#[derive(Debug, Clone, Default)]
pub struct InfoSections {
	/// show source code
	pub code: bool,
	/// show callers
	pub callers: bool,
	/// show callees
	pub callees: bool,
	/// show references
	pub refs: bool,
}

/// Raw CLI flags for the info command
#[derive(Default)]
pub struct InfoFlags {
	/// which sections to display
	pub sections: InfoSections,
}

/// Display options for the info command
pub struct InfoDisplayOpts {
	/// which sections to display
	pub sections: InfoSections,
}

impl InfoDisplayOpts {
	pub fn from_flags(
		flags: InfoFlags,
		all: bool,
	) -> Self {
		let sec = &flags.sections;
		Self {
			sections: InfoSections {
				code: sec.code || all,
				callers: sec.callers || all,
				callees: sec.callees || all,
				refs: sec.refs || all,
			},
		}
	}
}
