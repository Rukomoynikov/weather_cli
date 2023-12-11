use reqwest::Error;

mod api_client;
mod commands;
mod entities;
mod utils;

use commands::get_current_weather::get_current_weather;
use commands::update_config_value::update_config_value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.get(0) {
        None => match get_current_weather(&args).await {
            Ok(_) => {}
            Err(err) => {
                println!("{err}");
                return Ok(());
            }
        },
        Some(command) => match command.as_str() {
            "current" => match get_current_weather(&args).await {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("{err}");
                    return Ok(());
                }
            },
            "config" => match update_config_value(&args) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("{err}");
                    return Ok(());
                }
            },
            "help" => {
                println!("Commands:");
                println!("  forecaster config api_key 1234567890   - set api key");
                println!("  forecaster config default_town Cardiff - set default town");
                println!(
                    "  forecaster current london              - get current weather for London"
                );
                println!("  forecaster current                     - get current weather for default town taken from settings");
                println!("  forecaster 4d london                   - get weather forecast for next 4 days");
                println!("  forecaster 4d                          - get weather forecast for next 4 days for default town taken from settings");
            }
            "4d" => {
                println!("Sorry development of 4days forecast is in progress");
            }
            unknown_command => {
                eprintln!("Command \"{unknown_command}\" is not found");
                return Ok(());
            }
        },
    };

    Ok(())
}
