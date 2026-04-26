use crate::domain::git_graph::Connector;

/// Compare before/after to assign connectors.
pub fn build_connectors(
	col: usize,
	before: &[bool],
	after: &[bool],
) -> Vec<Connector> {
	let width = before.len().max(after.len());
	(0..width)
		.map(|i| {
			connector_for(i, col, before, after)
		})
		.collect()
}

fn connector_for(
	lane: usize,
	col: usize,
	before: &[bool],
	after: &[bool],
) -> Connector {
	let was = before
		.get(lane)
		.copied()
		.unwrap_or(false);
	let now = after
		.get(lane)
		.copied()
		.unwrap_or(false);
	if lane == col {
		return Connector::Pipe;
	}
	if !was && now {
		return Connector::Fork;
	}
	if was && !now {
		return Connector::Merge;
	}
	if now {
		return Connector::Pipe;
	}
	Connector::Empty
}
