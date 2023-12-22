use anyhow::Result;

mod api_client;
mod commands;
mod entities;
mod utils;

use crate::commands::get_5d_forecast::get_4d_forecast;
use commands::get_current_weather::get_current_weather;
use commands::update_config_value::update_config_value;

#[tokio::main]
async fn main() -> Result<()> {
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
            "4d" => {
                get_4d_forecast(&args).await?;
                println!("Sorry development of 4days forecast is in progress");
            }
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
            unknown_command => {
                eprintln!("Command \"{unknown_command}\" is not found");
                return Ok(());
            }
        },
    };

    Ok(())
}
