use rustean::domain::agent_spec::AgentSpec;
use rustean::domain::preflight::PreFlightData;

#[test]
fn new_preflight_stores_main_prompt() {
	// Arrange
	let data = PreFlightData {
		main_prompt: "Explain this".to_string(),
		agents: vec![],
		mode_label: "Default".to_string(),
		file_count: 0,
		symbol_count: 0,
	};

	// Assert
	assert_eq!(data.main_prompt, "Explain this");
	assert_eq!(data.mode_label, "Default");
}

#[test]
fn preflight_with_agents_and_counts() {
	// Arrange
	let agents = vec![AgentSpec {
		model: "sonnet".to_string(),
		description: "Review security".to_string(),
	}];

	// Act
	let data = PreFlightData {
		main_prompt: "Fix the API".to_string(),
		agents,
		mode_label: "Action".to_string(),
		file_count: 3,
		symbol_count: 2,
	};

	// Assert
	assert_eq!(data.agents.len(), 1);
	assert_eq!(data.file_count, 3);
	assert_eq!(data.symbol_count, 2);
}

#[test]
fn preflight_clone_is_independent() {
	// Arrange
	let data = PreFlightData {
		main_prompt: "hello".to_string(),
		agents: vec![],
		mode_label: "Plan".to_string(),
		file_count: 1,
		symbol_count: 0,
	};

	// Act
	let cloned = data.clone();

	// Assert
	assert_eq!(cloned.main_prompt, data.main_prompt);
	assert_eq!(cloned.mode_label, data.mode_label);
}
