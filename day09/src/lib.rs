use std::fmt::Display;

use indicatif::ProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;

// Point defined by its cartesian coordinates.
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

// Rect defined as its opposite corners.
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

    fn x_range(&self) -> Range {
        Range(self.x_min, self.x_max)
    }

    fn y_range(&self) -> Range {
        Range(self.y_min, self.y_max)
    }
}

// 1D range defined as its endpoints.
#[derive(Clone, Copy, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}

#[derive(Clone)]
struct RangeSet {
    ranges: Vec<Range>,
}

impl std::fmt::Debug for RangeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ranges.fmt(f)
    }
}

impl RangeSet {
    fn one(r: Range) -> Self {
        Self { ranges: vec![r] }
    }

    fn cut(&mut self, knife: Range) {
        let mut new_ranges = Vec::with_capacity(self.ranges.len());
        for r in self.ranges.drain(..) {
            let Some(inter) = r.intersect(&knife) else {
                new_ranges.push(r);
                continue;
            };

            // The knife fully cuts out this range.
            if inter == r {
                continue;
            }

            if r.start <= inter.start - 1 {
                new_ranges.push(Range(r.start, inter.start - 1));
            }

            if inter.end + 1 <= r.end {
                new_ranges.push(Range(inter.end + 1, r.end));
            }
        }

        self.ranges = new_ranges;
    }
}

impl IntoIterator for RangeSet {
    type Item = Range;

    type IntoIter = <Vec<Range> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into_iter()
    }
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..={}", self.start, self.end)
    }
}

#[allow(non_snake_case)]
fn Range(start: u64, end: u64) -> Range {
    debug_assert!(start <= end, "{start} > {end}");
    Range { start, end }
}

impl Range {
    fn contains(&self, n: u64) -> bool {
        n >= self.start && n <= self.end
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        let inter_start = self.start.max(other.start);
        let inter_end = self.end.min(other.end);
        (inter_start <= inter_end).then(|| Range(inter_start, inter_end))
    }
}

impl IntoIterator for Range {
    type Item = u64;

    type IntoIter = std::ops::RangeInclusive<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
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

    fn x_range(&self) -> Range {
        Range(self.start.x.min(self.end.x), self.start.x.max(self.end.x))
    }

    fn x(&self) -> u64 {
        debug_assert!(!self.is_horizontal());
        self.start.x
    }

    fn y(&self) -> u64 {
        debug_assert!(self.is_horizontal());
        self.start.y
    }

    fn y_range(&self) -> Range {
        Range(self.start.y.min(self.end.y), self.start.y.max(self.end.y))
    }

    fn contains(&self, p: Point) -> bool {
        if self.is_horizontal() {
            p.y == self.start.y && self.x_range().contains(p.x)
        } else {
            p.x == self.start.x && self.y_range().contains(p.y)
        }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    color_eyre::install().unwrap();

    let input = include_str!("input.txt");
    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point(x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect_vec();

    let (horizontal_segments, vertical_segments) = points
        .iter()
        .copied()
        .circular_tuple_windows::<(Point, Point)>()
        .map(Segment::from)
        .partition::<Vec<_>, _>(|segment: &Segment| segment.is_horizontal());

    let part1 = solve_part1(points.iter().copied());

    let part2 = points
        .iter()
        .copied()
        .tuple_combinations::<(_, _)>()
        .map(Rect::from)
        .sorted_by_key(|r| std::cmp::Reverse(r.area()))
        .progress()
        .find(|rect: &Rect| {
            let interested_horizontal_segments = horizontal_segments
                .iter()
                .copied()
                .filter(|seg| rect.x_range().intersect(&seg.x_range()).is_some() && rect.y_range().contains(seg.y()))
                .collect_vec();

            // Iterating over all cells would be unfeasible, but the first algorithm simplification
            // that comes to mind still involves iterating over each row.
            for y in rect.y_range() {
                // In this row, points may be green if they are part of an horizontal segment or if
                // they are inside the larger polygon; let's first try to handle the horizontal
                // segment case.

                // We'll handle it by "paring down" what interval we have to check for "green-ness
                // by being inside the polygon"; at the start, this is everything.
                let mut to_check = RangeSet::one(Range(rect.x_min, rect.x_max));

                // Horizontal segments inside the rect and on this row do not need to be checked
                // for further greenery.
                interested_horizontal_segments
                    .iter()
                    .copied()
                    .filter(|seg| seg.y() == y)
                    .for_each(|seg| to_check.cut(seg.x_range()));

                // Any other points on this row which lay on vertical segments are also green, so
                // remove them from to_check as well.
                vertical_segments
                    .iter()
                    .copied()
                    // .filter(|seg| (x_min..=x_max).contains(&seg.0.0) && (seg.0.1..=seg.1.1).contains(&y))
                    .filter(|seg| rect.x_range().contains(seg.x()) && seg.y_range().contains(y))
                    .for_each(|seg| to_check.cut(Range(seg.x(), seg.x())));

                // For the remaining points, we have to check if a ray going to the right from them
                // intersects an odd number of vertical segments... but we must do this without
                // just iterating over the segments.
                // Here's an idea:
                //     For each x_min..=x_max segment in to_check, we should check how many
                //     vertical segments there are with x >=x_min; the answer should be odd, and
                //     even then (maybe) we need to check that there's no vertical segments with x
                //     ∈ [x_min, x_max], as that messes up the parity for at least one point
                //     surely.

                if to_check.ranges.into_par_iter().any(|r| {
                    let vertical_segments_to_right = vertical_segments
                        .iter()
                        .filter(|seg| seg.y_range().contains(y) && seg.y_range().contains(y - 1) && seg.x() > r.end)
                        .count();

                    vertical_segments_to_right % 2 == 0
                }) {
                    return false;
                }
            }

            true
        })
        .unwrap()
        .area();

    assert!(part2 > 19_019_542, "answer too low from website: {part2} <= 19_019_542");

    (part1, part2)
}

fn solve_part1(points: impl Iterator<Item = Point> + Clone) -> u64 {
    points
        .tuple_combinations::<(_, _)>()
        .map(Rect::from)
        .map(Rect::area)
        .max()
        .unwrap()
}
