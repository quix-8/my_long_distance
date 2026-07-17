use crate::io::Log;
use crate::io::TimeInput;
use crate::ml;
use crate::ml::ParsedData;
use chrono::NaiveTime;
use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{default, error::Error, fs};

pub struct Adapter {
    map: HashMap<String, petgraph::graph::EdgeIndex>,
}

impl Adapter {
    fn new(&mut self, m: HashMap<String, petgraph::graph::EdgeIndex>) {
        self.map = m;
    }
    fn get_time(input: &TimeInput) -> Option<f32> {
        match input {
            // Если готовые минуты (43.0)
            TimeInput::Duration(mins) => Some(*mins),

            // Если интервал ("07:45-09:12")
            TimeInput::TimeRange(range_str) => {
                let parts: Vec<&str> = range_str.split('-').collect();
                if parts.len() != 2 {
                    return None;
                }

                // Парсим часы и минуты
                let start = NaiveTime::parse_from_str(parts[0].trim(), "%H:%M").ok()?;
                let end = NaiveTime::parse_from_str(parts[1].trim(), "%H:%M").ok()?;

                // Считаем дельту и переводим в f32
                let duration = end.signed_duration_since(start);
                Some(duration.num_seconds() as f32 / 60.0)
            }
        }
    }
    pub fn transform(&self, log: Log) -> Option<ParsedData> {
        let index = self.map.get(&log.route_name)?;
        let time = Self::get_time(&log.time_spent)?;

        Some(ParsedData {
            edge_index: *index,
            day_of_week: 1, // заглушка
            duration_minutes: time,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stop {
    pub name: String,
}

fn load_graph() -> Result<Graph<Stop, ml::RouteState>, Box<dyn Error>> {
    let loaded_json = fs::read_to_string("graph.json")?;
    let loaded_graph: Graph<Stop, ml::RouteState> = serde_json::from_str(&loaded_json)?;
    println!("Граф успешно загружен из файла!");
    Ok(loaded_graph)
}

fn save_graph(graph: &Graph<Stop, ml::RouteState>) -> anyhow::Result<()> {
    let json_data = serde_json::to_string_pretty(&graph)?;
    fs::write("graph.json", &json_data)?;
    println!("Граф успешно сохранен в graph.json!");
    Ok(())
}
