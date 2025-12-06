use crate::util::get_input;

fn count_digits(mut n: u64) -> u64 {
    let mut digits = if n == 0 { 1 } else { 0 };
    while n > 0 {
        digits += 1;
        n /= 10;
    }
    digits
}

pub fn part1() -> String {
    let input = get_input(file!());
    let mut invalid = 0;

    fn is_invalid(n: u64) -> bool {
        let digits = count_digits(n);
        if digits & 1 == 0 {
            let base = 10_u64.pow((digits / 2).try_into().unwrap());
            let lower = n % base;
            let upper = (n - lower) / base;
            if lower == upper {
                return true;
            }
        }
        false
    }

    for range in input.trim().split(",") {
        let range = range.split("-").map(|x| x.parse().unwrap()).collect::<Vec<u64>>();
        let [start, end] = [range[0], range[1]];

        for n in start..=end {
            if is_invalid(n) {
                invalid += n;
            }
        }
    }

    invalid.to_string()
}

pub fn part2() -> String {
    let input = get_input(file!());
    let mut invalid = 0;

    fn is_invalid(n: u64) -> bool {
        let digits = count_digits(n) as u32;
        'len: for len in 1..=(digits / 2) {
            if digits % len != 0 {
                continue;
            }
            let mut n = n;
            let mut base = 10_u64.pow(digits - len);
            let head = n / base;
            n %= base;
            loop {
                base /= 10_u64.pow(len);
                if base == 0 {
                    break;
                }

                if n / base != head {
                    continue 'len;
                }
                n %= base;
            }
            return true;
        }
        false
    }

    for range in input.trim().split(",") {
        let range = range.split("-").map(|x| x.parse().unwrap()).collect::<Vec<u64>>();
        let [start, end] = [range[0], range[1]];
        for n in start..=end {
            if is_invalid(n) {
                invalid += n;
            }
        }
    }

    invalid.to_string()
}
