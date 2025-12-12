use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (blocks, cases) = include_str!("input.txt")
        .rsplit_once("\n\n").unwrap();

    let areas = blocks.split("\n\n")
        .map(|block| block.bytes().filter(|&b| b == b'#').count())
        .collect::<Vec<usize>>();

    let part1 = 
        cases.lines()
        .filter(|line| {
            let (size, blocks) = line.split_once(": ").unwrap();
            let (w, h) = size.split_once('x').unwrap();
            let (w, h) = (w.parse::<usize>().unwrap(), h.parse::<usize>().unwrap());
            let total = w * h;
            let requested = blocks.split(' ')
                .map(|b| b.parse::<usize>().unwrap())
                .zip(areas.iter())
                .map(|(count, &area)| count * area)
                .sum::<usize>();
            requested <= total
        })
        .count();

    (part1, "Merry Christmas...? 🎄")
}
