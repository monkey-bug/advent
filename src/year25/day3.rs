use crate::util::get_input;

pub fn part1() -> String {
    let input = get_input(file!());
    let banks: Vec<Vec<_>> = input.lines().map(|line|
        line.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    ).collect();
    let mut total = 0u32;

    for bank in banks {
        let mut max = 0;
        for i in 0..(bank.len() - 1) {
            let head = bank[i];
            for n in &bank[(i + 1)..] {
                let result = (head * 10) + n;
                max = max.max(result);
            }
        }
        total += max as u32;
    }

    total.to_string()
}

pub fn part2() -> String {
    let input = get_input(file!());
    let banks: Vec<Vec<_>> = input.lines().map(|line|
        line.trim().chars().map(|c| c.to_digit(10).unwrap() as u64).collect()
    ).collect();
    let mut total = 0u64;

    for bank in banks {
        let mut max = [0u64; 12];
        let mut start = 0;
        for (i, digit) in max.iter_mut().enumerate() {
            let mut max = 0;
            let mut index = 0;
            let slice = &bank[start..(bank.len() - 12 + (i + 1))];
            for (j, n) in slice.iter().enumerate() {
                if *n > max {
                    max = *n;
                    index = j;
                }
            }
            start += index + 1;
            *digit = max;
        }
        total += max.iter().fold(0, |acc, digit| acc * 10 + digit);
    }

    total.to_string()
}
