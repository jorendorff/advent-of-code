use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

#[aoc_generator(day1, part1, jorendorff)]
#[aoc_generator(day1, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Vec<u64>>> {
    let p = parser!(sections(lines(u64)));
    aoc_parse(text, p)
}

#[aoc(day1, part1, jorendorff)]
fn part_1(lists: &[Vec<u64>]) -> u64 {
    lists
        .iter()
        .map(|list| list.iter().sum::<u64>())
        .max()
        .unwrap()
}

#[aoc(day1, part2, jorendorff)]
fn part_2(lists: &[Vec<u64>]) -> u64 {
    let mut totals = lists
        .iter()
        .map(|list| list.iter().sum::<u64>())
        .collect::<Vec<u64>>();
    totals.sort();
    totals[totals.len() - 3..totals.len()].iter().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 24000);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 45000);
    }
}
