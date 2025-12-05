use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let (ranges, tests) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges
        .lines()
        .map(|range| {
            let (a, b) = range.split_once('-').unwrap();
            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    let part1 = tests
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .filter(|n| ranges.iter().any(|&(start, end)| (start..=end).contains(&n)))
        .count();

    ranges.sort_unstable();

    'outer: loop {
        for i in 0..ranges.len() - 1 {
            let [a, b] = ranges.get_disjoint_mut([i, i + 1]).unwrap();
            debug_assert!(a.0 <= b.0);

            if a.0 == b.0 {
                a.1 = a.1.max(b.1);
                ranges.remove(i + 1);
                continue 'outer;
            }

            if a.1 >= b.0 {
                b.1 = a.1.max(b.1);
                a.1 = b.0 - 1;
            }
        }
        break;
    }
    let part2 = ranges.into_iter().map(|(a, b)| b - a + 1).sum::<u64>();

    (part1, part2)
}

