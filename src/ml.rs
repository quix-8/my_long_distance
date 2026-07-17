use core::error;

use serde::{Deserialize, Serialize};

// Кофф. для скорости обучения модели
const ALFA: f32 = 0.2;
const BETA: f32 = 0.1;
const GAMMA: f32 = 0.1;
const DELTA: f32 = 0.1;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteState {
    pub base_time: f32,
    pub trend: f32,
    pub seasonal_factors: [f32; 7],
    pub variance: f32,
    pub anomaly_streak: u8,
    pub last_updated_days: u32,
}

impl RouteState {
    // Конструктор: рождение нового маршрута (например, из расписания)
    pub fn new(initial_time: f32) -> Self {
        Self {
            base_time: initial_time,
            trend: 0.0,
            seasonal_factors: [1.0; 7],
            variance: 2.0,
            anomaly_streak: 0,
            last_updated_days: 0,
        }
    }

    // Метод для обучения, уже вечером после предсказаний
    pub fn update_from_reality(&mut self, t_real: f32, day_of_week: usize, today_days: u32) {
        let pred_time = (self.base_time + self.trend) * self.seasonal_factors[day_of_week];
        let error = (t_real - pred_time).abs();
        if error > self.variance * 3.0 {
            self.anomaly_streak += 1;
            self.last_updated_days = today_days;
        } else {
            self.anomaly_streak = 0;

            let old = self.base_time;

            self.base_time = ALFA * (t_real / self.seasonal_factors[day_of_week])
                + (1.0 - ALFA) * (self.base_time + self.trend);

            self.trend = BETA * (self.base_time - old) + self.trend * (1.0 - BETA);

            self.seasonal_factors[day_of_week] = GAMMA * (t_real / self.base_time)
                + self.seasonal_factors[day_of_week] * (1.0 - GAMMA);

            self.variance = DELTA * error + (1.0 - DELTA) * self.variance;

            self.last_updated_days = today_days;
        }
    }

    // Метод предсказания новых весов
    pub fn get_predicted_cost(&self, target_day: usize, today_days: u32) -> f32 {
        let predict = (self.base_time + self.trend) * self.seasonal_factors[target_day];
        let left_behind = today_days - self.last_updated_days;
        let risk = self.variance * 1.05 * left_behind as f32;
        predict + risk
    }
}
