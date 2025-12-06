use crate::util::get_input;

pub fn part1() -> String {
    let input = get_input(file!());
    let mut data = Vec::new();
    for line in input.lines().map(|s| s.trim()) {
        data.push(line.split_whitespace().collect::<Vec<_>>());
    }

    let iter = data[0].iter().zip(data[1].iter().zip(data[2].iter().zip(data[3].iter().zip(data[4].iter()))));
    let mut total = 0u64;
    for (a, (b, (c, (d, op)))) in iter {
        let a = a.parse::<u64>().unwrap();
        let b = b.parse::<u64>().unwrap();
        let c = c.parse::<u64>().unwrap();
        let d = d.parse::<u64>().unwrap();
        match *op {
            "*" => total += a * b * c * d,
            "+" => total += a + b + c + d,
            s => {
                panic!("Unexpected op {}", s);
            }
        }
    }

    total.to_string()
}

pub fn part2() -> String {
    let input = get_input(file!());
    let mut data = Vec::new();
    for line in input.lines() {
        data.push(line.chars().rev().collect::<Vec<_>>());
    }

    let iter = data[0].iter().copied()
        .zip(data[1].iter().copied())
        .zip(data[2].iter().copied())
        .zip(data[3].iter().copied())
        .zip(data[4].iter().copied())
        .map(|((((a, b), c), d), e)| [a, b, c, d, e]);

    let mut total = 0u64;
    let mut numbers = Vec::new();
    for line in iter {
        if line.iter().fold(true, |acc, c| acc && *c == ' ') {
            numbers.clear();
            continue;
        }

        let mut n = 0u64;
        for c in &line[0..line.len() - 1] {
            if *c != ' ' {
                n *= 10;
                n += c.to_digit(10).unwrap() as u64;
            }
        }
        numbers.push(n);

        match *line.last().unwrap() {
            '+' => {
                let result = numbers.iter().fold(0, |acc, n| acc + n);
                total += result;
            }
            '*' => {
                let result = numbers.iter().fold(1, |acc, n| acc * n);
                total += result;
            }
            ' ' => continue,
            _ => panic!(),
        }
    }

    total.to_string()
}
