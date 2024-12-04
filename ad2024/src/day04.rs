// Part 1 rank 759, part 2 rank 306.

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<char>>;

#[aoc_generator(day4, part1, jorendorff)]
#[aoc_generator(day4, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(alpha+));
    Ok(p.parse(text)?)
}

fn add(x: usize, d: isize) -> usize {
    x.wrapping_add(d as usize)
}

#[aoc(day4, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let mut count = 0;
    let h = input.len();
    let w = input[0].len();
    for r in 0..h {
        for c in 0..w {
            if input[r][c] == 'X' {
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr != 0 || dc != 0 {
                            let endr = r as isize + 3 * dr;
                            let endc = c as isize + 3 * dc;
                            if 0 <= endr
                                && (endr as usize) < h
                                && 0 <= endc
                                && (endc as usize) < w
                                && input[add(r, dr)][add(c, dc)] == 'M'
                                && input[add(r, 2 * dr)][add(c, 2 * dc)] == 'A'
                                && input[add(r, 3 * dr)][add(c, 3 * dc)] == 'S'
                            {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

#[aoc(day4, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let mut count = 0;
    let h = input.len();
    let w = input[0].len();
    for r in 1..h - 1 {
        for c in 1..w - 1 {
            if input[r][c] == 'A'
                && (input[r - 1][c - 1] == 'S' || input[r - 1][c - 1] == 'M')
                && (input[r - 1][c + 1] == 'S' || input[r - 1][c + 1] == 'M')
                && (input[r + 1][c - 1] == 'S' || input[r + 1][c - 1] == 'M')
                && (input[r + 1][c + 1] == 'S' || input[r + 1][c + 1] == 'M')
                && input[r - 1][c - 1] != input[r + 1][c + 1]
                && input[r - 1][c + 1] != input[r + 1][c - 1]
            {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 18);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 9);
    }
}
