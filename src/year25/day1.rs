use crate::util::get_input;

pub fn part1() -> String {
    let input = get_input(file!());
    let mut dial = 50;
    let mut zeros = 0;

    for line in input.lines().map(|line| line.trim()) {
        let dir = line.chars().next().unwrap();
        let amount: i32 = line[1..].parse().unwrap();

        if dir == 'L' {
            dial -= amount;
        } else if dir == 'R' {
            dial += amount;
        } else {
            panic!("unrecognized direction: {}", dir);
        }

        if dial < 0 {
            dial += 100;
        }

        dial %= 100;

        if dial == 0 {
            zeros += 1;
        }
    }

    zeros.to_string()
}

pub fn part2() -> String {
    let input = get_input(file!());
    let mut dial = 50;
    let mut zeros = 0;

    for line in input.lines().map(|line| line.trim()) {
        let dir = line.chars().next().unwrap();
        let mut amount: i32 = line[1..].parse().unwrap();
        let start = dial;

        zeros += amount / 100;
        amount %= 100;

        if dir == 'L' {
            dial -= amount;
        } else if dir == 'R' {
            dial += amount;
        } else {
            panic!("unrecognized direction: {}", dir);
        }

        if dial < 0 {
            dial += 100;
            if start != 0 {
                zeros += 1;
            }
        } else if dial > 99 {
            dial %= 100;
            zeros += 1;
        } else if dial == 0 {
            zeros += 1;
        }
    }

    zeros.to_string()
}
