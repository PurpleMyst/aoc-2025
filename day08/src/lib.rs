use std::fmt::Display;

use itertools::Itertools;
use union_find::*;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let boxes = input
        .lines()
        .map(|line| -> (i64, i64, i64) {
            line.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let mut circuits = QuickUnionUf::<UnionBySize>::new(boxes.len());
    let mut part1 = 0;
    let mut part2 = 0;
    let mut merges = 0;

    for (k, pair) in boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .sorted_unstable_by_key(|(a, b)| dist(*a.1, *b.1))
        .enumerate()
    {
        if k == 1000 {
            let mut size = vec![0usize; boxes.len()];
            for i in 0..boxes.len() {
                size[circuits.find(i)] += 1;
            }
            size.sort_unstable();
            size.reverse();
            part1 = size[0] * size[1] * size[2];
        }

        let i = pair.0.0;
        let j = pair.1.0;
        if circuits.union(i, j) {
            merges += 1;
        }

        if merges == boxes.len() - 1 {
            part2 = pair.0.1.0 * pair.1.1.0;
            break;
        }
    }

    (part1, part2)
}

fn dist(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}
