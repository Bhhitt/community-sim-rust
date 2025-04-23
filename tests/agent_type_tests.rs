use community_sim::agent::{AgentType};

#[test]
fn test_agent_type_deserialize() {
    let yaml = r#"
- type: "worker"
  color: "blue"
  move_speed: 2.0
  stamina: 10
  vision: 5
  work_rate: 1
  strength: 5
  icon: "W"
  movement_profile: { terrain_effects: {} }
  damping: ~
  move_probability: ~
  name: ~
- type: "scout"
  color: "green"
  move_speed: 4.0
  stamina: 10
  vision: 5
  work_rate: 1
  strength: 2
  icon: "S"
  movement_profile: { terrain_effects: {} }
  damping: ~
  move_probability: ~
  name: ~
"#;
    let agent_types: Vec<AgentType> = serde_yaml::from_str(yaml).expect("YAML parse failed");
    assert_eq!(agent_types.len(), 2);
    assert_eq!(agent_types[0].r#type, "worker");
    assert_eq!(agent_types[1].icon, "S");
    assert_eq!(agent_types[0].color, "blue");
    assert_eq!(agent_types[1].move_speed, 4.0);
}
