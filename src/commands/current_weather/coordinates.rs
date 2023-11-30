use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::utils::config::read_config_value;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub name: String,
    pub lat: f32,
    pub lon: f32,
    pub country: String,
    pub state: String,
}

pub async fn get_coords_from_city_name(city_name: &String) -> Option<Place> {
    let limit = 1;
    let api_key = read_config_value("api_key")?;

    let url = format!(
        "https://api.openweathermap.org/geo/1.0/direct?q={city_name}&limit={limit}&appid={api_key}"
    );

    let response = Client::new().get(&url).send().await;

    let response = match response {
        Ok(response) => response,
        Err(err) => {
            println!("{}", err);
            return None;
        }
    };

    if response.status() == StatusCode::UNAUTHORIZED {
        println!("Wrong API key");
        return None;
    }

    if response.status() != StatusCode::OK {
        return None;
    }

    let places = match response.json::<Vec<Place>>().await {
        Ok(places) => places,
        Err(_) => {
            println!("Something is not OK with geo coding request");
            return None;
        }
    };

    if places.is_empty() {
        println!("No places found under such term");
        return None;
    };

    places.get(0).cloned()
}
