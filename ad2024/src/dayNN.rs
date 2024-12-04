use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<u64>;

#[aoc_generator(dayNN, part1, jorendorff)]
#[aoc_generator(dayNN, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(u64));
    Ok(p.parse(text)?)
}

#[aoc(dayNN, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    0
}

#[aoc(dayNN, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 0);
    }
}
