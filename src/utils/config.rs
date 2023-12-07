use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub api_key: Option<String>,
    pub default_town: Option<String>,
}

impl Config {
    fn default() -> Self {
        Config {
            api_key: Some("".to_string()),
            default_town: Some("".to_string()),
        }
    }
}

pub fn read_config_value(value: &str) -> Option<String> {
    let config = read_config();

    match value {
        "api_key" => config.api_key,
        "default_town" => config.default_town,
        &_ => None,
    }
}

pub fn read_config() -> Config {
    let default_config = Config::default();

    let project_dirs = match ProjectDirs::from("com", "rukomoynikov", "weather_cli") {
        None => {
            println!("Couldn't find directory for config");
            return default_config.clone();
        }
        Some(dir) => dir,
    };

    let config_dir = project_dirs.config_dir();

    let config_file_path = config_dir.join("config.toml");

    let default_config_stringified = toml::to_string(&default_config).unwrap();

    let config_file = fs::read_to_string(config_file_path).unwrap_or(default_config_stringified);

    match toml::from_str::<Config>(&config_file) {
        Ok(config) => config,
        Err(_) => {
            println!("Couldn't parse config file");
            default_config
        }
    }
}

pub fn get_config_dir() -> Result<PathBuf, String> {
    let project_dirs = match ProjectDirs::from("com", "rukomoynikov", "weather_cli") {
        None => return Err("Couldn't find directory for config".to_string()),
        Some(dir) => dir,
    };

    Ok(project_dirs.config_dir().to_path_buf())
}
