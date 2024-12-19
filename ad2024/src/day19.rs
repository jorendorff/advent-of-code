// Part 2 rank 883.

use std::collections::*;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = (Vec<Vec<usize>>, Vec<Vec<usize>>);

#[aoc_generator(day19, part1, jorendorff)]
#[aoc_generator(day19, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(
        section(line(repeat_sep(char_of("wubrg")+, ", ")))
        section(lines(char_of("wubrg")+))
    );
    Ok(p.parse(text)?)
}

#[aoc(day19, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let mut towels: Vec<HashSet<Vec<usize>>> = vec![];
    for towel in &input.0 {
        while towels.len() < towel.len() + 1 {
            towels.push(HashSet::new());
        }
        towels[towel.len()].insert(towel.clone());
    }

    let mut count = 0;
    for pattern in &input.1 {
        let mut reachable = vec![false; pattern.len() + 1];
        reachable[0] = true;
        for i in 1..=pattern.len() {
            for j in i.saturating_sub(towels.len() - 1)..i {
                if reachable[j] && towels[i - j].contains(&pattern[j..i]) {
                    reachable[i] = true;
                    break;
                }
            }
        }
        if reachable[pattern.len()] {
            count += 1;
        }
    }
    count
}

#[aoc(day19, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let mut towels: Vec<HashSet<Vec<usize>>> = vec![];
    for towel in &input.0 {
        while towels.len() < towel.len() + 1 {
            towels.push(HashSet::new());
        }
        towels[towel.len()].insert(towel.clone());
    }

    let mut count = 0;
    for pattern in &input.1 {
        let mut reachable = vec![0; pattern.len() + 1];
        reachable[0] = 1;
        for i in 1..=pattern.len() {
            for j in i.saturating_sub(towels.len() - 1)..i {
                if reachable[j] != 0 && towels[i - j].contains(&pattern[j..i]) {
                    reachable[i] += reachable[j];
                }
            }
        }
        count += reachable[pattern.len()] ;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 16);
    }
}
