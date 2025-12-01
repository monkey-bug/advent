use std::{env, time::Instant};

use ansi_term::Color;
use term_table::{row, rows, row::*, table_cell::*, Table, TableStyle};

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
        let time = format!("{}ms", elapsed as f64 / 1000.0);

        let row = row!(format!("Day {}", i + 1), &parts[0], &parts[1], time);
        table.add_row(row);
    }

    print!("{}", table.render());
}

fn main() {
    let years = [
        ("2015", year15::FUNCS),
        ("2025", year25::FUNCS),
    ];

    let mut colors = [
        Color::Blue,
        Color::Purple,
        Color::Cyan,
        Color::Green,
    ].iter().cycle();

    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 0 {
        let year = &args[0];
        let funcs = years.iter().find(|x| x.0 == year).unwrap().1;
        run_year(year, funcs, *colors.next().unwrap());
    } else {
        for (year, funcs) in years {
            run_year(&year, funcs, *colors.next().unwrap());
        }
    }
}
