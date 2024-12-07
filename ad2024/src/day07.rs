// Part 2 rank 913

use std::collections::HashSet;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<(u64, Vec<u64>)>;

#[aoc_generator(day7, part1, jorendorff)]
#[aoc_generator(day7, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(u64 ": " repeat_sep(u64, ' ')));
    Ok(p.parse(text)?)
}

fn is_possible(is_part_2: bool, result: u64, values: &[u64]) -> bool {
    let mut hits: HashSet<u64> = [values[0]].into_iter().collect::<HashSet<u64>>();
    for &v in &values[1..] {
        let next = hits
            .into_iter()
            .flat_map(|x| {
                let mut answers = vec![];
                if x + v <= result {
                    answers.push(x + v);
                }
                if x * v <= result {
                    answers.push(x * v);
                }
                if let Ok(xv) = format!("{x}{v}").parse::<u64>() {
                    if is_part_2 && xv <= result {
                        answers.push(xv);
                    }
                }
                answers
            })
            .collect();
        hits = next;
    }
    hits.contains(&result)
}

#[aoc(day7, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    input
        .iter()
        .filter(|pair| is_possible(false, pair.0, &pair.1))
        .map(|pair| pair.0)
        .sum()
}

#[aoc(day7, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    input
        .iter()
        .filter(|pair| is_possible(true, pair.0, &pair.1))
        .map(|pair| pair.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 3749);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 11387);
    }
}
