use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::{default, error::Error, fs};

use crate::io;
use crate::ml;

// 1. Описываем данные узла (Остановка)
// Обязательно вешаем макросы Serialize и Deserialize
#[derive(Serialize, Deserialize, Debug)]
pub struct Stop {
    pub name: String,
}

fn load_graph() -> Result<Graph<Stop, ml::RouteState>, Box<dyn Error>> {
    let loaded_json = fs::read_to_string("graph.json")?;
    // if loaded_json.is_empty() {
    //     let default = mio::load_csv()?;
    // }
    let loaded_graph: Graph<Stop, ml::RouteState> = serde_json::from_str(&loaded_json)?;
    println!("Граф успешно загружен из файла!");
    Ok(loaded_graph)
}

fn save_graph(graph: Graph<Stop, ml::RouteState>) -> anyhow::Result<()> {
    let json_data = serde_json::to_string_pretty(&graph)?;
    fs::write("graph.json", &json_data)?;
    println!("Граф успешно сохранен в graph.json!");
    Ok(())
}
