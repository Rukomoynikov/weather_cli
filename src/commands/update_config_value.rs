use std::error::Error;
use std::fs;

use crate::utils::config::{get_config_dir, read_config};

pub fn update_config_value(args: &[String]) -> Result<(), Box<dyn Error>> {
    let config_to_update = match args.get(1) {
        None => Err("Please provide which value you want to update")?,
        Some(value) => match value.as_str() {
            "api_key" => value,
            "default_town" => value,
            _ => Err("Wrong command. Only api_key and default_town are allowed".to_string())?,
        },
    };

    let Some(value_to_set) = args.get(2) else {
        Err(format!("Please provide value for {config_to_update}"))?
    };

    create_config_dir()?;

    let mut config = read_config();

    match config_to_update.as_str() {
        "api_key" => {
            config.api_key = Some(value_to_set.clone());
        }
        "default_town" => {
            config.default_town = Some(value_to_set.clone());
        }
        &_ => {}
    };

    let congig_stringified = toml::to_string(&config)?;

    let config_dir = get_config_dir()?;

    let config_file_path = config_dir.join("config.toml");

    fs::write(config_file_path, congig_stringified)?;

    Ok(())
}

fn create_config_dir() -> Result<(), Box<dyn Error>> {
    let config_dir = get_config_dir()?;

    if config_dir.exists() {
        return Ok(());
    }

    if !config_dir.exists() {
        fs::create_dir(&config_dir)?;
    }

    Ok(())
}
