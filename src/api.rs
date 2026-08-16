use dotenvy::dotenv;
use std::env;

pub struct Api {
    key: String,
}

impl Api {
    pub fn get_api_key() -> String {
        dotenv().ok();

        return env::var("OPEN_WEATHER_API_KEY").expect("Expected API key!");
    }
}
