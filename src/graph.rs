use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

// 1. Описываем данные узла (Остановка)
// Обязательно вешаем макросы Serialize и Deserialize
#[derive(Serialize, Deserialize, Debug)]
pub struct Stop {
    pub name: String,
}

// 2. Описываем данные ребра (Состояние маршрута из нашего ML)
#[derive(Serialize, Deserialize, Debug)]
pub struct RouteState {
    pub weight: f32,
    pub anomaly_streak: u8,
}

fn load_graph() -> Result<Graph<Stop, RouteState>, Box<dyn Error>> {
    let loaded_json = fs::read_to_string("graph.json")?;
    let loaded_graph: Graph<Stop, RouteState> = serde_json::from_str(&loaded_json)?;
    println!("Граф успешно загружен из файла!");
    Ok(loaded_graph)
}

fn save_graph(graph: Graph<Stop, RouteState>) -> anyhow::Result<()> {
    let json_data = serde_json::to_string_pretty(&graph)?;
    fs::write("graph.json", &json_data)?;
    println!("Граф успешно сохранен в graph.json!");
    Ok(())
}
