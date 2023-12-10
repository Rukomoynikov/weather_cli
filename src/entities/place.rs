use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub name: String,
    pub lat: f32,
    pub lon: f32,
    pub country: String,
    pub state: String,
}
