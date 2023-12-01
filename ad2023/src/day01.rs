use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

#[aoc_generator(day1, part1, jorendorff)]
fn parse_input_1(text: &str) -> anyhow::Result<Vec<usize>> {
    let p = parser!(lines({
        alpha* x:digit any_char* y:digit any_char* => 10 * x + y,
        alpha* x:digit any_char* => 11 * x,
    }));
    Ok(p.parse(text)?)
}

#[aoc_generator(day1, part2, jorendorff)]
fn parse_input_2(text: &str) -> anyhow::Result<Vec<String>> {
    let p = parser!(lines(string(any_char*)));
    Ok(p.parse(text)?)
}

#[aoc(day1, part1, jorendorff)]
fn part_1(nums: &[usize]) -> usize {
    nums.iter().copied().sum()
}

static SPELLED_DIGITS: &[(&str, u64)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_digit(line: &str, places: impl Iterator<Item=(usize, char)>) -> u64 {
    for (i, c) in places {
        if let Some(d) = c.to_digit(10) {
            return d.into();
        }
        for (word, value) in SPELLED_DIGITS.iter().copied() {
            if line[i..].starts_with(word) {
                return value;
            }
        }
    }
    panic!("no digits found");
}

fn first_digit(line: &str) -> u64 {
    find_digit(line, line.char_indices())
}

fn last_digit(line: &str) -> u64 {
    find_digit(line, line.char_indices().rev())
}

#[aoc(day1, part2, jorendorff)]
fn part_2(lines: &[String]) -> u64 {
    lines
        .iter()
        .map(|s| 10 * first_digit(s) + last_digit(s))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input_1(EXAMPLE_1).unwrap()), 142);
    }

    const EXAMPLE_2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input_2(EXAMPLE_2).unwrap()), 281);
    }
}
