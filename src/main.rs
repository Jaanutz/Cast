mod get_coordinates;
mod weather;

use crate::weather::Weather;
use std::io::{self};

fn get_city_from_user() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("ERROR: Input went wrong.");

    return input.trim().to_string();
}

fn main() {
    println!(
        "Welcome! Please enter a location for which you would like to get the current weather:"
    );

    let city = get_city_from_user();
    let weather = Weather::new(&city);

    println!(
        "The temperature in {} is currently {}°C, but feels like {}°C and a humidity of {}%.",
        city,
        weather.get_temperature(),
        weather.get_feels_like_temp(),
        weather.get_humidity()
    );
}
