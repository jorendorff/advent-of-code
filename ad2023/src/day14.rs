use std::collections::hash_map::Entry;
use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Cell = u8;
type Input = Vec<Vec<Cell>>;

const SPACE: Cell = 0;
const ROCK: Cell = 1;

#[aoc_generator(day14, part1, jorendorff)]
#[aoc_generator(day14, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        (x:char_of(".O#") => x as Cell)+
    ));
    Ok(p.parse(text)?)
}

fn total_load_north(input: &Input) -> usize {
    let h = input.len();
    input
        .iter()
        .enumerate()
        .map(|(r, row)| row.iter().copied().filter(|c| *c == ROCK).count() * (h - r))
        .sum()
}

fn tilt_north(input: &mut Input) {
    let w = input[0].len();
    let h = input.len();
    for c in 0..w {
        let mut out = 0;
        for r in 0..h {
            match input[r][c] {
                SPACE => {}
                ROCK => {
                    if out != r {
                        input[out][c] = ROCK;
                        input[r][c] = SPACE;
                    }
                    out += 1;
                }
                _ => out = r + 1,
            }
        }
    }
}

#[aoc(day14, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    // #207 on the global leaderboard
    let mut input = input.clone();
    tilt_north(&mut input);
    total_load_north(&input)
}

fn one_cycle(input: &mut Input) {
    let w = input[0].len();
    let h = input.len();

    tilt_north(input);

    //west
    for row in input.iter_mut() {
        let mut out = 0;
        for c in 0..w {
            match row[c] {
                SPACE => {}
                ROCK => {
                    if out != c {
                        row[out] = ROCK;
                        row[c] = SPACE;
                    }
                    out += 1;
                }
                _ => out = c + 1,
            }
        }
    }

    //south
    for c in 0..w {
        let mut out = h - 1;
        for r in (0..h).rev() {
            match input[r][c] {
                SPACE => {}
                ROCK => {
                    if out != r {
                        input[out][c] = ROCK;
                        input[r][c] = SPACE;
                    }
                    out = out.wrapping_sub(1);
                }
                _ => out = r.wrapping_sub(1),
            }
        }
    }

    //east
    for row in input.iter_mut() {
        let mut out = w - 1;
        for c in (0..w).rev() {
            match row[c] {
                SPACE => {}
                ROCK => {
                    if out != c {
                        row[out] = ROCK;
                        row[c] = SPACE;
                    }
                    out = out.wrapping_sub(1);
                }
                _ => out = c.wrapping_sub(1),
            }
        }
    }
}

#[aoc(day14, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    // #67 on the global leaderboard

    let mut cache: HashMap<Input, usize> = HashMap::new();

    let n = 1000000000;

    let mut t = 0;
    let mut grid = input.clone();
    cache.insert(grid.clone(), t);
    loop {
        one_cycle(&mut grid);
        t += 1;
        match cache.entry(grid.clone()) {
            Entry::Occupied(e) => {
                let u = *e.get();
                let clen = t - u;
                t += (n - t) / clen * clen;
                break;
            }
            Entry::Vacant(e) => {
                e.insert(t);
            }
        }
    }

    for _ in t..n {
        one_cycle(&mut grid);
    }

    total_load_north(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 136);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 64);
    }
}
