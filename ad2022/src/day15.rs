use std::collections::HashSet;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Point = (i64, i64);
type Input = Vec<(Point, Point)>;

#[aoc_generator(day15, part1, jorendorff)]
#[aoc_generator(day15, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let point = parser!("x=" i64 ", y=" i64);
    let p = parser!(lines(
        "Sensor at " s:point ": closest beacon is at " b:point
    ));
    Ok(p.parse(text)?)
}

const TARGET_ROW: i64 = 2_000_000;

fn paint(painted: &mut [u8], start: i64, stop: i64) {
    assert!(start < stop);
    let start = (start + 5_000_000) as usize;
    let stop = (stop + 5_000_000) as usize;
    for i in &mut painted[start..stop] {
        *i = 1;
    }
}

fn unbeaconable(input: &Input, target_row: i64) -> u64 {
    // this algorithm is unconscionable
    let mut painted = vec![0u8; 20_000_000];
    let mut beacons_on_row = HashSet::new();
    for ((x, y), (bx, by)) in input.iter().copied() {
        let d = x.abs_diff(bx) + y.abs_diff(by);
        let dtarget = y.abs_diff(target_row);
        if dtarget <= d {
            let r = (d - dtarget) as i64;
            paint(&mut painted, x - r, x + r + 1);
        }

        if by == target_row {
            beacons_on_row.insert(bx);
        }
    }

    painted.into_iter().map(|x| x as u64).sum::<u64>() - beacons_on_row.len() as u64
}

#[aoc(day15, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    unbeaconable(input, TARGET_ROW)
}

fn distance(p: Point, q: Point) -> u64 {
    p.0.abs_diff(q.0) + p.1.abs_diff(q.1)
}

fn find_distress_signal(input: &Input, xmax: i64, ymax: i64) -> i64 {
    // This runs in half a second on my laptop. Fast enough.
    for y in 0..=ymax {
        let mut x = 0;
        'xloop: while x <= xmax {
            for ((sx, sy), (bx, by)) in input.iter().copied() {
                let d = distance((sx, sy), (bx, by));
                let dtarget = sy.abs_diff(y);
                if dtarget <= d {
                    let r = (d - dtarget) as i64;
                    assert_eq!(distance((sx, sy), (sx - r, y)), d);
                    assert_eq!(distance((sx, sy), (sx + r, y)), d);
                    if sx - r <= x && x <= sx + r {
                        x = sx + r + 1;
                        continue 'xloop;
                    }
                }
            }
            println!("possibility at {x},{y}");
            let h = (x, y);
            assert!(input
                .iter()
                .copied()
                .all(|(s, b)| distance(s, b) < distance(s, h)));
            return x * 4_000_000 + y;
        }
    }
    panic!("distress signal not found");
}

#[aoc(day15, part2, jorendorff)]
fn part_2(input: &Input) -> i64 {
    find_distress_signal(input, 4_000_000, 4_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_part_1() {
        assert_eq!(unbeaconable(&parse_input(EXAMPLE).unwrap(), 10), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            find_distress_signal(&parse_input(EXAMPLE).unwrap(), 20, 20),
            56000011
        );
    }
}
