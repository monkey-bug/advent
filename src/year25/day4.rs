use crate::util::get_input;

pub fn part1() -> String {
    let input = get_input(file!());
    let grid: Vec<Vec<bool>> = input.lines().map(|line|
        line.trim().chars().map(|c| c == '@').collect()
    ).collect();

    let (w, h) = (grid[0].len() as i32, grid.len() as i32);
    let mut rolls = Vec::new();

    for y in 0..h {
        for x in 0..w {
            if !grid[y as usize][x as usize] {
                continue;
            }

            let neighbors = [
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y - 1),
                (x, y + 1),
            ];
            let mut total = 0;
            for dir in neighbors {
                if dir.0 < 0 || dir.0 >= w || dir.1 < 0 || dir.1 >= h {
                    continue;
                }
                if grid[dir.1 as usize][dir.0 as usize] {
                    total += 1;
                }
            }

            if total < 4 {
                rolls.push((x, y));
            }
        }
    }

    rolls.len().to_string()
}

pub fn part2() -> String {
    let input = get_input(file!());
    let mut grid: Vec<Vec<bool>> = input.lines().map(|line|
        line.trim().chars().map(|c| c == '@').collect()
    ).collect();

    let (w, h) = (grid[0].len() as i32, grid.len() as i32);
    let mut removed = 0;

    loop {
        let mut rolls = Vec::new();
        for y in 0..h {
            for x in 0..w {
                if !grid[y as usize][x as usize] {
                    continue;
                }
                let neighbors = [
                    (x + 1, y - 1),
                    (x + 1, y),
                    (x + 1, y + 1),
                    (x - 1, y - 1),
                    (x - 1, y),
                    (x - 1, y + 1),
                    (x, y - 1),
                    (x, y + 1),
                ];
                let mut total = 0;
                for dir in neighbors {
                    if dir.0 < 0 || dir.0 >= w || dir.1 < 0 || dir.1 >= h {
                        continue;
                    }
                    if grid[dir.1 as usize][dir.0 as usize] {
                        total += 1;
                    }
                }
                if total < 4 {
                    rolls.push((x, y));
                }
            }
        }
        if rolls.is_empty() {
            break;
        }
        for roll in rolls {
            grid[roll.1 as usize][roll.0 as usize] = false;
            removed += 1;
        }
    }

    removed.to_string()
}
