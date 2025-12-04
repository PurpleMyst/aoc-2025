use std::{fmt::Display, iter::zip};

fn is_valid_id_for_p1(n: u64) -> bool {
    let s = n.to_string();
    if s.len() % 2 != 0 {
        return true;
    }
    let l = s.len() / 2;
    s.as_bytes()[..l] != s.as_bytes()[l..]
}

fn is_valid_id_for_p2(n: u64) -> bool {
    let s = n.to_string();
    for l in 1..=s.len() / 2 {
        let mut chunks = s.as_bytes().chunks(l);
        let representative = chunks.next().unwrap();
        if chunks.all(|c| c == representative) {
            return false;
        }
    }
    true
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let part1 = input
        .trim()
        .split(',')
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start..=end).filter(|&n| !is_valid_id_for_p1(n)).sum::<u64>()
        })
        .sum::<u64>();

    let part2 = input
        .trim()
        .split(',')
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start..=end).filter(|&n| !is_valid_id_for_p2(n)).sum::<u64>()
        })
        .sum::<u64>();

    (part1, part2)
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

