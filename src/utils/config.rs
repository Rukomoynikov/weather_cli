use directories::ProjectDirs;
use crate::entities::config::Config;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

pub fn read_config_value(value: &str) -> String {
    let config = read_config();

    match value {
        "api_key" => config.api_key.unwrap_or("".to_string()),
        "default_town" => config.default_town.unwrap_or("".to_string()),
        &_ => "".to_string(),
    }
}

pub fn read_config() -> Config {
    let default_config = Config::default();

    let project_dirs = match ProjectDirs::from("com", "rukomoynikov", "weather_cli") {
        None => {
            eprintln!("Couldn't find directory for config");
            return default_config.clone();
        }
        Some(dir) => dir,
    };

    let config_dir = project_dirs.config_dir();

    let config_file_path = config_dir.join("config.toml");

    let Ok(config_file) = fs::read_to_string(config_file_path) else {
        return default_config;
    };

    match toml::from_str::<Config>(&config_file) {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Couldn't parse config file");
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

pub fn update_cache_value(city_name: String, lat: f32, lon: f32) -> Result<(), Box<dyn std::error::Error>> {
    create_config_dir()?;

    let mut config = read_config();

    config.cache.for_town = Some(city_name);
    config.cache.cached_result = Some((lat, lon));

    let config_stringified = toml::to_string(&config)?;

    let config_dir = get_config_dir()?;

    let config_file_path = config_dir.join("config.toml");

    fs::write(config_file_path, config_stringified)?;

    Ok(())
}

pub fn get_cached_value(city: &String) -> Option<(String, f32, f32)> {
    let config = read_config();

    if let Some(cached_city) = config.cache.for_town {
        if cached_city != *city {
            return None;
        }
    };

    if let Some(coords) = config.cache.cached_result {
        return Some((city.clone(), coords.0, coords.1));
    }

    None
}

pub fn create_config_dir() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = get_config_dir()?;

    if config_dir.exists() {
        return Ok(());
    }

    if !config_dir.exists() {
        fs::create_dir(&config_dir)?;
    }

    Ok(())
}
