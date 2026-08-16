use dotenvy::dotenv;
use std::env;

pub struct OpenWeatherApi {
    key: String,
}

impl OpenWeatherApi {
    pub fn new() -> Self {
        dotenv().ok();
        let key = env::var("OPEN_WEATHER_API_KEY").expect("Expected API key!");

        return Self { key }
    }

    pub fn get_api_key(&self) -> &str {
        return &self.key
    }
}
