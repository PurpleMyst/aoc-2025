use std::fmt::Display;

use memoize::memoize;
use rustc_hash::FxHashMap as HashMap;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    memoized_flush_solve_part1();
    memoized_flush_solve_part2();

    let input = include_str!("input.txt");

    let g: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (src, dsts) = line.split_once(": ").unwrap();
            let dsts = dsts.split(' ').collect::<Vec<_>>();
            (src, dsts)
        })
        .collect();

    (solve_part1(&g, "you"), solve_part2(&g, "svr", 0))
}

#[memoize(Ignore: g, CustomHasher: HashMap, HasherInit: HashMap::default())]
fn solve_part1(g: &HashMap<&'static str, Vec<&'static str>>, node: &'static str) -> usize {
    g.get(node).map_or(0, |adjacent| {
        adjacent
            .iter()
            .map(|&next| if next == "out" { 1 } else { solve_part1(g, next) })
            .sum()
    })
}

#[memoize(Ignore: g, CustomHasher: HashMap, HasherInit: HashMap::default())]
fn solve_part2(g: &HashMap<&'static str, Vec<&'static str>>, node: &'static str, visited: u8) -> usize {
    g.get(node).map_or(0, |adjacent| {
        adjacent
            .iter()
            .map(|&next| {
                if next == "out" {
                    if visited == 0b11 { 1 } else { 0 }
                } else {
                    solve_part2(
                        g,
                        next,
                        if next == "dac" {
                            visited | 0b01
                        } else if next == "fft" {
                            visited | 0b10
                        } else {
                            visited
                        },
                    )
                }
            })
            .sum()
    })
}
