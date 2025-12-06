use std::fmt::Display;

use rayon::prelude::*;

fn is_valid_id_for_p1(n: u64) -> bool {
    let l = n.ilog10() + 1;
    if l % 2 != 0 {
        return true;
    }
    n % 10u64.pow(l / 2 as u32) != n / 10u64.pow(l / 2 as u32)
}

fn is_valid_id_for_p2(n: u64) -> bool {
    let l = n.ilog10() + 1;
    'outer: for k in 1..=l / 2 {
        if l % k != 0 {
            continue;
        }
        let mask = 10u64.pow(k as u32);
        let first_part = n % mask;
        let mut rest = n / mask;
        while rest != 0 {
            if rest % mask != first_part {
                continue 'outer;
            }
            rest /= mask;
        }
        return false;
    }
    true
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
                    let start = start.parse::<u64>().unwrap();
                    let end = end.parse::<u64>().unwrap();
                    (start..=end).filter(|&n| !is_valid_id_for_p1(n)).sum::<u64>()
                })
                .sum::<u64>()
        },
        || {
            input
                .trim()
                .par_split(',')
                .map(|line| {
                    let (start, end) = line.split_once('-').unwrap();
                    let start = start.parse::<u64>().unwrap();
                    let end = end.parse::<u64>().unwrap();
                    (start..=end).filter(|&n| !is_valid_id_for_p2(n)).sum::<u64>()
                })
                .sum::<u64>()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!((11..=22).filter(|&n| !is_valid_id_for_p1(n)).count(), 2);
        assert_eq!((95..=115).filter(|&n| !is_valid_id_for_p1(n)).count(), 1);
        assert_eq!((998..=1012).filter(|&n| !is_valid_id_for_p1(n)).count(), 1);
        assert_eq!((1188511880..=1188511890).filter(|&n| !is_valid_id_for_p1(n)).count(), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!((11..=22).filter(|&n| !is_valid_id_for_p2(n)).count(), 2);
        assert_eq!((95..=115).filter(|&n| !is_valid_id_for_p2(n)).count(), 2);
        assert_eq!((998..=1012).filter(|&n| !is_valid_id_for_p2(n)).count(), 2);
        assert_eq!((1188511880..=1188511890).filter(|&n| !is_valid_id_for_p2(n)).count(), 1);
    }
}
