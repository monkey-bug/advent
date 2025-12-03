pub mod day1;
pub mod day2;

pub static FUNCS: &[[fn() -> String; 2]] = &[
    [day1::part1, day1::part2],
    [day2::part1, day2::part2],
];

