use crate::util::get_input;

pub fn part1() -> String {
    let input = get_input(file!());
    let mut total = 0;

    for line in input.lines() {
        let d: Vec<_> = line.split('x').map(|s| s.parse::<i32>().unwrap()).collect();
        let [w, h, l] = [d[0] * d[1], d[1] * d[2], d[2] * d[0]];
        let min = w.min(h.min(l));
        total += (w * 2) + (h * 2) + (l * 2) + min;
    }

    total.to_string()
}

pub fn part2() -> String {
    let input = get_input(file!());
    let mut total = 0;

    for line in input.lines() {
        let d: Vec<_> = line.split('x').map(|s| s.parse::<i32>().unwrap()).collect();
        let perim = [
            (d[0] * 2) + (d[1] * 2),
            (d[1] * 2) + (d[2] * 2),
            (d[0] * 2) + (d[2] * 2),
        ];
        let min = perim.iter().min().unwrap();
        total += min + (d[0] * d[1] * d[2]);
    }

    total.to_string()
}
