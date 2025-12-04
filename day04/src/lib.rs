use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let width = input.lines().next().unwrap().trim().len();
    let map = grid::Grid::from_vec(
        input
            .bytes()
            .filter(|b| !b.is_ascii_whitespace())
            .map(|b| b == b'@')
            .collect(),
        width,
    );

    let mut neighbors = grid::Grid::new(map.rows(), map.cols());
    for ((y, x), &cell) in map.indexed_iter() {
        if !cell {
            neighbors[(y, x)] = u8::MAX;
            continue;
        }

        let mut cur_neighbors = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let ny = y.wrapping_add_signed(dy);
                let nx = x.wrapping_add_signed(dx);
                if map.get(ny, nx) == Some(&true) {
                    cur_neighbors += 1;
                }
            }
        }

        neighbors[(y, x)] = cur_neighbors;
    }

    let part1 = neighbors.iter().filter(|&&n| n < 4).count();

    let mut part2 = 0usize;

    let mut to_remove = neighbors
        .indexed_iter()
        .filter_map(|((y, x), n)| (*n < 4).then_some((y, x)))
        .collect::<Vec<_>>();

    while let Some((y, x)) = to_remove.pop() {
        neighbors[(y, x)] = u8::MAX;
        part2 += 1;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let ny = y.wrapping_add_signed(dy);
                let nx = x.wrapping_add_signed(dx);
                if let Some(n) = neighbors.get_mut(ny, nx) {
                    *n -= 1;
                    if *n == 3 {
                        to_remove.push((ny, nx));
                    }
                }
            }
        }
    }

    (part1, part2)
}
