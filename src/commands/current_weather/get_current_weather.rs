use crate::commands::current_weather::coordinates::get_coords_from_city_name;
use crate::commands::current_weather::print_result::print_results;
use crate::commands::current_weather::weather::get_weather;
use crate::utils::config::read_config;

pub async fn get_current_weather(args: &[String]) -> Result<(), std::io::Error> {
    let config = read_config();

    let city_name = match args.get(1) {
        Some(city_name) => city_name.clone(),
        None => match config.default_town {
            None => {
                println!("No city was provided in arguments or set as default in config");
                return Ok(());
            }
            Some(default_town) => default_town,
        },
    };

    let place = match get_coords_from_city_name(&city_name).await {
        None => return Ok(()),
        Some(place) => place,
    };

    let weather = get_weather((&place.lat, &place.lon))
        .await
        .ok_or("Wrong request to weather service");

    let weather = match weather {
        Ok(weather) => weather,
        Err(_) => return Ok(()),
    };

    print_results(&weather);

    Ok(())
}
