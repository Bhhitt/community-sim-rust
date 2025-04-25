use crate::agent::{AgentType, MovementProfile, MovementEffect, DecisionEngineConfig};
use crate::agent::mlp::MLPConfig;
use std::fs::File;
use std::io::Read;

pub fn load_agent_types(path: &str) -> Vec<AgentType> {
    let mut file = File::open(path).expect("Failed to open config/agent_types.yaml");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config/agent_types.yaml");
    let raw_types: Vec<serde_yaml::Value> = serde_yaml::from_str(&contents).expect("Failed to parse config/agent_types.yaml");
    raw_types.into_iter().map(|raw| {
        let name = raw["type"].as_str().unwrap_or("").to_string();
        let color = if let Some(seq) = raw["color"].as_sequence() {
            if seq.len() == 3 {
                (
                    seq[0].as_u64().unwrap_or(255) as u8,
                    seq[1].as_u64().unwrap_or(255) as u8,
                    seq[2].as_u64().unwrap_or(255) as u8,
                )
            } else {
                (255, 255, 255)
            }
        } else if let Some(hex_str) = raw["color"].as_str() {
            // Parse hex string like "#FF7043"
            if let Some(hex) = hex_str.strip_prefix('#') {
                if hex.len() == 6 {
                    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
                    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
                    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
                    (r, g, b)
                } else {
                    (255, 255, 255)
                }
            } else {
                (255, 255, 255)
            }
        } else {
            (255, 255, 255)
        };
        let speed = raw["move_speed"].as_f64().unwrap_or(1.0) as f32;
        let effect = match raw["move_effect"].as_str() {
            Some("blocked") => MovementEffect::Blocked,
            Some("slowed") => MovementEffect::Slowed(raw["move_cost"].as_f64().unwrap_or(1.0) as f32),
            _ => MovementEffect::None,
        };
        let movement_profile = MovementProfile { speed, effect };
        // --- DecisionEngineConfig: parse from YAML ---
        let decision_engine = if let Some(decision_engine_val) = raw.get("decision_engine") {
            if decision_engine_val.is_null() {
                DecisionEngineConfig::Simple
            } else if let Some(s) = decision_engine_val.as_str() {
                match s {
                    "Simple" => DecisionEngineConfig::Simple,
                    _ => DecisionEngineConfig::Simple,
                }
            } else if decision_engine_val.is_mapping() {
                // Try to parse as MLPConfig (tagged or tagless)
                serde_yaml::from_value::<MLPConfig>(decision_engine_val.clone())
                    .map(DecisionEngineConfig::MLP)
                    .unwrap_or(DecisionEngineConfig::Simple)
            } else {
                DecisionEngineConfig::Simple
            }
        } else {
            DecisionEngineConfig::Simple
        };
        // --- Parse hunger_rate from YAML ---
        let hunger_rate = raw["hunger_rate"].as_f64().unwrap_or(0.01) as f32;
        let hunger_threshold = raw["hunger_threshold"].as_f64().unwrap_or(50.0) as f32;
        AgentType {
            name,
            color,
            movement_profile,
            decision_engine,
            hunger_rate,
            hunger_threshold,
        }
    }).collect()
}
