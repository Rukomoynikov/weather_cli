use crate::api_client::{APIClient, Get};
use crate::entities::forecast::Forecast;
use crate::entities::place::Place;
use crate::utils::config::read_config;
use std::error::Error;

pub async fn get_current_weather(args: &[String]) -> Result<(), Box<dyn Error>> {
    let config = read_config();

    let city_name = match args.get(1) {
        Some(city_name) => city_name.clone(),
        None => match config.default_town {
            None => {
                return Err(
                    "No city was provided in arguments or set as default in config"
                        .to_string()
                        .into(),
                )
            }
            Some(default_town) => default_town,
        },
    };

    let place = match get_coords_from_city_name(&city_name).await {
        None => return Ok(()),
        Some(place) => place,
    };

    let weather = get_weather((&place.lat, &place.lon)).await?;

    print_results(&weather);

    Ok(())
}

async fn get_coords_from_city_name(city_name: &String) -> Option<Place> {
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

async fn get_weather(coords: (&f32, &f32)) -> Result<Forecast, Box<dyn Error>> {
    let lat = coords.0;
    let lon = coords.1;

    let api_client = APIClient::new();

    let Ok(forecast) = api_client
        .get::<Forecast>(format!(
            "https://api.openweathermap.org/data/2.5/weather?units=metric&lat={lat}&lon={lon}"
        ))
        .await
    else {
        return Err("Couldn't get weather".into());
    };

    Ok(forecast)
}

fn print_results(weather_data: &Forecast) {
    println!("Temperature: {}", weather_data.main.temp);
    println!("Feels like:  {}", weather_data.main.feels_like);
    println!("Wind speed:  {}", weather_data.wind.speed);
}
