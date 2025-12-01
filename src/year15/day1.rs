use crate::util::get_input;

pub fn part1() -> String {
    let input = get_input(file!());
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }

    floor.to_string()
}

pub fn part2() -> String {
    let input = get_input(file!());
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }

        if floor == -1 {
            return (i + 1).to_string();
        }
    }

    String::new()
}
