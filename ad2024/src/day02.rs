use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

use std::cmp::Reverse;

#[aoc_generator(day2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Vec<u64>>> {
    let p = parser!(lines(repeat_sep(u64, ' '+)));
    Ok(p.parse(text)?)
}

fn is_safe(line: &[u64]) -> bool {
    (line.is_sorted() || line.is_sorted_by_key(Reverse))
        && line.windows(2).all(|pair| (1..=3).contains(&pair[0].abs_diff(pair[1])))
}

#[aoc(day2, part1, jorendorff)]
fn part_1(grid: &Vec<Vec<u64>>) -> usize {
    grid.iter().filter(|&line| is_safe(line)).count()
}

#[aoc(day2, part2, jorendorff)]
fn part_2(grid: &Vec<Vec<u64>>) -> usize {
    fn is_index_safe(line: &[u64], i: usize) -> bool {
        is_safe(
            &line[..i]
                .iter()
                .copied()
                .chain(line[i + 1..].iter().copied())
                .collect::<Vec<u64>>()
        )
    }

    grid.iter().filter(|&line| (0..line.len()).any(|i| is_index_safe(line, i))).count()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE_1).unwrap()), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE_1).unwrap()), 4);
    }
}
