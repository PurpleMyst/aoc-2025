use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let width = input.lines().next().unwrap().trim().len();
    let mut map = grid::Grid::from_vec(
        input
            .bytes()
            .filter(|b| !b.is_ascii_whitespace())
            .map(|b| b == b'@')
            .collect(),
        width,
    );

    let mut part1 = None;
    let mut part2 = 0usize;

    let mut to_remove = Vec::new();

    loop {
        for ((y, x), &cell) in map.indexed_iter() {
            if !cell {
                continue;
            }

            let mut neighbors = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let ny = y.wrapping_add_signed(dy);
                    let nx = x.wrapping_add_signed(dx);
                    if map.get(ny, nx) == Some(&true) {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                to_remove.push((y, x));
            }
        }

        if to_remove.is_empty() {
            break;
        }

        part1.get_or_insert(to_remove.len());
        part2 += to_remove.len();

        for (y, x) in to_remove.drain(..) {
            map[(y, x)] = false;
        }
    }

    (part1.unwrap_or_default(), part2)
}

