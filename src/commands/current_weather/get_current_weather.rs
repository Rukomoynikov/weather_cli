use crate::commands::current_weather::coordinates::get_coords_from_city_name;
use crate::commands::current_weather::print_result::print_results;
use crate::commands::current_weather::weather::get_weather;
use crate::utils::config::read_config;
use std::error::Error;

pub async fn get_current_weather(args: &[String]) -> Result<(), Box<dyn Error>> {
    let config = read_config();

    let city_name = match args.get(1) {
        Some(city_name) => city_name.clone(),
        None => match config.default_town {
            None => {
                return Err("No city was provided in arguments or set as default in config".into())
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
