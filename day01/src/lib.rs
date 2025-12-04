use std::fmt::Display;

use atoi::FromRadix10Signed;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let instrs = input.lines().map(|line| {
        let bs = line.as_bytes();
        let sign = match bs[0] {
            b'L' => -1,
            b'R' => 1,
            _ => unreachable!(),
        };
        i64::from_radix_10_signed(&bs[1..]).0 * sign
    });

    let mut pos = 50;
    let mut part1 = 0usize;
    let mut part2 = 0usize;
    for delta in instrs {
        for _ in 0..delta.abs() {
            pos = (pos + delta.signum()).rem_euclid(100);

            if pos == 0 {
                part2 += 1;
            }
        }
        if pos == 0 {
            part1 += 1;
        }
    }

    (part1, part2)
}
