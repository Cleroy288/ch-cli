//! Tests for AgentSpec type.

use rustean::domain::agent_spec::{
    AgentSpec, DEFAULT_AGENT_MODEL,
};

#[test]
fn default_model_is_sonnet() {
    assert_eq!(DEFAULT_AGENT_MODEL, "sonnet");
}

#[test]
fn agent_spec_clone_eq() {
    // Arrange
    let spec = AgentSpec {
        model: "opus".into(),
        description: "Review code".into(),
    };

    // Act
    let cloned = spec.clone();

    // Assert
    assert_eq!(spec, cloned);
}

#[test]
fn agent_spec_debug_format() {
    // Arrange
    let spec = AgentSpec {
        model: "haiku".into(),
        description: "Write tests".into(),
    };

    // Act
    let debug = format!("{:?}", spec);

    // Assert
    assert!(debug.contains("haiku"));
    assert!(debug.contains("Write tests"));
}
