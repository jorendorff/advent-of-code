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

#[aoc(day23, part1, jorendorff)]
fn part_1(input: &Input) -> i64 {
    // 225
    let mut elves = HashSet::new();

    let width = input[0].len();
    let height = input.len();
    for y in 0..height {
        for x in 0..width {
            if input[y][x] {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    const N: (i64, i64) = (0, -1);
    const NE: (i64, i64) = (1, -1);
    const E: (i64, i64) = (1, 0);
    const SE: (i64, i64) = (1, 1);
    const S: (i64, i64) = (0, 1);
    const SW: (i64, i64) = (-1, 1);
    const W: (i64, i64) = (-1, 0);
    const NW: (i64, i64) = (-1, -1);

    let all_dirs = [N, NE, E, SE, S, SW, W, NW];
    let dirs = [[N, NE, NW], [S, SE, SW], [W, NW, SW], [E, NE, SE]];

    for round in 0..10 {
        let elf_vec: Vec<(i64, i64)> = elves.iter().copied().collect();
        let proposals: Vec<(i64, i64)> = elf_vec
            .iter()
            .copied()
            .map(|(x, y)| {
                let test = |(dx, dy)| elves.contains(&(x + dx, y + dy));
                if all_dirs.iter().copied().all(|d| !test(d)) {
                    (x, y)
                } else {
                    for d in round..round + 4 {
                        let dir_array = &dirs[d % 4];
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

        elves = elf_vec
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
            .collect();
    }
    let x_min = elves.iter().map(|&(x, _y)| x).min().unwrap();
    let x_max = elves.iter().map(|&(x, _y)| x).max().unwrap();
    let y_min = elves.iter().map(|&(_x, y)| y).min().unwrap();
    let y_max = elves.iter().map(|&(_x, y)| y).max().unwrap();

    (y_max - y_min + 1) * (x_max - x_min + 1) - elves.len() as i64
}

#[aoc(day23, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    // 195
    let mut elves = HashSet::new();

    let width = input[0].len();
    let height = input.len();
    for y in 0..height {
        for x in 0..width {
            if input[y][x] {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    const N: (i64, i64) = (0, -1);
    const NE: (i64, i64) = (1, -1);
    const E: (i64, i64) = (1, 0);
    const SE: (i64, i64) = (1, 1);
    const S: (i64, i64) = (0, 1);
    const SW: (i64, i64) = (-1, 1);
    const W: (i64, i64) = (-1, 0);
    const NW: (i64, i64) = (-1, -1);

    let all_dirs = [N, NE, E, SE, S, SW, W, NW];
    let dirs = [[N, NE, NW], [S, SE, SW], [W, NW, SW], [E, NE, SE]];

    for round in 0.. {
        let elf_vec: Vec<(i64, i64)> = elves.iter().copied().collect();
        let proposals: Vec<(i64, i64)> = elf_vec
            .iter()
            .copied()
            .map(|(x, y)| {
                let test = |(dx, dy)| elves.contains(&(x + dx, y + dy));
                if all_dirs.iter().copied().all(|d| !test(d)) {
                    (x, y)
                } else {
                    for d in round..round + 4 {
                        let dir_array = &dirs[d % 4];
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

        let mut moved = false;
        
        elves = elf_vec
            .iter()
            .copied()
            .zip(proposals)
            .map(|(current, proposed)| {
                if proposal_counts.get(&proposed) == Some(&1) {
                    if proposed != current {
                        moved = true;
                    }
                    proposed
                } else {
                    current
                }
            })
            .collect();

        if !moved {
            return round + 1;
        }
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
