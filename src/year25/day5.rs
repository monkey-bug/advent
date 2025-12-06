use std::collections::BTreeSet;
use crate::util::get_input;

pub fn part1() -> String {
    let input = get_input(file!());
    let mut lines_iter = input.lines().map(|line| line.trim());

    let mut ranges = Vec::new();
    for line in &mut lines_iter {
        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once('-').unwrap();
        let (x, y) = (start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap());
        ranges.push(x..=y);
    }

    let ingredients: Vec<usize> = lines_iter.map(|s| s.parse::<usize>().unwrap()).collect();
    let mut total = 0;
    for i in &ingredients {
        for r in &ranges {
            if r.contains(i) {
                total += 1;
                break;
            }
        }
    }

    total.to_string()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, i: usize) -> bool {
        i >= self.start && i <= self.end
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

// TODO: currently wrong
pub fn part2() -> String {
    let input = get_input(file!());
    // let input = r#"
    //     3-5
    //     10-14
    //     16-20
    //     12-18
    //
    //     1
    //     5
    //     8
    //     11
    //     17
    //     32
    // "#.trim();

    let mut set = BTreeSet::new();
    for line in input.lines().map(|line| line.trim()) {
        if line.is_empty() {
            break;
        }
        let parts = line.split_once('-').unwrap();
        let (start, end) = (parts.0.parse::<usize>().unwrap(), parts.1.parse::<usize>().unwrap());
        set.insert(Range { start, end });
    }

    let mut iter = set.into_iter();
    let mut result = BTreeSet::new();
    let mut current = iter.next().unwrap();

    for next in iter {
        if next.start <= (current.end + 1) {
            current.end = next.end.max(current.end);
        } else {
            result.insert(current);
            current = next;
        }
    }
    result.insert(current);

    // (result.iter().fold(0, |acc, r| acc + (r.end - r.start) + 1)).to_string()
    String::new()
}
