use std::collections::HashSet;

use crate::util::get_input;

pub fn part1() -> String {
    let input = get_input(file!());
    let mut pos = [0, 0];
    let mut map: HashSet<[i32; 2]> = HashSet::new();
    map.insert(pos);

    for c in input.trim().chars() {
        match c {
            '>' => pos[0] += 1,
            '<' => pos[0] -= 1,
            '^' => pos[1] -= 1,
            'v' => pos[1] += 1,
            _ => panic!(),
        }
        map.insert(pos);
    }

    map.len().to_string()
}

pub fn part2() -> String {
    let input = get_input(file!());
    let mut pos1 = [0, 0];
    let mut pos2 = [0, 0];
    let mut map: HashSet<[i32; 2]> = HashSet::new();
    map.insert(pos1);

    for (i, c) in input.trim().chars().enumerate() {
        let pos = if i % 2 == 0 { &mut pos1 } else { &mut pos2 };
        match c {
            '>' => pos[0] += 1,
            '<' => pos[0] -= 1,
            '^' => pos[1] -= 1,
            'v' => pos[1] += 1,
            _ => panic!(),
        }
        map.insert(*pos);
    }

    map.len().to_string()
}
