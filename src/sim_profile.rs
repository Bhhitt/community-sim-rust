use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SimProfile {
    pub name: String,
    pub map_width: Option<i32>,
    pub map_height: Option<i32>,
    pub map_size: Option<i32>,
    pub num_agents: usize,
    pub ticks: usize,
    pub benchmark: Option<bool>,
    pub quiet: Option<bool>,
    pub spawn_config: Option<String>, // Optional path to spawn config file
}

pub fn load_profiles_from_yaml(path: &str) -> Vec<SimProfile> {
    let yaml = std::fs::read_to_string(path).expect("Failed to read sim_profiles.yaml");
    serde_yaml::from_str(&yaml).expect("Failed to parse sim_profiles.yaml")
}

pub fn find_profile<'a>(profiles: &'a [SimProfile], name: &str) -> Option<&'a SimProfile> {
    profiles.iter().find(|p| p.name == name)
}
