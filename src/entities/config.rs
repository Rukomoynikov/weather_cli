use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub api_key: Option<String>,
    pub default_town: Option<String>,
    pub cache: Cache,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Cache {
    pub for_town: Option<String>,
    pub cached_result: Option<(f32, f32)>
}

impl Config {
    pub fn default() -> Self {
        Config {
            api_key: Some("".to_string()),
            default_town: Some("".to_string()),
            cache: Cache::default()
        }
    }
}