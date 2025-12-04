use std::fmt::Display;

use memoize::memoize;

fn max_joltage_p1(battery: &[u8]) -> u8 {
    let mut largest = 0;
    let mut dp = Vec::new();

    for c in battery.iter().map(|&c| c - b'0').rev() {
        dp.push(largest);
        if c > largest {
            largest = c;
        }
    }
    dp.reverse();

    battery
        .iter()
        .map(|&c| c - b'0')
        .zip(dp)
        .take(battery.len() - 1)
        .map(|(c1, c2)| c1 * 10 + c2)
        .max()
        .unwrap()
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{a}{b}").parse().unwrap()
}

#[memoize]
fn max_joltage_p2(battery: &'static [u8], used: usize) -> u64 {
    if battery.is_empty() || used == 12 {
        return 0;
    }

    let (first, rest) = battery.split_first().unwrap();
    let next = u64::from(*first - b'0');

    (concat(next, max_joltage_p2(rest, used + 1))).max(max_joltage_p2(rest, used))
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let part1 = input
        .lines()
        .map(|line| max_joltage_p1(line.as_bytes()) as u64)
        .sum::<u64>();

    let part2 = input
        .lines()
        .map(|line| max_joltage_p2(line.as_bytes(), 0) as u64 / 10)
        .sum::<u64>();

    (part1, part2)
}

