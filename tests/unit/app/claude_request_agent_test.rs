//! Tests for claude_request_agent — prompt injection.

use rustean::app::claude_request_agent::{
    format_agent_instruction, merge_agent_sys,
};
use rustean::domain::agent_spec::AgentSpec;

#[test]
fn format_empty_returns_none() {
    assert!(format_agent_instruction(&[]).is_none());
}

#[test]
fn format_one_agent() {
    // Arrange
    let agents = vec![AgentSpec {
        model: "sonnet".into(),
        description: "Review security".into(),
    }];

    // Act
    let result = format_agent_instruction(&agents);

    // Assert
    let text = result.unwrap();
    assert!(text.contains("PARALLEL SUBAGENTS"));
    assert!(text.contains("[sonnet] Review"));
    assert!(text.contains("independently"));
}

#[test]
fn merge_both_present() {
    // Arrange
    let base = Some("Base system".to_string());
    let agents = vec![AgentSpec {
        model: "opus".into(),
        description: "Write tests".into(),
    }];

    // Act
    let result = merge_agent_sys(base, &agents);

    // Assert
    let text = result.unwrap();
    assert!(text.starts_with("Base system"));
    assert!(text.contains("PARALLEL SUBAGENTS"));
}

#[test]
fn merge_no_agents_returns_base() {
    // Arrange / Act
    let result = merge_agent_sys(
        Some("Base".into()),
        &[],
    );

    // Assert
    assert_eq!(result.unwrap(), "Base");
}

#[test]
fn merge_no_base_no_agents_returns_none() {
    assert!(merge_agent_sys(None, &[]).is_none());
}
