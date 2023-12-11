use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub api_key: Option<String>,
    pub default_town: Option<String>,
}