use crate::api_client::{APIClient, Get};
use crate::entities::forecast::Forecast;
use crate::entities::place::Place;
use crate::utils::config::{get_cached_value, read_config, update_cache_value};
use anyhow::Result;

pub async fn get_current_weather(args: &[String]) -> Result<()> {
    let config = read_config();

    let city_name = match args.get(1) {
        Some(city_name) => city_name.clone(),
        None => match config.default_town {
            None => {
                return Err(anyhow::anyhow!(
                    "No city was provided in arguments or set as default in config"
                ));
            }
            Some(default_town) => default_town,
        },
    };

    let place = match get_coords_from_city_name(&city_name).await {
        None => return Ok(()),
        Some(place) => place,
    };

    update_cache_value(place.name, place.lat, place.lon)?;

    let weather = get_weather((&place.lat, &place.lon)).await?;

    print_results(&city_name, &weather);

    Ok(())
}

async fn get_coords_from_city_name(city_name: &String) -> Option<Place> {
    if let Some((_, lat, lon)) = get_cached_value(city_name) {
        return Some(Place {
            name: city_name.clone(),
            lat,
            lon,
            country: "".to_string(),
            state: "".to_string(),
        });
    }

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

async fn get_weather(coords: (&f32, &f32)) -> Result<Forecast> {
    let lat = coords.0;
    let lon = coords.1;

    let api_client = APIClient::new();

    let Ok(forecast) = api_client
        .get::<Forecast>(format!(
            "https://api.openweathermap.org/data/2.5/weather?units=metric&lat={lat}&lon={lon}"
        ))
        .await
    else {
        return Err(anyhow::anyhow!("Couldn't get weather"));
    };

    Ok(forecast)
}

fn print_results(city_name: &String, weather_data: &Forecast) {
    println!("City:        {}", city_name);
    println!("Temperature: {}", weather_data.main.temp);
    println!("Feels like:  {}", weather_data.main.feels_like);
    println!("Wind speed:  {}", weather_data.wind.speed);
    if let Some(rain) = &weather_data.rain {
        println!("Rain:        {}", rain.n1h);
    }
    println!("Clouds:      {}", weather_data.clouds.all);
}
