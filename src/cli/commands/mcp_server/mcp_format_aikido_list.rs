use crate::domain::aikido::{
	ContainerRepo, Repo,
};

/// Format repo list
pub fn format_repos(repos: &[Repo]) -> String {
	if repos.is_empty() {
		return "(no repositories)".into();
	}
	repos
		.iter()
		.map(|r| {
			format!(
				"{} ({}) [id:{}]",
				r.name, r.provider, r.id,
			)
		})
		.collect::<Vec<_>>()
		.join("\n")
}

/// Format container list
pub fn format_containers(
	items: &[ContainerRepo],
) -> String {
	if items.is_empty() {
		return "(no containers)".into();
	}
	items
		.iter()
		.map(format_container_line)
		.collect::<Vec<_>>()
		.join("\n")
}

fn format_container_line(
	c: &ContainerRepo,
) -> String {
	let status = if c.active {
		"active"
	} else {
		"inactive"
	};
	format!(
		"{} ({}) [{}, {} images]",
		c.name, c.registry, status,
		c.image_count,
	)
}
