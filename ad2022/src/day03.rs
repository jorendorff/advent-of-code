use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => 1 + (item as u32 - 'a' as u32),
        'A'..='Z' => 27 + (item as u32 - 'A' as u32),
        _ => panic!("invalid item {item:?}"),
    }
}

fn items(s: &[char]) -> u64 {
    s.iter()
        .copied()
        .map(priority)
        .map(|p| 1u64 << p)
        .fold(0, |a, b| (a | b))
}

#[aoc_generator(day3, part1, jorendorff)]
#[aoc_generator(day3, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<(u64, u64)>> {
    aoc_parse(
        text,
        parser!((
            (line_str: line()) => {
                let ch = line_str.chars().collect::<Vec<char>>();
                let n = ch.len();
                assert_eq!(n % 2, 0, "line {line_str:?} has an odd number of characters");
                (items(&ch[..n / 2]), items(&ch[n / 2..]))
            }
        )*),
    )
}

#[aoc(day3, part1, jorendorff)]
fn part_1(sacks: &[(u64, u64)]) -> u64 {
    sacks
        .iter()
        .map(|&(c1, c2)| {
            let shared = c1 & c2;
            assert_eq!(shared.count_ones(), 1);
            shared.trailing_zeros() as u64
        })
        .sum()
}

#[aoc(day3, part2, jorendorff)]
fn part_2(sacks: &[(u64, u64)]) -> u64 {
    let sacks = sacks
        .iter()
        .map(|&(c1, c2)| (c1 | c2))
        .collect::<Vec<u64>>();

    sacks
        .chunks_exact(3)
        .map(|group| {
            let shared = group[0] & group[1] & group[2];
            assert_eq!(shared.count_ones(), 1);
            shared.trailing_zeros() as u64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 70);
    }
}
