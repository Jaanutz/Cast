# 🌦️ Cast

**Cast** is a lightweight CLI weather tool built with **Rust**.

I started this project as a hands-on journey to explore the Rust ecosystem, focusing on performance and seamless API integration.

## ✨ Features
Currently, Cast provides real-time weather data via the OpenWeather API. Simply input a city to get:
* **Temperature**
* **Humidity**
* **Feels Like** temperature

## 🚀 Roadmap
The CLI is the beginning. The goal is to evolve **Cast** into a full-scale desktop application using the **Tauri Framework** to provide a sleek, modern user interface.

## 🛠️ Installation & Setup

To get Cast running locally, follow these steps:

### 1. Get your API Key
You'll need a free API key from OpenWeather:
1. Create an account at [openweathermap.org](https://openweathermap.org/).
2. Generate your API Key in your dashboard.

### 2. Configure Environment
In the root directory of this repository, create a file named `.env` and add your key:
```
OPEN_WEATHER_API_KEY=your_api_key_here
```

### 3. Run the App
Make sure you have Rust and Cargo installed. Then, simply run:
```
cargo run
```
Enter any city name when prompted and get your weather data instantly.

---
*Developed as a learning journey into the world of Rust.* 🦀