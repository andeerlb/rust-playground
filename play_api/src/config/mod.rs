pub mod settings;

use std::fs;
use settings::Config;

pub fn load_config() -> Config {
    let contents = fs::read_to_string("config.yaml")
        .expect("Error reading config.yaml");

    serde_yaml::from_str(&contents)
        .expect("Error parsing YAML")
}