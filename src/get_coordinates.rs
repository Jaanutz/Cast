use dotenvy::dotenv;
use reqwest::blocking::get;
use serde_json::Value;
use std::env;

pub struct Coordinates {
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
}

fn api_request(city: &String) -> Value {
    dotenv().ok();

    let open_weather_api_key = env::var("OPEN_WEATHER_API_KEY").expect("Expected API key!");

    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
        city, open_weather_api_key
    );

    let response = get(url).unwrap();
    let response_text: String = response.text().unwrap();

    let json: Value = serde_json::from_str(&response_text).expect("JSON not well-formatted.");

    return json;
}

pub fn get_coordinates(city: &String) -> Coordinates {
    let result: Value = api_request(city);

    let lat = result[0].get("lat").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let lon = result[0].get("lon").and_then(|v| v.as_f64()).unwrap_or(0.0);

    return Coordinates {
        latitude: lat,
        longitude: lon,
    };
}
