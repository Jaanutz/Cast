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
        "Herzlich willkommen! Bitte gib einen Ort ein, von welchem du die Temperatur erhalten möchtest:"
    );

    let city = get_city_from_user();
    let weather = Weather::new(&city);

    println!(
        "Die Temperatur in {} beträgt aktuell {}°C, gefühlt {}°C mit einer Luftfeuchtigkeit von {}%.",
        city,
        weather.get_temperature(),
        weather.get_feels_like_temp(),
        weather.get_humidity()
    );
}
