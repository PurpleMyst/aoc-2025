use std::{fmt::Display, mem::swap};

use fixedbitset::FixedBitSet;
use grid::Grid;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let width = input.lines().next().unwrap().len();
    let map = Grid::from_vec(input.lines().flat_map(|l| l.bytes()).collect(), width);
    let start = map.indexed_iter().find(|(_pos, cell)| **cell == b'S').unwrap().0;

    (solve_part1(&map, start), solve_part2(&map, start))
}

fn solve_part1(map: &Grid<u8>, start: (usize, usize)) -> i32 {
    debug_assert_eq!(start.0, 0);
    let mut part1 = 0;
    let mut xs = FixedBitSet::with_capacity(map.cols());
    let mut new_xs = FixedBitSet::with_capacity(map.cols());
    xs.insert(start.1);
    for y in 0..map.rows()-1 {
        let next_y = y + 1;
        for x in xs.ones() {
            if map[(next_y, x)] == b'^' {
                part1 += 1;
                if x != 0 {
                    new_xs.insert(x - 1);
                }
                if x != map.cols() - 1 {
                    new_xs.insert(x + 1);
                }
            } else {
                new_xs.insert(x);
            }
        }
        xs.clear();
        swap(&mut xs, &mut new_xs);
    }
    part1
}

fn solve_part2(map: &Grid<u8>, pos: (usize, usize)) -> usize {
    let mut memo: Grid<Option<usize>> = Grid::new(map.rows(), map.cols());
    solve_part2_rec(map, pos, &mut memo)
}

fn solve_part2_rec(map: &Grid<u8>, pos: (usize, usize), memo: &mut Grid<Option<usize>>) -> usize {
    if let Some(cached_result) = memo[pos] {
        return cached_result;
    }

    let (y, x) = pos;
    let new_y = y + 1;

    let result = if new_y == map.rows() {
        1
    } else if map[(new_y, x)] == b'^' {
        let left = if x != 0 {
            solve_part2_rec(map, (y, x - 1), memo)
        } else {
            0
        };

        let right = if x != map.cols() - 1 {
            solve_part2_rec(map, (y, x + 1), memo)
        } else {
            0
        };

        left + right
    } else {
        solve_part2_rec(map, (new_y, x), memo)
    };

    memo[pos] = Some(result);
    result
}
