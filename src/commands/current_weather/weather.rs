use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::utils::config::read_config_value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    // pub rain: Rain,
    // pub clouds: Clouds,
    // pub dt: i64,
    // pub sys: Sys,
    // pub timezone: i64,
    // pub id: i64,
    // pub name: String,
    // pub cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    #[serde(rename = "temp_min")]
    pub temp_min: f64,
    #[serde(rename = "temp_max")]
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    // pub gust: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rain {
    #[serde(rename = "1h")]
    pub n1h: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clouds {
    pub all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub id: i64,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}
pub async fn get_weather(coords: (&f32, &f32)) -> Option<Forecast> {
    let api_key = read_config_value("api_key")?;
    let lat = coords.0;
    let lon = coords.1;

    let url = format!("https://api.openweathermap.org/data/2.5/weather?units=metric&lat={lat}&lon={lon}&appid={api_key}");

    let response = Client::new().get(&url).send().await;

    match response {
        Ok(response) => Some(response.json::<Forecast>().await.ok()?),
        _ => None,
    }
}
