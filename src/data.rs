use std::fs;
use std::borrow::Borrow;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct MainData {
    pub cal_data: Vec<String>,
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
            },
            Err(_) => vec!["".to_string()]
        };

        MainData { cal_data }
    }


}
