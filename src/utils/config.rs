use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: Option<String>,
    pub default_town: Option<String>,
}

pub fn read_config_value(value: &str) -> Option<String> {
    let Ok(config) = read_config() else {
        return None;
    };

    match value {
        "api_key" => config.api_key,
        "default_town" => config.default_town,
        &_ => None,
    }
}

pub fn read_config() -> Result<Config, String> {
    let project_dirs = match ProjectDirs::from("com", "rukomoynikov", "weather_cli") {
        None => return Err("Couldn't find directory for config".to_string()),
        Some(dir) => dir,
    };

    let config_dir = project_dirs.config_dir();

    let config_file_path = config_dir.join("config.toml");

    let default_config = Config {
        api_key: Some("".to_string()),
        default_town: Some("".to_string()),
    };

    let default_config = toml::to_string(&default_config).unwrap();

    let config_file = fs::read_to_string(config_file_path).unwrap_or(default_config);

    match toml::from_str::<Config>(&config_file) {
        Ok(config) => Ok(config),
        Err(_) => Err("Couldn't parse config file".to_string()),
    }
}

pub fn get_config_dir() -> Result<PathBuf, String> {
    let project_dirs = match ProjectDirs::from("com", "rukomoynikov", "weather_cli") {
        None => return Err("Couldn't find directory for config".to_string()),
        Some(dir) => dir,
    };

    Ok(project_dirs.config_dir().to_path_buf())
}
