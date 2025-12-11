use std::fmt::Display;

use itertools::Itertools;
use pathfinding::prelude::*;
use rayon::prelude::*;
use z3::ast::*;
use z3::*;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    (solve_part1(input), solve_part2(input))
}

fn solve_part1(input: &'static str) -> u64 {
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
        .sum::<u64>()
}

fn solve_part2(input: &'static str) -> u64 {
    input
        .par_lines()
        .map(|line| {
            let opt = Optimize::new();

            let zero = Int::from_u64(0);
            let mut presses_sum = Int::from_u64(0);

            let mut parts = line.split(' ');
            let _ = parts.next().unwrap(); // Skip lights

            let expected_counters = parse_int_list(parts.next_back().unwrap()).collect::<Vec<u64>>();

            let mut real_counters: Vec<Int> = (0..expected_counters.len()).map(|_| Int::from_u64(0)).collect();

            // Iterate over buttons (middle elements)

            let vars = parts
                .enumerate()
                .map(|(i, btn_str)| {
                    let var_name = format!("btn{}", i);
                    let var = Int::new_const(var_name);

                    opt.assert(&var.ge(&zero));

                    presses_sum = &presses_sum + &var;

                    let indices = parse_int_list::<usize>(btn_str);
                    for idx in indices {
                        real_counters[idx] = &real_counters[idx] + &var;
                    }

                    var
                })
                .collect::<Vec<Int>>();

            // Add equality constraints: real == expected
            for (real, &expected) in real_counters.iter().zip(expected_counters.iter()) {
                opt.assert(&real.eq(&Int::from_u64(expected)));
            }

            // Minimize presses
            opt.minimize(&presses_sum);

            // Solve
            match opt.check(&[]) {
                SatResult::Sat => {
                    let model = opt.get_model().unwrap();
                    vars.into_iter()
                        .map(|var| model.eval(&var, true).unwrap().as_u64().unwrap())
                        .sum::<u64>()
                }
                SatResult::Unsat | SatResult::Unknown => unreachable!(),
            }
        })
        .sum()
}

fn parse_int_list<T>(s: &str) -> impl Iterator<Item = T> + '_
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    s[1..s.len() - 1].split(',').map(|n| n.parse().unwrap())
}
