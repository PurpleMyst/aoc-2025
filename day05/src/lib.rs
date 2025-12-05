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
        .filter(|&n| ranges.iter().any(|&(start, end)| n >= start && n <= end))
        .count();

    ranges.sort_unstable();

    let mut prev = ranges[0];
    let mut part2 = 0;
    for mut next in ranges.into_iter().skip(1) {
        if prev.1 >= next.0 {
            next.1 = next.1.max(prev.1);
            prev.1 = next.0 - 1;
        }

        if prev.0 <= prev.1 {
            part2 += prev.1 - prev.0 + 1;
        }

        prev = next;
    }
    if prev.0 <= prev.1 {
        part2 += prev.1 - prev.0 + 1;
    }

    (part1, part2)
}
