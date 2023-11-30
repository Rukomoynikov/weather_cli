use crate::commands::current_weather::weather::Forecast;

pub fn print_results(weather_data: &Forecast) {
    let temperature = weather_data.main.temp;
    let feels_like = weather_data.main.feels_like;
    let wind_speed = weather_data.wind.speed;

    println!("Temperature: {temperature}",);
    println!("Feels like:  {feels_like}");
    println!("Wind speed:   {wind_speed}");
}
