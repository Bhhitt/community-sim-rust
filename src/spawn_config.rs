//! Loader and types for data-driven spawn configuration (YAML)
use serde::Deserialize;
use std::collections::VecDeque;

#[derive(Debug, Deserialize, Clone)]
pub struct SpawnConfig {
    pub map: Option<MapConfig>,
    pub agents: Option<Vec<AgentSpawnEntry>>,
    pub food: Option<Vec<FoodSpawnEntry>>,
    pub items: Option<Vec<ItemSpawnEntry>>,
    pub money: Option<Vec<MoneySpawnEntry>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MapConfig {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AgentSpawnEntry {
    pub r#type: String,
    pub pos: Position,
    pub count: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FoodSpawnEntry {
    pub pos: Position,
    pub count: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ItemSpawnEntry {
    pub r#type: String,
    pub pos: Position,
    pub count: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MoneySpawnEntry {
    pub pos: Position,
    pub amount: u32,
    pub count: Option<usize>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Loads a SpawnConfig from a YAML file at the given path. Accepts both &str and Path types.
pub fn load_spawn_config<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<SpawnConfig> {
    SpawnConfig::load_spawn_config(path)
}

impl SpawnConfig {
    pub fn from_yaml_file(path: &str) -> anyhow::Result<Self> {
        let yaml = std::fs::read_to_string(path)?;
        let config = serde_yaml::from_str(&yaml)?;
        Ok(config)
    }

    pub fn load_spawn_config<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let yaml = std::fs::read_to_string(&path)?;
        let config = serde_yaml::from_str(&yaml)?;
        Ok(config)
    }
}
