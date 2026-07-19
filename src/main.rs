use crate::io::clear_and_archive_logs;
use chrono::Local;
mod graph;
mod io;
mod ml;
mod render;

fn main() -> anyhow::Result<()> {
    let now = Local::now();
    let today_days = (now.timestamp() / 86400) as u32;
    let logs = io::load_logs()?;
    let qgraph = graph::load_graph()?;
    if logs.is_empty() {
        println!("Отсутствуют новые логи")
    } else {
        let re = clear_and_archive_logs();
        match re {
            Ok(_) => println!("Логи заархивированы"),
            Err(e) => println!("Ошибка архивации: {}", e),
        }
    }
    Ok(())
}
