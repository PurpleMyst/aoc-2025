use std::fmt::Display;

use itertools::Itertools;

type Point = (u64, u64);

type Rect = (Point, Point);

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect_vec();
    let part1 = max_rect(points.iter().copied());
    (part1, "TODO")
}

fn max_rect(points: impl Iterator<Item = (u64, u64)> + Clone) -> u64 {
    points.tuple_combinations().map(area).max().unwrap()
}

fn area((a, b): Rect) -> u64 {
    (1 + a.0.abs_diff(b.0)) * (1 + a.1.abs_diff(b.1))
}
