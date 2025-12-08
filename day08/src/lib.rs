use std::fmt::Display;

use itertools::Itertools;

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

    let mut circuits = (0..boxes.len()).collect_vec();
    let mut part1 = 0;
    let mut part2 = 0;

    for (k, pair) in boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .sorted_unstable_by_key(|(a, b)| dist(*a.1, *b.1))
        .enumerate()
    {
        if k == 1000 {
            let mut size = vec![0usize; boxes.len()];
            for &c in &circuits {
                size[c] += 1;
            }
            size.sort_unstable();
            size.reverse();
            part1 = size[0] * size[1] * size[2];
        }

        let i = pair.0.0;
        let j = pair.1.0;
        let orig_c_i = circuits[i];
        let orig_c_j = circuits[j];
        let new_c = orig_c_i.min(orig_c_j);
        circuits.iter_mut().for_each(|d| {
            if *d == orig_c_i || *d == orig_c_j {
                *d = new_c
            }
        });

        if circuits.iter().all(|&c| c == circuits[0]) {
            part2 = pair.0.1.0 * pair.1.1.0;
            break;
        }
    }

    (part1, part2)
}

fn dist(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}
