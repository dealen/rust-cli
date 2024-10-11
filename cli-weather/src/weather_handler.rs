use crate::location::WeatherData;
use api_handler::handler::Handler;
use log::info;

pub struct WeatherHandler {
    pub key: String,
}

impl WeatherHandler {
    pub fn new(key: String) -> WeatherHandler {
        WeatherHandler { key: key }
    }

    pub fn get_info_from_weather_api(
        &self,
        city: &str,
    ) -> Result<WeatherData, Box<dyn std::error::Error>> {
        let url = format!(
            "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no&lang=pl",
            self.key, city
        );

        info!("Sending request to {}", url);

        let handler = Handler::new(url);
        let response = handler.get_resposnse()?;

        info!("Response: {}", response);

        let weather_data: WeatherData = serde_json::from_str(&response)?;

        Ok(weather_data)
    }
}
