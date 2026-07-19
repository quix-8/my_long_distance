use core::error;

use serde::{Deserialize, Serialize};

// Кофф. для скорости обучения модели
const ALFA: f32 = 0.2;
const BETA: f32 = 0.1;
const GAMMA: f32 = 0.1;
const DELTA: f32 = 0.1;

pub struct ParsedData {
    pub edge_index: petgraph::graph::EdgeIndex, // Конкретное ребро в графе
    pub day_of_week: usize,                     // 0..6 (Пн..Вс)
    pub duration_minutes: f32,                  // Чистое время поездки
}

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
    pub fn get_predicted_cost(&self, day_of_week: usize, today_days: u32) -> f32 {
        let predict = (self.base_time + self.trend) * self.seasonal_factors[day_of_week];
        let left_behind = today_days - self.last_updated_days;
        let risk = (self.variance * (1.05_f32).powi(left_behind as i32)).min(15.0);
        predict + risk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Тест 1: Базовое обучение. Проверяем, что модель подстраивается под новую реальность.
    #[test]
    fn test_normal_learning() {
        let mut route = RouteState::new(20.0);

        // Понедельник (индекс 0). Ехали 25 минут.
        route.update_from_reality(25.0, 0, 1);

        // Проверяем, что база сдвинулась вверх (от 20.0 к 25.0)
        assert!(route.base_time > 20.0, "База должна была вырасти");

        // Проверяем, что коэффициент понедельника стал больше 1.0
        assert!(
            route.seasonal_factors[0] > 1.0,
            "Коэффициент понедельника должен вырасти"
        );

        // Проверяем тренд (он должен стать положительным)
        assert!(route.trend > 0.0, "Тренд должен быть положительным");
    }

    // Тест 2: Защита от аномалий (ДТП)
    #[test]
    fn test_anomaly_rejection() {
        let mut route = RouteState::new(20.0);
        let initial_base = route.base_time;

        // Жесткая аномалия: ехали 60 минут (при variance = 2.0 порог срабатывания = 6.0)
        route.update_from_reality(60.0, 1, 2);

        // База не должна была измениться!
        assert_eq!(
            route.base_time, initial_base,
            "Аномалия не должна менять базу"
        );

        // Счетчик аномалий должен вырасти
        assert_eq!(route.anomaly_streak, 1, "Счетчик аномалий должен стать 1");
    }

    // Тест 3: Инфляция неопределенности ("Ржавчина")
    #[test]
    fn test_rust_inflation() {
        let route = RouteState::new(20.0);

        // Спрашиваем прогноз "сегодня" (прошло 0 дней)
        let cost_today = route.get_predicted_cost(0, 0);

        // Спрашиваем прогноз "через 10 дней"
        let cost_future = route.get_predicted_cost(0, 10);

        // Будущий прогноз должен быть дороже из-за "ржавчины"
        assert!(
            cost_future > cost_today,
            "Прогноз через 10 дней должен быть дороже из-за ржавчины"
        );
    }
}
