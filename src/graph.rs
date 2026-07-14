use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::fs;

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
