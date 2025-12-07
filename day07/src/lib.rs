use std::{collections::HashSet, fmt::Display, mem::swap};

use grid::Grid;
use memoize::memoize;
use rustc_hash::FxHashMap;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    memoized_flush_solve_part2(); // for benchmarking purposes

    let input = include_str!("input.txt");
    let width = input.lines().next().unwrap().len();
    let map = Grid::from_vec(input.lines().flat_map(|l| l.bytes()).collect(), width);
    let start = map.indexed_iter().find(|(_pos, cell)| **cell == b'S').unwrap().0;

    (solve_part1(&map, start), solve_part2(&map, start))
}

fn solve_part1(map: &Grid<u8>, start: (usize, usize)) -> i32 {
    let mut beams = HashSet::new();
    let mut new_beams = HashSet::new();
    beams.insert(start);
    let mut part1 = 0;
    while !beams.is_empty() {
        for (y, x) in beams.drain() {
            let new_y = y + 1;
            if new_y == map.rows() {
                continue;
            }
            if map[(new_y, x)] == b'^' {
                part1 += 1;
                if x != 0 {
                    new_beams.insert((new_y, x - 1));
                }
                if x != map.cols() - 1 {
                    new_beams.insert((new_y, x + 1));
                }
            } else {
                new_beams.insert((new_y, x));
            }
        }

        swap(&mut beams, &mut new_beams);
    }
    part1
}

#[memoize(Ignore: map, CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn solve_part2(map: &Grid<u8>, pos: (usize, usize)) -> usize {
    let (y, x) = pos;
    let new_y = y + 1;
    if new_y == map.rows() {
        return 1;
    }
    if map[(new_y, x)] == b'^' {
        (if x != 0 { solve_part2(map, (y, x - 1)) } else { 0 })
            + (if x != map.cols() - 1 {
                solve_part2(map, (y, x + 1))
            } else {
                0
            })
    } else {
        solve_part2(map, (new_y, x))
    }
}
