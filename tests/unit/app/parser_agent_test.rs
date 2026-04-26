//! Tests for parser_agent — agent line parsing.

use rustean::app::parser_agent::{
    extract_agent_suffix, is_agent_line,
    parse_agent_lines,
};
use rustean::domain::agent_spec::AgentSpec;

// -- parse_agent_lines --

#[test]
fn parse_no_agents_returns_full_text() {
    // Arrange
    let input = "Write a REST API";

    // Act
    let (main, agents) = parse_agent_lines(input);

    // Assert
    assert_eq!(main, "Write a REST API");
    assert!(agents.is_empty());
}

#[test]
fn parse_one_agent() {
    // Arrange
    let input = "Do X\n- agent(sonnet): Review";

    // Act
    let (main, agents) = parse_agent_lines(input);

    // Assert
    assert_eq!(main, "Do X");
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].model, "sonnet");
    assert_eq!(agents[0].description, "Review");
}

#[test]
fn parse_two_agents() {
    // Arrange
    let input =
        "Main\n- agent(opus): Tests\n\
         - agent(haiku): Lint";

    // Act
    let (main, agents) = parse_agent_lines(input);

    // Assert
    assert_eq!(main, "Main");
    assert_eq!(agents.len(), 2);
    assert_eq!(agents[0].model, "opus");
    assert_eq!(agents[1].model, "haiku");
}

#[test]
fn parse_default_model_when_empty() {
    // Arrange
    let input = "X\n- agent(): Do Y";

    // Act
    let (_, agents) = parse_agent_lines(input);

    // Assert
    assert_eq!(agents[0].model, "sonnet");
}

#[test]
fn parse_ignores_non_agent_dashes() {
    // Arrange
    let input = "- not an agent\n- agent(s): ok";

    // Act
    let (main, agents) = parse_agent_lines(input);

    // Assert
    assert_eq!(main, "- not an agent");
    assert_eq!(agents.len(), 1);
}

// -- is_agent_line --

#[test]
fn is_agent_true_for_valid() {
    assert!(is_agent_line(
        "- agent(sonnet): Review security"
    ));
}

#[test]
fn is_agent_false_for_plain() {
    assert!(!is_agent_line("just text"));
}

#[test]
fn is_agent_false_no_description() {
    assert!(!is_agent_line("- agent(sonnet): "));
}

// -- extract_agent_suffix --

#[test]
fn extract_no_agents() {
    let (main, block) =
        extract_agent_suffix("hello world");
    assert_eq!(main, "hello world");
    assert!(block.is_empty());
}

#[test]
fn extract_with_agents() {
    // Arrange
    let input =
        "Main text\n- agent(opus): Task";

    // Act
    let (main, block) =
        extract_agent_suffix(input);

    // Assert
    assert_eq!(main, "Main text");
    assert_eq!(block, "- agent(opus): Task");
}
