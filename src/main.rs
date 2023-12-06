use reqwest::Error;

mod commands;
mod utils;

use commands::current_weather::get_current_weather::get_current_weather;
use commands::set_config::update_config_value::update_config_value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.get(0) {
        None => {
            get_current_weather(&args).await.unwrap();
        }
        Some(command) => match command.as_str() {
            "current" => {
                get_current_weather(&args).await.unwrap();
            }
            "config" => match update_config_value(&args) {
                Ok(_) => {}
                Err(err) => {
                    println!("{err}");
                    return Ok(());
                }
            },
            "help" => {
                println!("Commands:");
                println!("  wthr config api_key 1234567890 - set api key");
                println!("  wthr config default_town Cardiff - set default town");
                println!("  wthr current london - get current weather for London");
                println!("  wthr current- get current weather for default town");
                println!("  wthr 4d - get weather forecast for next 4 days");
            },
            "4d" => {}
            unknown_command => {
                println!("Command \"{unknown_command}\" is not found");
                return Ok(());
            }
        },
    };

    Ok(())
}
