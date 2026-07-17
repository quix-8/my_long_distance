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
    Duration(f32),     // Сработает, если просто 43
    TimeRange(String), // Сработает, если "07:45-09:12"
}

// Твоя сырая строчка из лога теперь выглядит так:
#[derive(Deserialize, Debug)]
pub struct Log {
    pub date: String,
    pub route_name: String,
    pub time_spent: TimeInput,
}

pub fn load_schedule() -> Result<Vec<Bus>, Box<dyn Error>> {
    let mut schedule: Vec<Bus> = Vec::new();
    let f = File::open("schedule.csv")?;
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Bus = result?;
        schedule.push(record);
    }
    Ok(schedule)
}

pub fn load_logs() -> Result<Vec<Log>, Box<dyn Error>> {
    let mut logs: Vec<Log> = Vec::new();
    let f = File::open("log.csv")?;
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Log = result?;
        logs.push(record);
    }
    Ok(logs)
}
