use serde::Deserialize;
use toml;

static CONFIG_FILE_NAME: &str = "karuta.toml";

#[derive(Debug, Clone, Deserialize)]
pub struct GlobalConfig {
    pub sources: Vec<String>,
}

pub fn load_global_config() -> GlobalConfig {
    let config_str = std::fs::read_to_string(CONFIG_FILE_NAME).expect("Failed to read config file");
    toml::de::from_str(&config_str).expect("Failed to parse config file")
}
