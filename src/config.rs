use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct StatsWindowConfig {
    pub components: Option<Vec<String>>,
}

impl StatsWindowConfig {
    pub fn load_from_yaml<P: AsRef<Path>>(path: P) -> Self {
        let contents = fs::read_to_string(path);
        if let Ok(contents) = contents {
            serde_yaml::from_str(&contents).unwrap_or_else(|_| Self { components: None })
        } else {
            Self { components: None }
        }
    }
}
