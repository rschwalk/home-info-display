extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate chrono;

use serde_json::Value;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use chrono::prelude::*;

pub struct CurrentWeather {
    pub temp: f64,
    pub min: f64,
    pub max: f64,
    pub description: String,
    pub icon: String,
}

pub struct Forecast {
    pub temp_min: f64,
    pub temp_max: f64,
    pub forecast_icon: String,
    pub date: String,
}

pub struct MainData {
    pub cal_data: Vec<String>,
    pub current_weather: CurrentWeather,
    pub forecast: Vec<Forecast>,
    pub curr_time: String,
    pub curr_date: String,
    api_key: String,
}

impl MainData {
    pub fn load_data() -> Self {
        let file = File::open("data/cal.txt");
        let cal_data = match file {
            Ok(f) => {
                let buf = BufReader::new(f);
                buf.lines()
                    .map(|l| l.expect("Could not parse line!"))
                    .collect()
            }
            Err(_) => vec!["".to_string()],
        };

        let api_key =
            std::fs::read_to_string("assets/api.txt").expect("Reading the api file failed!");

        let request_url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q=Dachau,de&units=metric&APPID={}",
            api_key
        );

        // TODO: Handle network error and display invalid data
        let weather = reqwest::blocking::get(&request_url)
            .expect("Failed to load current weather data!")
            .json::<Value>()
            .expect("Failed to convert current weather data to jason!");

        let temp = weather["main"]["temp"].as_f64().unwrap().round();
        let desc = String::from(
            weather["weather"][0]["description"]
                .to_string()
                .replace("\"", ""),
        );
        let mut icon = String::from(weather["weather"][0]["icon"].to_string().replace("\"", ""));
        icon.push_str(".png");

        let forecast_url = format!("http://api.openweathermap.org/data/2.5/forecast/daily?q=Dachau,de&cnt=4&units=metric&APPID={}", api_key);

        // TODO: Handle network error and display invalid data
        let forecast = reqwest::blocking::get(&forecast_url)
            .expect("Failed to load forecast data!")
            .json::<Value>()
            .expect("Failed to convert forecast data to json");
        let mut forecast_days = Vec::new();
        let mut today_min: f64 = 0.0;
        let mut today_max: f64 = 0.0;
        for n in 0..4 {
            match n {
                0 => {
                    today_max = forecast["list"][n]["temp"]["max"].as_f64().unwrap().round();
                    today_min = forecast["list"][n]["temp"]["min"].as_f64().unwrap().round();
                }
                _ => {
                    let dt = forecast["list"][n]["dt"].as_i64().unwrap();
                    let time = Local.timestamp(dt, 0);
                    let date = time.format("(%:wA)").to_string();
                    let temp_max = forecast["list"][n]["temp"]["max"].as_f64().unwrap().round();
                    let temp_min = forecast["list"][n]["temp"]["min"].as_f64().unwrap().round();
                    let mut forecast_icon = String::from(forecast["list"][n]["weather"][0]["icon"].to_string().replace("\"", ""));
                    forecast_icon.push_str(".png");
                    let f = Forecast { temp_min, temp_max, forecast_icon, date, };
                    forecast_days.push(f);
                }
            }
        }

        let current_time = Local::now();
        let curr_time = current_time.format("%H:%M").to_string();
        let curr_date = current_time.format("%A, %d. %B %Y").to_string();
        println!("{}", curr_date);

        let current_weather = CurrentWeather {
            temp,
            description: desc,
            max: today_max,
            min: today_min,
            icon: icon.to_string(),
        };

        MainData {
            cal_data,
            current_weather,
            forecast: forecast_days,
            curr_time,
            curr_date,
            api_key,
        }
    }
}
