extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_json::Value;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct CurrentWeather {
    pub temp: f64,
    pub description: String,
    pub icon: String,
}

pub struct MainData {
    pub cal_data: Vec<String>,
    pub current_weather: CurrentWeather,
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

        let rewuest_url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q=Dachau,de&units=metric&APPID={}",
            api_key
        );

        let weather = reqwest::blocking::get(&rewuest_url)
            .expect("Failed to load current weather data!")
            .json::<Value>()
            .expect("Failed to convert current weather data to jason!");

        let temp = weather["main"]["temp"].as_f64().unwrap().round();
        let desc = String::from(weather["weather"][0]["description"].to_string().replace("\"", ""));
        let mut icon = String::from(weather["weather"][0]["icon"].to_string().replace("\"", ""));
        icon.push_str(".png");
        let current_weather = CurrentWeather  { temp, description: desc, icon: icon.to_string(),};

        MainData { cal_data, current_weather, api_key }
    }
}
