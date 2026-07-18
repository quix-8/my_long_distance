use crate::io::clear_and_archive_logs;

mod graph;
mod io;
mod ml;
mod render;

fn main() -> anyhow::Result<()> {
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
