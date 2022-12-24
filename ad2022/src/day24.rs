use std::collections::HashSet;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use Dir::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Sign {
    Wall,
    Clear,
    Blizz(Dir),
}
use Sign::*;

type Input = (Vec<Blizzard>, Vec<Vec<bool>>);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Blizzard {
    x: usize,
    y: usize,
    dir: Dir,
}

#[aoc_generator(day24, part1, jorendorff)]
#[aoc_generator(day24, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines({
        '#' => Wall,
        '.' => Clear,
        '^' => Blizz(Up),
        'v' => Blizz(Down),
        '>' => Blizz(Right),
        '<' => Blizz(Left),
    }+));
    let grid = p.parse(text)?;

    let blizzards: Vec<Blizzard> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .copied()
                .enumerate()
                .filter_map(move |(x, sign)| match sign {
                    Blizz(dir) => Some(Blizzard { x, y, dir }),
                    _ => None,
                })
        })
        .collect();

    let grid: Vec<Vec<bool>> = grid
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|sign| match sign {
                    Wall => false,
                    _ => true,
                })
                .collect::<Vec<bool>>()
        })
        .collect();

    Ok((blizzards, grid))
}

fn options(width: usize, height: usize, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut out = vec![(x, y)];

    if x > 0 {
        out.push((x - 1, y));
    }
    if y > 0 {
        out.push((x, y - 1));
    }
    if x + 1 < width {
        out.push((x + 1, y));
    }
    if y + 1 < height {
        out.push((x, y + 1));
    }
    out
}

fn move_blizzards(grid: &[Vec<bool>], blizzards: &mut [Blizzard]) {
    let height = grid.len();
    let width = grid[0].len();

    for b in blizzards {
        match b.dir {
            Up => {
                if b.y == 0 || !grid[b.y - 1][b.x] {
                    b.y = (0..height).rev().filter(|&y| grid[y][b.x]).next().unwrap();
                } else {
                    b.y -= 1;
                }
            }
            Down => {
                b.y += 1;
                if b.y > height || !grid[b.y][b.x] {
                    b.y = (0..height).filter(|&y| grid[y][b.x]).next().unwrap();
                }
            }
            Left => {
                b.x -= 1;
                if !grid[b.y][b.x] {
                    b.x = (0..width).rev().filter(|&x| grid[b.y][x]).next().unwrap();
                }
            }
            Right => {
                b.x += 1;
                if !grid[b.y][b.x] {
                    b.x = (0..width).filter(|&x| grid[b.y][x]).next().unwrap();
                }
            }
        }
    }
}

// determine possible points we can move to
fn explore(grid: &[Vec<bool>], start: (usize, usize), goal: (usize, usize), blizzards: &[Blizzard], points: HashSet<(usize, usize, usize)>) -> HashSet<(usize, usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();
    let blizzard_locs = blizzards
        .iter()
        .map(|b| (b.x, b.y))
        .collect::<HashSet<(usize, usize)>>();
    let mut after_points = HashSet::new();
    for (x, y, stage) in points {
        for p in options(width, height, (x, y)) {
            if grid[p.1][p.0] && !blizzard_locs.contains(&p) {
                let after_stage = if stage == 0 && p == goal {
                    1
                } else if stage == 1 && p == start {
                    2
                } else {
                    stage
                };
                after_points.insert((p.0, p.1, after_stage));
            }
        }
    }
    after_points
}

#[aoc(day24, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    // Rank 288 on the global leaderboard. There are lots of ways to cut
    // corners here. The problem statement didn't say that the puzzle input
    // would be a wide-open rectangle (though that does make valid puzzle inputs
    // much easier to create).
    let (blizzards, grid) = input;
    let height = grid.len();
    let width = grid[0].len();

    let mut blizzards = blizzards.clone();
    let x0 = (0..width).filter(|&x| grid[0][x]).next().unwrap();
    let start = (x0, 0);
    let xn = (0..width).filter(|&x| grid[height - 1][x]).next().unwrap();
    let goal = (xn, height - 1);

    let mut t = 0;
    let mut points = HashSet::new();
    points.insert((start.0, start.1, 0));
    while !points.contains(&(goal.0, goal.1, 1)) {
        move_blizzards(grid, &mut blizzards);
        points = explore(grid, start, goal, &blizzards, points);
        t += 1;
        assert!(!points.is_empty(), "no safe path");
    }

    t
}

#[aoc(day24, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    // Rank 282 on the global leaderboard.
    let (blizzards, grid) = input;
    let height = grid.len();
    let width = grid[0].len();

    let mut blizzards = blizzards.clone();
    let x0 = (0..width).filter(|&x| grid[0][x]).next().unwrap();
    let start = (x0, 0);
    let xn = (0..width).filter(|&x| grid[height - 1][x]).next().unwrap();
    let goal = (xn, height - 1);

    let mut t = 0;
    let mut points = HashSet::new();
    points.insert((start.0, start.1, 0));
    while !points.contains(&(goal.0, goal.1, 2)) {
        move_blizzards(grid, &mut blizzards);
        points = explore(grid, start, goal, &blizzards, points);
        t += 1;
        assert!(!points.is_empty(), "no safe path");
    }

    t
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 18);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 54);
    }
}
