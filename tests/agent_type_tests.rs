use community_sim::agent::{AgentType, components::DecisionEngineConfig};

#[test]
fn test_agent_type_deserialize() {
    let yaml = r#"
- name: "worker"
  color: [0, 0, 255]
  movement_profile: { speed: 2.0, effect: None }
  decision_engine: Simple
- name: "mlp_agent"
  color: [255, 0, 0]
  movement_profile: { speed: 2.0, effect: None }
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
- name: "scout"
  color: [0, 255, 0]
  movement_profile: { speed: 4.0, effect: None }
  decision_engine: Simple
"#;
    let agent_types: Vec<AgentType> = serde_yaml::from_str(yaml).expect("YAML parse failed");
    assert_eq!(agent_types.len(), 3);
    assert_eq!(agent_types[0].name.as_str(), "worker");
    assert_eq!(agent_types[1].color, (255, 0, 0));
    assert_eq!(agent_types[0].color, (0, 0, 255));
    assert_eq!(agent_types[1].movement_profile.speed, 2.0);
    match &agent_types[1].decision_engine {
        DecisionEngineConfig::MLP(_mlp) => {
            // Additional checks can be added here
        }
        _ => panic!("Expected MLP decision engine for mlp_agent"),
    }
}
