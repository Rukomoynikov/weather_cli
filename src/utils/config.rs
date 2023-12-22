use crate::entities::config::Config;
use anyhow::Result;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub fn read_config() -> Config {
    let Ok(config_dir) = get_config_dir() else {
        return Config::default();
    };

    let config_file_path = config_dir.join("config.toml");

    let Ok(config_file) = fs::read_to_string(config_file_path) else {
        return Config::default();
    };

    toml::from_str::<Config>(&config_file).unwrap_or_default()
}

pub fn get_config_dir() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("com", "rukomoynikov", "weather_cli")
        .ok_or_else(|| anyhow::anyhow!("Couldn't find directory for config"))?;

    Ok(project_dirs.config_dir().to_path_buf())
}

pub fn update_cache_value(city_name: String, lat: f32, lon: f32) -> Result<()> {
    create_config_dir()?;

    if read_config()
        .default_town
        .unwrap_or_default()
        .to_lowercase()
        != *city_name.to_lowercase()
    {
        return Ok(());
    }

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

fn create_config_dir() -> Result<()> {
    let config_dir = get_config_dir()?;

    fs::create_dir_all(config_dir)?;

    Ok(())
}
