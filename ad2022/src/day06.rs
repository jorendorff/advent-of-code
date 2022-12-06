use std::collections::HashSet;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<char>;

#[aoc_generator(day6, part1, jorendorff)]
#[aoc_generator(day6, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(line(alpha+));
    aoc_parse(text, p)
}

#[aoc(day6, part1, jorendorff)]
fn part_1(signal: &[char]) -> usize {
    for (i, d) in signal.iter().copied().enumerate() {
        if i >= 3 {
            let a = signal[i - 3];
            let b = signal[i - 2];
            let c = signal[i - 1];
            if a != b && a != c && a != d && b != c && b != d && c != d {
                return i + 1;
            }
        }
    }
    panic!("no start-of-packet marker detected");
}

#[aoc(day6, part2, jorendorff)]
fn part_2(signal: &[char]) -> usize {
    for i in 1..=signal.len() {
        if i >= 14
            && signal[i - 14..i]
                .iter()
                .copied()
                .collect::<HashSet<char>>()
                .len()
                == 14
        {
            return i;
        }
    }
    panic!("no start-of-message marker detected");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p1(s: &str) -> usize {
        part_1(&parse_input(s).unwrap())
    }

    #[test]
    fn test_part_1() {
        assert_eq!(p1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(p1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(p1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(p1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    fn p2(s: &str) -> usize {
        part_2(&parse_input(s).unwrap())
    }

    #[test]
    fn test_part_2() {
        assert_eq!(p2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(p2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(p2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(p2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(p2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
