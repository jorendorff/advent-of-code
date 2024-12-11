// Part 1 rank 724, part 2 rank 614.

use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<u64>;

#[aoc_generator(day11, part1, jorendorff)]
#[aoc_generator(day11, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(line(repeat_sep(u64, ' ')));
    Ok(p.parse(text)?)
}

fn blink(v: Vec<u64>) -> Vec<u64> {
    v.into_iter()
        .flat_map(|n| {
            if n == 0 {
                return vec![1];
            }
            let s = n.to_string();
            if s.len() % 2 == 0 {
                let h = s.len() / 2;
                vec![s[..h].parse().unwrap(), s[h..].parse().unwrap()]
            } else {
                vec![n * 2024]
            }
        })
        .collect()
}

#[aoc(day11, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let mut v = input.clone();
    for _ in 0..25 {
        v = blink(v);
    }
    v.len()
}

fn how_many_after(k: u64, blinks: usize, cache: &mut HashMap<(u64, usize), u64>) -> u64 {
    if blinks == 0 {
        1
    } else if let Some(&v) = cache.get(&(k, blinks)) {
        v
    } else {
        let answer = if k == 0 {
            how_many_after(1, blinks - 1, cache)
        } else {
            let s = k.to_string();
            if s.len() % 2 == 0 {
                let h = s.len() / 2;
                how_many_after(s[..h].parse().unwrap(), blinks - 1, cache)
                    + how_many_after(s[h..].parse().unwrap(), blinks - 1, cache)
            } else {
                how_many_after(k * 2024, blinks - 1, cache)
            }
        };

        cache.insert((k, blinks), answer);
        answer
    }
}

#[aoc(day11, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    let mut cache = HashMap::new();
    input
        .iter()
        .copied()
        .map(|k| how_many_after(k, 75, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
125 17
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 55312);
    }

    #[test]
    fn test_part_2() {
        let mut cache = HashMap::new();
        assert_eq!(
            how_many_after(125, 25, &mut cache) + how_many_after(17, 25, &mut cache),
            55312
        );
    }
}
