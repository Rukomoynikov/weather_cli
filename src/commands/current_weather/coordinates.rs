use crate::api_client::{APIClient, Get};
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

pub async fn get_coords_from_city_name(city_name: &String) -> Option<Place> {
    let api_client = APIClient::new();

    let places = match api_client
        .get::<Vec<Place>>(format!(
            "https://api.openweathermap.org/geo/1.0/direct?q={city_name}&limit=1"
        ))
        .await
    {
        Ok(places) => places,
        Err(err) => {
            println!("{}", err);
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
