pub mod settings;

use std::fs;
use std::env;
use settings::Config;

pub fn load_config() -> Config {
    // Read the environment variable to determine which config file to load
    let env = env::var("ENVIRONMENT").unwrap_or_else(|_| "dev".to_string());
    let filename = format!("{}.yaml", env);
    let contents = fs::read_to_string(&filename)
        .expect(&format!("Error reading {}", filename));

    serde_yaml::from_str(&contents)
        .expect("Error parsing YAML")
}