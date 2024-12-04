use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

#[aoc_generator(day1, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<(Vec<u64>, Vec<u64>)> {
    let p = parser!(lines({
        x:u64 ' '+ y:u64 => (x, y)
    }));
    Ok(p.parse(text)?.into_iter().unzip())
}

#[aoc(day1, part1, jorendorff)]
fn part_1((list1, list2): &(Vec<u64>, Vec<u64>)) -> u64 {
    let mut list1 = list1.clone();
    let mut list2 = list2.clone();
    list1.sort_unstable();
    list2.sort_unstable();
    list1.into_iter().zip(list2).map(|(a, b)| a.abs_diff(b)).sum()
}

#[aoc(day1, part2, jorendorff)]
fn part_2((list1, list2): &(Vec<u64>, Vec<u64>)) -> u64 {
    use std::collections::HashMap;

    let mut counts: HashMap<u64, u64> = HashMap::new();

    for &v in list2 {
        *counts.entry(v).or_default() += 1;
    }

    list1.iter().copied()
        .map(|x| x * counts.get(&x).copied().unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE_1).unwrap()), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE_1).unwrap()), 31);
    }
}
