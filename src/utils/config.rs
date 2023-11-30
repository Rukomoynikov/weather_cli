use crate::commands::set_config::update_config_value::read_config;

pub fn read_config_value(value: &str) -> Option<String> {
    let Ok(config) = read_config() else {
        return None
    };



    match value {
        "api_key" => {
            config.api_key
        }
        "default_town" => {
            config.default_town
        }
        &_ => {
            None
        }
    }
}