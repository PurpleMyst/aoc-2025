use std::fmt::Display;

use atoi::FromRadix10;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    (part1, part2)
}

fn solve_part1(input: &'static str) -> u64 {
    let matrix: Vec<Vec<&'static str>> = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();
    let width = matrix[0].len();
    let height = matrix.len();

    let mut part1 = 0;
    for x in 0..width {
        let mul = matrix[height - 1][x] == "*";
        let mut result = if mul { 1 } else { 0 };

        for y in 0..height - 1 {
            let n = u64::from_radix_10(matrix[y][x].as_bytes()).0;
            if mul {
                result *= n;
            } else {
                result += n;
            }
        }
        part1 += result;
    }
    part1
}

fn solve_part2(input: &'static str) -> u64 {
    let width = input.lines().map(|line| line.len()).max().unwrap();
    let height = input.lines().count();

    let binput = input.as_bytes();
    let get = |y: usize, x: usize| binput[y * (width + 1) + x];

    let mut dividers = Vec::new();
    for x in 0..width {
        if (0..height).all(|y| get(y, x) == b' ') {
            dividers.push(x);
        }
    }
    dividers.push(width);
    let mut part2 = 0;

    let mut start = 0;
    for end in dividers {
        let mul = get(height-1, start) == b'*';
        let mut result = if mul { 1 } else { 0 };

        for x in start..end {
            let mut n = 0;

            let mut mult = 0;
            for y in 0..height {
                mult *= 10;
                let cell = get(y, x);
                if cell.is_ascii_digit() {
                    n *= mult;
                    n += (cell - b'0') as u64;
                    mult = 1;
                }
            }
            if mul {
                result *= n;
            } else {
                result += n;
            }
        }
        part2 += result;

        start = end + 1;
    }
    part2
}
