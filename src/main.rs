use std::time::{Instant, Duration};

use ansi_term::Color;
use term_table::{row, row::Row, rows, table_cell::*, Table, TableStyle};

mod util;
mod year15;
mod year25;

fn run_year(year: &str, funcs: &[[fn() -> String; 2]], color: Color) {
    let mut table = Table::builder()
        .rows(rows![row!(
            color.bold().paint(year),
            Color::Yellow.paint("★"),
            Color::Yellow.paint("★★"),
            "⌚",
        )])
        .style(TableStyle::rounded())
        .build();

    for (i, day) in funcs.iter().enumerate() {
        let now = Instant::now();
        let parts = [day[0](), day[1]()];
        let elapsed = now.elapsed().as_micros();
        let time = format!("{}ms {}μs", elapsed / 1000, elapsed % 1000);
        let time = format!("{}ms", elapsed as f64 / 1000.0);

        let row = row!(format!("Day {}", i + 1), &parts[0], &parts[1], time);
        table.add_row(row);
    }

    print!("{}", table.render());
}

fn main() {
    let mut colors = [
        Color::Blue,
        Color::Purple,
        Color::Cyan,
        Color::Green,
    ].iter().cycle();

    run_year("2015", year15::FUNCS, *colors.next().unwrap());
    run_year("2025", year25::FUNCS, *colors.next().unwrap());
}
