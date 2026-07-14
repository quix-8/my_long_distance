use chrono::DateTime;
use chrono::Local;
use csv;
use serde;
use serde_json;
use std::fs::File;
use std::io;

#[derive(Debug, serde::Deserialize)]
struct Bus {
    id: u8,
    time: DateTime<Local>,
}

fn load_csv() -> std::io::Result<()> {
    let mut schedule: Vec<Bus> = Vec::new();
    let f = File::open("data.csv")?;
    let mut rdr = csv::Reader::from_reader(f);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Bus = result?;
        schedule.push(record);
    }
    Ok(())
}
