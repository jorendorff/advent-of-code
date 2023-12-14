use std::collections::HashMap;
use std::collections::hash_map::Entry;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<usize>>;

#[aoc_generator(day14, part1, jorendorff)]
#[aoc_generator(day14, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        char_of(".O#")+
    ));
    Ok(p.parse(text)?)
}

fn total_load_north(input: &Input) -> usize {
    let h = input.len();
    input.iter().enumerate().map(|(r, row)| {
        row.iter().copied().filter(|c| *c == 1).count()  * (h - r)
    }).sum()
}

fn tilt_north(input: &mut Input) {
    let w = input[0].len();
    let h = input.len();
    for c in 0..w {
        loop {
            let mut more = false;
            for r in 0..(h - 1) {
                if input[r][c] == 0 && input[r+1][c] == 1 {
                    input[r][c] = 1;
                    input[r+1][c] = 0;
                    more = true;
                }
            }
            if !more {
                break;
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
        loop {
            let mut more = false;
            for c in 0..(w - 1) {
                if row[c] == 0 && row[c + 1] == 1 {
                    row[c] = 1;
                    row[c+1] = 0;
                    more = true;
                }
            }
            if !more { break; }
        }
    }

    //south
    for c in 0..w {
        loop {
            let mut more = false;
            for r in 0..(h - 1) {
                if input[r][c] == 1 && input[r+1][c] == 0 {
                    input[r][c] = 0;
                    input[r+1][c] = 1;
                    more = true;
                }
            }
            if !more {
                break;
            }
        }
    }

    //east
    for row in input.iter_mut() {
        loop {
            let mut more = false;
            for c in 0..(w - 1) {
                if row[c] == 1 && row[c + 1] == 0 {
                    row[c] = 0;
                    row[c+1] = 1;
                    more = true;
                }
            }
            if !more { break; }
        }
    }
}

#[aoc(day14, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    // #67 on the global leaderboard

    let mut cache: HashMap<Vec<Vec<usize>>, usize> = HashMap::new();

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
