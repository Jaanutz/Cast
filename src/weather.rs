use crate::get_coordinates::get_coordinates;
use dotenvy::dotenv;
use reqwest::blocking::get;
use serde_json::Value;
use std::env;

pub struct Weather {
    temperature: f64,
    feels_like_temp: f64,
    humidity: f64,
}

impl Weather {
    pub fn new(city: &String) -> Weather {
        dotenv().ok();

        let open_weather_api_key = env::var("OPEN_WEATHER_API_KEY").expect("Expected API key!");

        let coords = get_coordinates(city);

        let url = format!(
            "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&appid={}",
            coords.latitude, coords.longitude, open_weather_api_key
        );

        let response = get(url).unwrap();
        let response_text: String = response.text().unwrap();

        let json_result: Value =
            serde_json::from_str(&response_text).expect("JSON not well-formatted.");

        let temperature_kelvin_response = json_result["current"]["temp"].as_f64().unwrap_or(0.0);
        let temperature_celsius = (temperature_kelvin_response - 273.15).round();

        let feels_like_temp_response = json_result["current"]["feels_like"].as_f64().unwrap_or(0.0);
        let feels_like_temp_celsius = (feels_like_temp_response - 273.15).round();

        let humidity_response = json_result["current"]["humidity"].as_f64().unwrap_or(0.0);

        Weather {
            temperature: temperature_celsius,
            feels_like_temp: feels_like_temp_celsius,
            humidity: humidity_response,
        }
    }

    pub fn get_temperature(&self) -> f64 {
        return self.temperature;
    }

    pub fn get_feels_like_temp(&self) -> f64 {
        return self.feels_like_temp;
    }

    pub fn get_humidity(&self) -> f64 {
        return self.humidity;
    }
}
