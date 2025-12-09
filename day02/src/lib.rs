use std::fmt::Display;

use atoi::FromRadix10;
use rayon::prelude::*;

fn digits(n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    (n.ilog10() + 1) as usize
}

// https://old.reddit.com/r/adventofcode/comments/1pbzqcx/2025_day_2_solutions/nrwn5ta/
fn sum_repeated_in_range_p1(lower: u64, upper: u64) -> u64 {
    let max_total_digits = digits(upper);
    let mut result: u64 = 0;

    let lower = lower as u128;
    let upper = upper as u128;

    // d is the number of digits in the repeated block, r is the number of blocks
    for d in 1..=max_total_digits {
        let r = 2;
        // 10^d and 10^(d*r)
        let pow10_d = 10u128.pow(d as u32);
        let pow10_dr = 10u128.pow((d * r) as u32);

        let f = (pow10_dr - 1) / (pow10_d - 1);

        if f > upper {
            continue; // even k = 1 would be too big
        }

        let min_k128 = 10u128.pow((d - 1) as u32);
        let max_k128 = pow10_d - 1;

        let k_lo = ((lower + f - 1) / f).max(min_k128);
        let k_hi = (upper / f).min(max_k128);

        if k_lo > k_hi {
            continue;
        }

        result += ((k_lo * f).max(lower)..=(k_hi * f).min(upper))
            .step_by(f as usize)
            .map(|n| n as u64)
            .sum::<u64>();
    }

    result
}

fn sum_repeated_in_range_p2(lower: u64, upper: u64) -> u64 {
    let max_total_digits = digits(upper);
    let mut candidates: Vec<u64> = Vec::new();

    let lower = lower as u128;
    let upper = upper as u128;

    // d is the number of digits in the repeated block, r is the number of blocks
    for d in 1..=max_total_digits {
        for r in 2..=max_total_digits / d {
            // 10^d and 10^(d*r)
            let pow10_d = 10u128.pow(d as u32);
            let pow10_dr = 10u128.pow((d * r) as u32);

            let f = (pow10_dr - 1) / (pow10_d - 1);

            if f > upper {
                continue; // even k = 1 would be too big
            }

            let min_k128 = 10u128.pow((d - 1) as u32);
            let max_k128 = pow10_d - 1;

            let k_lo = ((lower + f - 1) / f).max(min_k128);
            let k_hi = (upper / f).min(max_k128);

            if k_lo > k_hi {
                continue;
            }

            candidates.extend(
                ((k_lo * f).max(lower)..=(k_hi * f).min(upper))
                    .step_by(f as usize)
                    .map(|n| n as u64),
            );
        }
    }

    candidates.sort_unstable();
    candidates.dedup();
    candidates.into_iter().sum::<u64>()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    rayon::join(
        || {
            input
                .trim()
                .par_split(',')
                .map(|line| {
                    let (start, end) = line.split_once('-').unwrap();
                    let start = u64::from_radix_10(start.as_bytes()).0;
                    let end = u64::from_radix_10(end.as_bytes()).0;
                    sum_repeated_in_range_p1(start, end)
                })
                .sum::<u64>()
        },
        || {
            input
                .trim()
                .par_split(',')
                .map(|line| {
                    let (start, end) = line.split_once('-').unwrap();
                    let start = u64::from_radix_10(start.as_bytes()).0;
                    let end = u64::from_radix_10(end.as_bytes()).0;
                    sum_repeated_in_range_p2(start, end)
                })
                .sum::<u64>()
        },
    )
}
