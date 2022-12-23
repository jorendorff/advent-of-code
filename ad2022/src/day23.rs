use std::collections::{HashMap, HashSet};

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<bool>>;

#[aoc_generator(day23, part1, jorendorff)]
#[aoc_generator(day23, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines({
        '.' => false,
        '#' => true,
    }+));
    Ok(p.parse(text)?)
}

type Point = (i64, i64);

fn elf_set(grid: &[Vec<bool>]) -> HashSet<Point> {
    let mut elves = HashSet::new();

    let width = grid[0].len();
    let height = grid.len();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    elves
}

const N: Point = (0, -1);
const NE: Point = (1, -1);
const E: Point = (1, 0);
const SE: Point = (1, 1);
const S: Point = (0, 1);
const SW: Point = (-1, 1);
const W: Point = (-1, 0);
const NW: Point = (-1, -1);

const ALL_DIRS: [Point; 8] = [N, NE, E, SE, S, SW, W, NW];
const DIRS: [[Point; 3]; 4] = [[N, NE, NW], [S, SE, SW], [W, NW, SW], [E, NE, SE]];

fn elf_round(elves: &HashSet<Point>, round: usize) -> HashSet<Point> {
    let elf_vec: Vec<(i64, i64)> = elves.iter().copied().collect();
    let proposals: Vec<(i64, i64)> = elf_vec
        .iter()
        .copied()
        .map(|(x, y)| {
            let test = |(dx, dy)| elves.contains(&(x + dx, y + dy));
            if ALL_DIRS.iter().copied().all(|d| !test(d)) {
                (x, y)
            } else {
                for d in round..round + 4 {
                    let dir_array = &DIRS[d % 4];
                    if dir_array.iter().copied().all(|d| !test(d)) {
                        let (dx, dy) = dir_array[0];
                        return (x + dx, y + dy);
                    }
                }
                (x, y)
            }
        })
        .collect();
    let mut proposal_counts = HashMap::new();
    for p in proposals.iter().copied() {
        *proposal_counts.entry(p).or_insert(0) += 1;
    }

    elf_vec
        .iter()
        .copied()
        .zip(proposals)
        .map(|(current, proposed)| {
            if proposal_counts.get(&proposed) == Some(&1) {
                proposed
            } else {
                current
            }
        })
        .collect()
}

#[aoc(day23, part1, jorendorff)]
fn part_1(input: &Input) -> i64 {
    // Rank #225 on the global leaderboard.
    let mut elves = elf_set(input);

    for round in 0..10 {
        elves = elf_round(&elves, round);
    }
    let x_min = elves.iter().map(|&(x, _y)| x).min().unwrap();
    let x_max = elves.iter().map(|&(x, _y)| x).max().unwrap();
    let y_min = elves.iter().map(|&(_x, y)| y).min().unwrap();
    let y_max = elves.iter().map(|&(_x, y)| y).max().unwrap();

    (y_max - y_min + 1) * (x_max - x_min + 1) - elves.len() as i64
}

#[aoc(day23, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    // Rank #195 on the global leaderboard.
    let mut elves = elf_set(input);

    for round in 0.. {
        let new_elf_positions = elf_round(&elves, round);
        if new_elf_positions == elves {
            return round + 1;
        }
        elves = new_elf_positions;
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "\
.....
..##.
..#..
.....
..##.
.....
";

    const EXAMPLE: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(SMALL_EXAMPLE).unwrap()), 25);
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 110);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 20);
    }
}
