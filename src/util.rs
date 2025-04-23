use crate::agent::{AgentType, MovementProfile, MovementEffect};
use crate::terrain::types::TerrainType;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub fn load_agent_types(path: &str) -> Vec<AgentType> {
    let mut file = File::open(path).expect("Failed to open config/agent_types.yaml");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config/agent_types.yaml");
    let raw_types: Vec<serde_yaml::Value> = serde_yaml::from_str(&contents).expect("Failed to parse config/agent_types.yaml");
    raw_types.into_iter().map(|raw| {
        let r#type = raw["type"].as_str().unwrap_or("").to_string();
        let color = raw["color"].as_str().unwrap_or("white").to_string();
        let move_speed = raw["move_speed"].as_f64().unwrap_or(1.0) as f32;
        let strength = raw["strength"].as_i64().unwrap_or(1) as i32;
        let stamina = raw["stamina"].as_i64().unwrap_or(1) as i32;
        let vision = raw["vision"].as_i64().unwrap_or(1) as i32;
        let work_rate = raw["work_rate"].as_i64().unwrap_or(1) as i32;
        let icon = raw["icon"].as_str().unwrap_or("?").to_string();
        let damping = raw.get("damping").and_then(|v| v.as_f64()).map(|v| v as f32);
        let move_probability = raw.get("move_probability").and_then(|v| v.as_f64()).map(|v| v as f32);
        let name = raw.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
        // Parse movement_profile
        let mut terrain_effects = HashMap::new();
        if let Some(mprof) = raw.get("movement_profile") {
            if let Some(map) = mprof.as_mapping() {
                for (k, v) in map {
                    if let Some(terrain_str) = k.as_str() {
                        if let Ok(terrain) = serde_yaml::from_str::<TerrainType>(terrain_str) {
                            let effect = if let Some(effect_str) = v.as_str() {
                                match effect_str.to_lowercase().as_str() {
                                    "normal" => MovementEffect::Normal,
                                    "impassable" => MovementEffect::Impassable,
                                    _ if effect_str.to_lowercase().starts_with("slow(") && effect_str.ends_with(")") => {
                                        let inner = &effect_str[5..effect_str.len()-1];
                                        if let Ok(mult) = inner.parse::<f32>() {
                                            MovementEffect::Slow(mult)
                                        } else { MovementEffect::Normal }
                                    }
                                    _ => MovementEffect::Normal
                                }
                            } else { MovementEffect::Normal };
                            terrain_effects.insert(terrain, effect);
                        }
                    }
                }
            }
        }
        let movement_profile = MovementProfile { terrain_effects };
        AgentType {
            r#type,
            color,
            move_speed,
            strength,
            stamina,
            vision,
            work_rate,
            icon,
            damping,
            move_probability,
            movement_profile,
            name,
            decision_engine: None,
        }
    }).collect()
}
