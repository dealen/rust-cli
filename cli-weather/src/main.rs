#![warn(clippy::all, clippy::pedantic)]
use clap::Parser;
pub mod location;
pub mod weather_handler;

#[derive(Parser)]
struct Cli {
    city: String,
}

fn main() {
    let matches = Cli::parse();

    let city = matches.city;

    let weather_handler = weather_handler::WeatherHandler::new(String::new());
    let data = weather_handler.get_info_from_weather_api(&city).unwrap();

    let result = format!(
        "The weather in {} is {} with temperature of {}°C",
        data.location.name, data.current.condition.text, data.current.temp_c
    );
    let result2 = format!(
        "Wilgotnośc powietrza {}%. Siła wiatru {} km/h i podmuchy o sile {} km/h",
        data.current.humidity, data.current.wind_kph, data.current.gust_kph
    );
    println!("{result}");
    println!("{result2}");
}
