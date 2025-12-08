use std::fmt::Display;

use itertools::Itertools;
use rayon::prelude::*;
use union_find::*;
use atoi::FromRadix10;
use wide::u64x4;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let boxes = input
        .lines()
        .map(|line| -> (u64, u64, u64) {
            line.split(',')
                .map(|n| u64::from_radix_10(n.as_bytes()).0)
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let mut edges = Vec::with_capacity(boxes.len() * (boxes.len() - 1) / 2);
    
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let d = dist(boxes[i], boxes[j]);
            edges.push((d, i as u16, j as u16));
        }
    }

    edges.par_sort_unstable_by_key(|(d, _, _)| *d);

    let mut circuits = QuickUnionUf::<UnionBySize>::new(boxes.len());
    let mut part1 = 0;
    let mut part2 = 0;
    let mut merges = 0;

    for (k, pair) in edges.into_iter().enumerate() {
        if k == 1000 {
            let mut size = vec![0usize; boxes.len()];
            for i in 0..boxes.len() {
                size[circuits.find(i)] += 1;
            }
            size.sort_unstable();
            size.reverse();
            part1 = size[0] * size[1] * size[2];
        }

        // ↓ Returns true if the two elements were in different sets
        if circuits.union(pair.1 as usize, pair.2 as usize) {
            merges += 1;
        }

        if merges == boxes.len() - 1 {
            part2 = boxes[pair.1 as usize].0 * boxes[pair.2 as usize].0;
            break;
        }
    }

    (part1, part2)
}

#[inline(always)]
fn dist(a: (u64, u64, u64), b: (u64, u64, u64)) -> u64 {
    // 1. Convert tuples to SIMD vectors (Structure of Arrays)
    // We pad the 4th lane with 0.
    let a_vec = u64x4::from([a.0, a.1, a.2, 0]);
    let b_vec = u64x4::from([b.0, b.1, b.2, 0]);

    // 2. Compute difference
    // Note: SIMD integer subtraction is usually wrapping. 
    // In modular arithmetic, (a - b)^2 is mathematically valid 
    // even if (a - b) wraps around, so we don't need explicit abs_diff logic.
    let diff = a_vec - b_vec;

    // 3. Square (Multiplication)
    let squared = diff * diff;

    // 4. Horizontal Sum
    // We extract the array and sum it up to get the scalar result.
    let arr: [u64; 4] = squared.into();
    arr.into_iter().sum()
}
