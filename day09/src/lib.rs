use std::fmt::Display;

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u64,
    y: u64,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[allow(non_snake_case)]
fn Point(x: u64, y: u64) -> Point {
    Point { x, y }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    x_min: u64,
    x_max: u64,
    y_min: u64,
    y_max: u64,
}

impl From<(Point, Point)> for Rect {
    fn from((corner1, corner2): (Point, Point)) -> Self {
        let x_min = corner1.x.min(corner2.x);
        let x_max = corner1.x.max(corner2.x);
        let y_min = corner1.y.min(corner2.y);
        let y_max = corner1.y.max(corner2.y);

        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

impl Rect {
    fn area(self) -> u64 {
        (1 + self.x_max - self.x_min) * (1 + self.y_max - self.y_min)
    }
}

#[derive(Clone, Copy)]
struct Segment {
    start: Point,
    end: Point,
}

impl std::fmt::Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}--{:?}", self.start, self.end)
    }
}

impl From<(Point, Point)> for Segment {
    fn from((start, end): (Point, Point)) -> Self {
        Self {
            start: start.min(end),
            end: start.max(end),
        }
    }
}

impl Segment {
    fn is_horizontal(&self) -> bool {
        self.start.x != self.end.x
    }

    fn len(&self) -> u64 {
        if self.is_horizontal() {
            self.end.x - self.start.x + 1
        } else {
            self.end.y - self.start.y + 1
        }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point(x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect_vec();

    let segments = points
        .iter()
        .copied()
        .circular_tuple_windows::<(Point, Point)>()
        .map(Segment::from)
        .sorted_unstable_by_key(|s| s.len())
        .collect_vec();

    rayon::join(
        || solve_part1(points.iter().copied()),
        || solve_part2(&points, &segments),
    )
}

fn solve_part1(points: impl Iterator<Item = Point> + Clone) -> u64 {
    points
        .tuple_combinations::<(_, _)>()
        .map(Rect::from)
        .map(Rect::area)
        .max()
        .unwrap()
}

// https://www.reddit.com/r/adventofcode/comments/1phywvn/2025_day_9_solutions/nt64t2d/
fn solve_part2(points: &[Point], segments: &[Segment]) -> u64 {
    points
        .iter()
        .copied()
        .tuple_combinations::<(_, _)>()
        .map(Rect::from)
        .sorted_unstable_by_key(|r| std::cmp::Reverse(r.area()))
        .collect_vec()
        .into_par_iter()
        .find_first(|rect: &Rect| {
            !segments.iter().any(|segment: &Segment| {
                (segment.start.x < rect.x_max && segment.end.x > rect.x_min)
                    && (segment.start.y < rect.y_max && segment.end.y > rect.y_min)
            })
        })
        .unwrap()
        .area()
}
