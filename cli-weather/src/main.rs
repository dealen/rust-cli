use clap::Parser;

pub mod weather_handler;
pub mod location;

#[derive(Parser)]
struct Cli {
    city: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let matches = Cli::parse();

    let city = matches.city;

    let weather_handler = weather_handler::WeatherHandler::new("KEY".to_string());
    let data = weather_handler.get_info_from_weather_api(&city).unwrap();

    let result =  format!("The weather in {} is {} with temperature of {}°C", data.location.name, data.current.condition.text, data.current.temp_c);
    print!("{}", result);

    Ok(())
}
