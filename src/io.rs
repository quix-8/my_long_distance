use chrono::DateTime;
use chrono::Local;
use csv;
use serde;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io;

#[derive(Debug, serde::Deserialize)]
pub struct Bus {
    id: u8,
    time: DateTime<Local>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum TimeInput {
    Duration(f32),
    TimeRange(String),
}

#[derive(Deserialize, Debug)]
pub struct Log {
    pub date: String,
    pub day_of_week: String,
    pub route_name: String,
    pub time_spent: TimeInput,
}

pub fn load_logs() -> Result<Vec<Log>, Box<dyn Error>> {
    let mut logs: Vec<Log> = Vec::new();
    let f = File::open("log.csv")?;
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        let record: Log = result?;
        logs.push(record);
    }
    Ok(logs)
}
