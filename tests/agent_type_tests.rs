use community_sim::agent::{AgentType, mlp::MLPConfig, components::DecisionEngineConfig};

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
  decision_engine: RuleBased
- type: "mlp_agent"
  color: "red"
  move_speed: 2.0
  stamina: 10
  vision: 5
  work_rate: 1
  strength: 5
  icon: "M"
  movement_profile: { terrain_effects: {} }
  damping: ~
  move_probability: ~
  name: ~
  decision_engine: !MLP
    input_size: 3
    hidden_sizes: [4]
    output_size: 2
    weights:
      - [[0.1, 0.2, 0.3], [0.4, 0.5, 0.6], [0.7, 0.8, 0.9], [1.0, 1.1, 1.2]]
      - [[0.2, 0.3, 0.4, 0.5], [0.6, 0.7, 0.8, 0.9]]
    biases:
      - [0.1, 0.2, 0.3, 0.4]
      - [0.5, 0.6]
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
    assert_eq!(agent_types.len(), 3);
    assert_eq!(agent_types[0].r#type, "worker");
    assert_eq!(agent_types[1].icon, "M");
    assert_eq!(agent_types[0].color, "blue");
    assert_eq!(agent_types[1].move_speed, 2.0);
    match &agent_types[1].decision_engine {
        Some(DecisionEngineConfig::MLP(mlp)) => {
            assert_eq!(mlp.input_size, 3);
            assert_eq!(mlp.hidden_sizes, vec![4]);
            assert_eq!(mlp.output_size, 2);
        },
        _ => panic!("Expected MLP decision engine"),
    }
    assert_eq!(agent_types[2].icon, "S");
    assert_eq!(agent_types[2].move_speed, 4.0);
}
