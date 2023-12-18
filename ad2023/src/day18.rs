use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

use std::collections::*;
use adlib::*;

type Input = Vec<(Dir, usize, u32)>;

#[aoc_generator(day18, part1, jorendorff)]
#[aoc_generator(day18, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        d:{'R' => Right, 'L' => Left, 'U' => Up, 'D' => Down} " " n:usize " (#" c:u32_hex ")" => (d, n, c)
    ));
    Ok(p.parse(text)?)
}

#[aoc(day18, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    // #276 on the global leaderboard
    let mut visited = HashSet::new();

    let mut edges = HashSet::new();

    let mut p = Point { row: 500, col: 500 };
    visited.insert(p);
    for &(d, n, _c) in input {
        for _ in 0..n {
            if d == Down {
                edges.insert(p);
            }
            p += d;
            visited.insert(p);
            if d == Up {
                edges.insert(p);
            }
        }
    }

    let mut count = visited.len();
    for r in 0..1000 {
        let mut inside = false;
        for c in 0..1000 {
            let q = Point { row: r, col: c};
            if edges.contains(&q) {
                inside = !inside;
            } else if inside && !visited.contains(&q) {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day18, part2, jorendorff)]
fn part_2(input: &Input) -> i128 {
    // #220 on the global leaderboard
    let mut area = 0i128;

    let mut visited = 1;

    let mut x = 0i128;
    let mut y = 0i128;

    let mut prev_dir = 50;
    for (_d, _n, c) in input {
        let n = (c >> 4) as i128;
        let d = c & 15;
        assert_ne!(d, prev_dir);
        prev_dir = d;
        match d {
            0 => {
                x += n;
                visited += n;
            }
            1 => {
                y += n;
                visited += n;
                area += (i64::MAX as i128 - x) * n;
                //area2 += (i64::MAX as i128 - x) * n;
            }
            2 => {
                x -= n;
            }
            3 => {
                y -= n;
                area -= (i64::MAX as i128 - x) * n;
                //area2 -= (i64::MAX as i128 - x + 1) * n;
            }
            _ => panic!(),
        };
    }

    println!("{x} {y}");

    area.abs() + visited
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 62);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 952408144115);
    }
}
