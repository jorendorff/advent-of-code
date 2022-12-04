use std::ops::Range;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

#[aoc_generator(day4, part1, jorendorff)]
#[aoc_generator(day4, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<(Range<u64>, Range<u64>)>> {
    let range = parser!((a:u64) "-" (b:u64) => a .. (b + 1));
    let p = parser!(lines(range "," range));
    aoc_parse(text, p)
}

fn contains(a: &Range<u64>, b: &Range<u64>) -> bool {
    a.start <= b.start && a.end >= b.end
}

fn overlap(a: &Range<u64>, b: &Range<u64>) -> bool {
    a.start < b.end && b.start < a.end
}

#[aoc(day4, part1, jorendorff)]
fn part_1(lists: &[(Range<u64>, Range<u64>)]) -> usize {
    lists
        .iter()
        .filter(|pair| contains(&pair.0, &pair.1) || contains(&pair.1, &pair.0))
        .count()
}

#[aoc(day4, part2, jorendorff)]
fn part_2(lists: &[(Range<u64>, Range<u64>)]) -> usize {
    lists
        .iter()
        .filter(|pair| overlap(&pair.0, &pair.1))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 4);
    }
}
