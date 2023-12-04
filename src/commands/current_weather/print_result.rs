use crate::commands::current_weather::weather::Forecast;

pub fn print_results(weather_data: &Forecast) {
    println!("Temperature: {}", weather_data.main.temp);
    println!("Feels like:  {}", weather_data.main.feels_like);
    println!("Wind speed:  {}", weather_data.wind.speed);
}
