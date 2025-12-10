use std::fmt::Display;

use itertools::Itertools;
use pathfinding::prelude::*;
use z3::*;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    (solve_part1(input), "TODO")
}

fn solve_part1(input: &'static str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let ll = parts
                .next()
                .unwrap()
                .strip_prefix("[")
                .unwrap()
                .strip_suffix("]")
                .unwrap();

            let lights = ll
                .bytes()
                .rev()
                .enumerate()
                .filter_map(|(i, b)| (b == b'#').then_some(1 << i))
                .fold(0, |acc, mask| acc | mask);
            let _joltage = parts.next_back().unwrap();
            let buttons = parts
                .map(|button| {
                    button
                        .strip_prefix("(")
                        .unwrap()
                        .strip_suffix(")")
                        .unwrap()
                        .split(',')
                        .map(|n| 1 << (ll.len() as u16 - 1 - n.parse::<u16>().unwrap()))
                        .fold(0, |acc, mask| acc | mask)
                })
                .collect_vec();

            dijkstra(
                &0u16,
                |&n| buttons.iter().map(move |&btn| (n ^ btn, 1)),
                |&n| n == lights,
            )
            .unwrap()
            .1
        })
        .sum::<usize>()
}
