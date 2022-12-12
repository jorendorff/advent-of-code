use std::collections::{HashMap, HashSet, VecDeque};

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<char>>;

#[aoc_generator(day12, part1, jorendorff)]
#[aoc_generator(day12, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(alpha+));
    aoc_parse(text, p)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2(i32, i32);

const DIRS: [Vec2; 4] = [Vec2(1, 0), Vec2(0, 1), Vec2(-1, 0), Vec2(0, -1)];

#[aoc(day12, part1, jorendorff)]
fn part_1(map: &Input) -> usize {
    let mut map = map.clone();
    let nrows = map.len();
    let ncols = map[0].len();

    let mut origin = Point(-1, -1);
    let mut target = Point(-1, -1);
    for r in 0..nrows {
        for c in 0..ncols {
            if map[r][c] == 'S' {
                origin = Point(r as i32, c as i32);
                map[r][c] = 'a';
            } else if map[r][c] == 'E' {
                target = Point(r as i32, c as i32);
                map[r][c] = 'z';
            }
        }
    }
    assert!(origin.0 >= 0);
    assert!(target.0 >= 0);

    let mut edges: HashMap<Point, Vec<Point>> = HashMap::new();
    for r in 0..nrows {
        for c in 0..ncols {
            for d in DIRS {
                let p0 = Point(r as i32, c as i32);
                let p1 = Point(r as i32 + d.0, c as i32 + d.1);
                if 0 <= p1.0 && p1.0 < nrows as i32 && 0 <= p1.1 && p1.1 < ncols as i32 {
                    if map[p1.0 as usize][p1.1 as usize] as u32 <= map[r][c] as u32 + 1 {
                        edges.entry(p0).or_default().push(p1);
                    }
                }
            }
        }
    }

    pathfind(nrows, ncols, &edges, origin, target).expect("no route to target square!")
}

fn pathfind(
    nrows: usize,
    ncols: usize,
    edges: &HashMap<Point, Vec<Point>>,
    origin: Point,
    target: Point,
) -> Option<usize> {
    let mut todo = VecDeque::from([(origin, 0)]);
    let mut seen: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];
    while let Some((p, steps)) = todo.pop_front() {
        if let Some(neighbors) = edges.get(&p) {
            for q in neighbors {
                if !seen[q.0 as usize][q.1 as usize] {
                    seen[q.0 as usize][q.1 as usize] = true;
                    todo.push_back((*q, steps + 1));
                    if *q == target {
                        return Some(steps + 1);
                    }
                }
            }
        }
    }
    None
}

#[aoc(day12, part2, jorendorff)]
fn part_2(map: &Input) -> usize {
    let mut map = map.clone();
    let nrows = map.len();
    let ncols = map[0].len();

    let mut origin = Point(-1, -1);
    let mut target = Point(-1, -1);
    for r in 0..nrows {
        for c in 0..ncols {
            if map[r][c] == 'S' {
                origin = Point(r as i32, c as i32);
                map[r][c] = 'a';
            } else if map[r][c] == 'E' {
                target = Point(r as i32, c as i32);
                map[r][c] = 'z';
            }
        }
    }
    assert!(origin.0 >= 0);
    assert!(target.0 >= 0);

    let mut edges: HashMap<Point, Vec<Point>> = HashMap::new();
    for r in 0..nrows {
        for c in 0..ncols {
            for d in DIRS {
                let p0 = Point(r as i32, c as i32);
                let p1 = Point(r as i32 + d.0, c as i32 + d.1);
                if 0 <= p1.0 && p1.0 < nrows as i32 && 0 <= p1.1 && p1.1 < ncols as i32 {
                    if map[p1.0 as usize][p1.1 as usize] as u32 <= map[r][c] as u32 + 1 {
                        edges.entry(p0).or_default().push(p1);
                    }
                }
            }
        }
    }

    let rmap = &map;
    let redges = &edges;
    (0..nrows)
        .flat_map(move |r| {
            (0..ncols).filter_map(move |c| {
                if rmap[r][c] == 'a' {
                    pathfind(nrows, ncols, redges, Point(r as i32, c as i32), target)
                } else {
                    None
                }
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 31);
    }

    #[test]
    fn test_part_2() {
        //assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 0);
    }
}
