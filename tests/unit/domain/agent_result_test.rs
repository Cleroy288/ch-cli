//! Tests for AgentSection type.

use rustean::domain::agent_result::AgentSection;

#[test]
fn construction_and_equality() {
	// Arrange
	let section = AgentSection {
		model: "sonnet".into(),
		description: "Review security".into(),
		start: 0,
		end: 100,
	};

	// Act
	let cloned = section.clone();

	// Assert
	assert_eq!(section, cloned);
	assert_eq!(section.model, "sonnet");
	assert_eq!(section.start, 0);
	assert_eq!(section.end, 100);
}

#[test]
fn inequality_different_model() {
	// Arrange
	let a = AgentSection {
		model: "sonnet".into(),
		description: "task".into(),
		start: 0,
		end: 50,
	};
	let b = AgentSection {
		model: "opus".into(),
		description: "task".into(),
		start: 0,
		end: 50,
	};

	// Assert
	assert_ne!(a, b);
}

#[test]
fn debug_format_contains_fields() {
	// Arrange
	let section = AgentSection {
		model: "haiku".into(),
		description: "Write tests".into(),
		start: 10,
		end: 200,
	};

	// Act
	let debug = format!("{:?}", section);

	// Assert
	assert!(debug.contains("haiku"));
	assert!(debug.contains("Write tests"));
}
