use community_sim::agent::AgentType;

#[test]
fn test_yaml_file_load() {
    let yaml = r#"
- name: "explorer"
  color: [255, 255, 0]
  movement_profile: { speed: 3.0, effect: None }
  decision_engine: Simple
  x: 1.0
  y: 2.0
"#;
    let agent_types: Vec<AgentType> = serde_yaml::from_str(yaml).expect("YAML parse failed");
    assert_eq!(agent_types[0].name.as_str(), "explorer");
    assert_eq!(agent_types[0].color, (255, 255, 0));
    assert_eq!(agent_types[0].movement_profile.speed, 3.0);
}

#[test]
fn test_yaml_invalid_missing_fields() {
    let yaml = r#"
- name: "broken"
  color: [128, 128, 128]
  movement_profile: { speed: 1.0, effect: None }
  # decision_engine missing, but required
"#;
    let result: Result<Vec<AgentType>, _> = serde_yaml::from_str(yaml);
    assert!(result.is_err(), "Should fail if required fields are missing");
}
