// Part 1 rank 852, part 2 rank 696.

use std::collections::HashSet;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

struct Input {
    rules: HashSet<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

#[aoc_generator(day5, part1, jorendorff)]
#[aoc_generator(day5, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(
        rules:section(hash_set(lines(usize '|' usize)))
        updates:section(lines(repeat_sep(usize, ',')))
            => Input { rules, updates }
    );
    Ok(p.parse(text)?)
}

#[aoc(day5, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    input.updates
        .iter()
        .filter(|upd| {
            for (i, vi) in upd.iter().copied().enumerate() {
                for (j, vj) in upd.iter().copied().enumerate() {
                    if i < j && input.rules.contains(&(vj, vi)) {
                        return false;
                    }
                }
            }
            true
        })
        .map(|upd| upd[upd.len()/2])
        .sum()
}

#[aoc(day5, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    input.updates
        .iter()
        .cloned()
        .filter(|upd| {
            for (i, vi) in upd.iter().copied().enumerate() {
                for (j, vj) in upd.iter().copied().enumerate() {
                    if i < j && input.rules.contains(&(vj, vi)) {
                        return true;
                    }
                }
            }
            false
        })
        .map(|mut upd| {
            let mut done = false;
            while !done {
                done = true;
                for i in 0..upd.len() {
                    let vi = upd[i];
                    for j in 0..upd.len() {
                        let vj = upd[j];
                        if i < j && input.rules.contains(&(vj, vi)) {
                            upd[i..j+1].rotate_right(1);
                            done = false;
                            break;
                        }
                    }
                }
            }
            upd[upd.len()/2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 143);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 123);
    }
}
