use community_sim::agent::AgentType;

#[test]
fn test_yaml_file_load() {
    let yaml = r#"
- type: "explorer"
  color: "yellow"
  move_speed: 3.0
  strength: 2
  stamina: 10
  vision: 5
  work_rate: 1
  icon: "E"
  movement_profile: { terrain_effects: {} }
  damping: ~
  move_probability: ~
  name: ~
  x: 1.0
  y: 2.0
"#;
    let agent_types: Vec<AgentType> = serde_yaml::from_str(yaml).expect("YAML parse failed");
    assert_eq!(agent_types[0].r#type, "explorer");
    assert_eq!(agent_types[0].color, "yellow");
    assert_eq!(agent_types[0].icon, "E");
}

#[test]
fn test_yaml_invalid_missing_fields() {
    let yaml = r#"
- type: "broken"
  color: "grey"
  move_speed: 1.0
  # strength missing
  icon: "B"
"#;
    let result: Result<Vec<AgentType>, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err(), "Should fail if required fields are missing");
}
