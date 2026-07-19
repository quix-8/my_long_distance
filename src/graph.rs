use crate::io::Log;
use crate::io::TimeInput;
use crate::ml;
use crate::ml::ParsedData;
use crate::ml::RouteState;
use chrono::NaiveTime;
use petgraph::Graph;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};
use std::any;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::{default, error::Error};

pub struct Adapter {
    map: HashMap<String, petgraph::graph::EdgeIndex>,
    days: HashMap<String, usize>,
}

impl Adapter {
    fn new(
        &mut self,
        m: HashMap<String, petgraph::graph::EdgeIndex>,
        d: HashMap<String, usize>,
    ) -> Self {
        Self { map: m, days: d }
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
        let day = self.days.get(&log.day_of_week)?;

        Some(ParsedData {
            edge_index: *index,
            day_of_week: *day,
            duration_minutes: time,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stop {
    pub name: String,
}
pub fn find_best_route(
    graph: &Graph<Stop, RouteState>,
    start: NodeIndex,
    goal: NodeIndex,
    day_of_week: usize,
    today_days: u32,
) -> Option<(f32, Vec<String>)> {
    let result = astar(
        graph,
        start,
        |finish| finish == goal,
        |e| {
            let state = e.weight();
            state.get_predicted_cost(day_of_week, today_days)
        },
        |_| 0.0,
    );
    match result {
        Some((total_cost, path_indices)) => {
            let route_names: Vec<String> = path_indices
                .iter()
                .map(|&idx| graph[idx].name.clone())
                .collect();

            Some((total_cost, route_names))
        }
        None => None,
    }
}

fn prompt(message: &str) -> Result<String, anyhow::Error> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn generate() {
    println!("=== Генератор Графа ===");
    let mut graph = Graph::<Stop, RouteState>::new();
    let mut nodes = Vec::new();

    println!("\n--- Добавление остановок ---");
    loop {
        let name = prompt("Введите название остановки (или пустую строку для завершения): ");
        match name {
            Ok(name) => {
                if name.is_empty() {
                    break;
                }
                let node_index = graph.add_node(Stop { name: name.clone() });
                nodes.push((node_index, name));
            }
            Err(e) => eprint!("Ошибка при записи ответа: {}", e),
        }
    }

    println!("\n--- Создание маршрутов (ребер) ---");
    loop {
        println!("\nДоступные остановки:");
        for (i, (_, name)) in nodes.iter().enumerate() {
            println!("{}: {}", i, name);
        }

        let from_input = prompt("Введите номер остановки ОТКУДА (или пустую строку для выхода): ");
        let from_idx = match from_input {
            Ok(input) if input.is_empty() => break,
            Ok(input) => match input.parse::<usize>() {
                Ok(idx) if idx < nodes.len() => idx,
                Ok(_) => {
                    eprintln!("Ошибка: индекс остановки вне диапазона.");
                    continue;
                }
                Err(_) => {
                    eprintln!("Ошибка: введите корректное число.");
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Ошибка при записи ответа: {}", e);
                continue;
            }
        };

        let to_input = prompt("Введите номер остановки КУДА: ");
        let to_idx = match to_input {
            Ok(input) => match input.parse::<usize>() {
                Ok(idx) if idx < nodes.len() => idx,
                Ok(_) => {
                    eprintln!("Ошибка: индекс остановки вне диапазона.");
                    continue;
                }
                Err(_) => {
                    eprintln!("Ошибка: введите корректное число.");
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Ошибка при записи ответа: {}", e);
                continue;
            }
        };

        let time_input = prompt("Введите базовое время в пути (в минутах, например 15.5): ");
        let base_time = match time_input {
            Ok(input) => match input.parse::<f32>() {
                Ok(t) => t,
                Err(_) => {
                    eprintln!("Ошибка: введите корректное число с плавающей точкой.");
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Ошибка при записи ответа: {}", e);
                continue;
            }
        };

        let route_state = RouteState::new(base_time);

        graph.add_edge(nodes[from_idx].0, nodes[to_idx].0, route_state);
        println!("Ребро добавлено!");
    }

    match serde_json::to_string_pretty(&graph) {
        Ok(json_data) => {
            if let Err(e) = fs::write("graph.json", &json_data) {
                eprintln!("Ошибка при записи файла: {}", e);
            } else {
                println!("\nУспех! Файл graph.json сгенерирован и готов к работе.");
            }
        }
        Err(e) => eprintln!("Ошибка при сериализации графа: {}", e),
    }
}

pub fn load_graph() -> Result<Graph<Stop, RouteState>, anyhow::Error> {
    let loaded_json = fs::read_to_string("graph.json")?;
    let loaded_graph: Graph<Stop, RouteState> = serde_json::from_str(&loaded_json)?;
    println!("Граф успешно загружен из файла!");
    Ok(loaded_graph)
}

pub fn save_graph(graph: &Graph<Stop, RouteState>) -> anyhow::Result<()> {
    let json_data = serde_json::to_string_pretty(&graph)?;
    fs::write("graph.json", &json_data)?;
    println!("Граф успешно сохранен в graph.json!");
    Ok(())
}
