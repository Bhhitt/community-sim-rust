use crate::agent::AgentType;
use std::fs::File;
use std::io::Read;

pub fn load_agent_types(path: &str) -> Vec<AgentType> {
    let mut file = File::open(path).expect("Failed to open agent_types.yaml");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read agent_types.yaml");
    serde_yaml::from_str(&contents).expect("Failed to parse agent_types.yaml")
}
