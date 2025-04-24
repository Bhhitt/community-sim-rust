use crate::agent::{AgentType, MovementProfile, MovementEffect, DecisionEngineConfig};
use std::fs::File;
use std::io::Read;

pub fn load_agent_types(path: &str) -> Vec<AgentType> {
    let mut file = File::open(path).expect("Failed to open config/agent_types.yaml");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config/agent_types.yaml");
    let raw_types: Vec<serde_yaml::Value> = serde_yaml::from_str(&contents).expect("Failed to parse config/agent_types.yaml");
    raw_types.into_iter().map(|raw| {
        let name = raw["type"].as_str().unwrap_or("").to_string();
        let color = match raw["color"].as_sequence() {
            Some(seq) if seq.len() == 3 => (
                seq[0].as_u64().unwrap_or(255) as u8,
                seq[1].as_u64().unwrap_or(255) as u8,
                seq[2].as_u64().unwrap_or(255) as u8,
            ),
            _ => (255, 255, 255),
        };
        let speed = raw["move_speed"].as_f64().unwrap_or(1.0) as f32;
        let effect = match raw["move_effect"].as_str() {
            Some("blocked") => MovementEffect::Blocked,
            Some("slowed") => MovementEffect::Slowed(raw["move_cost"].as_f64().unwrap_or(1.0) as f32),
            _ => MovementEffect::None,
        };
        let movement_profile = MovementProfile { speed, effect };
        AgentType {
            name,
            color,
            movement_profile,
            decision_engine: DecisionEngineConfig::Simple, 
        }
    }).collect()
}
