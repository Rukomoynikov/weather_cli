use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Write;
use std::fs;
use directories::ProjectDirs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: Option<String>,
    pub default_town: Option<String>
}

pub fn update_config_value(args: &[String]) -> Result<(), Box<dyn Error>> {
    let config_to_update = match args.get(1) {
        None => {
            Err("Please provide which value you want to update")?
        }
        Some(value) => match value.as_str() {
            "api_key" => value,
            "default_town" => value,
            _ => {
                Err("Wrong command. Only api_key and default_town are allowed".to_string())?
            }
        },
    };

    let Some(value_to_set) = args.get(2) else {
        return Err(format!("Please provide value for {config_to_update}"))?
    };

    create_default_config()?;

    let Ok(mut config) = read_config() else {
        return Err("Couldn't read config file")?
    };

    match config_to_update.as_str() {
        "api_key" => { config.api_key = Some(value_to_set.clone()); }
        "default_town" => { config.default_town = Some(value_to_set.clone()); }
        &_ => {}
    };

    let congig_stringified = toml::to_string(&config).unwrap();

    let project_dirs = match ProjectDirs::from("com", "rukomoynikov", "weather_cli") {
        None => {
            return Err("Couldn't find directory for config".to_string())?
        }
        Some(dir) => { dir }
    };

    let config_dir = project_dirs.config_dir();

    let config_file_path = config_dir.join("config.toml");

    fs::write(&config_file_path, &congig_stringified).unwrap();

    Ok(())
}

fn create_default_config() -> Result<(), String> {
    let project_dirs = match ProjectDirs::from("com", "rukomoynikov", "weather_cli") {
        None => {
            return Err("Couldn't find directory for config".to_string())
        }
        Some(dir) => { dir }
    };

    let config_dir = project_dirs.config_dir();

    let config_file_path = config_dir.join("config.toml");

    if config_file_path.exists() {
        return Ok(())
    }

    if !config_dir.exists() {
        fs::create_dir(&config_dir);
    }

    let default_config = Config {
        api_key: Some("".to_string()),
        default_town: Some("".to_string()),
    };

    let default_config = toml::to_string(&default_config).unwrap();

    fs::write(&config_file_path, &default_config).unwrap();

    Ok(())
}

pub fn read_config() -> Result<Config, String> {
    let project_dirs = match ProjectDirs::from("com", "rukomoynikov", "weather_cli") {
        None => {
            return Err("Couldn't find directory for config".to_string())
        }
        Some(dir) => { dir }
    };

    let config_dir = project_dirs.config_dir();

    let config_file_path = config_dir.join("config.toml");

    let default_config = Config {
        api_key: Some("".to_string()),
        default_town: Some("".to_string()),
    };

    let default_config = toml::to_string(&default_config).unwrap();

    let config_file = fs::read_to_string(&config_file_path).unwrap_or(default_config);

    match toml::from_str::<Config>(&config_file) {
        Ok(config) => { Ok(config) }
        Err(_) => {
            return Err("Couldn't parse config file".to_string())
        }
    }
}