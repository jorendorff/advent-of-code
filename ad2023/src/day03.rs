use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

struct PartNumber {
    row: usize,
    col: usize,
    len: usize,
    value: u64,
}

struct Engine {
    numbers: Vec<PartNumber>,
    symbols: Vec<(usize, usize, char)>,
}

#[aoc_generator(day3, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Engine> {
    let p = parser!(lines(any_char+));
    let input = p.parse(text)?;

    let mut numbers: Vec<PartNumber> = vec![];
    let mut symbols: Vec<(usize, usize, char)> = vec![];
    for (r, line) in input.iter().enumerate() {
        let mut num: Option<PartNumber> = None;

        for (c, chr) in line.iter().copied().enumerate() {
            if let Some(d) = chr.to_digit(10) {
                let n = num.get_or_insert(PartNumber {
                    row: r,
                    col: c,
                    len: 0,
                    value: 0,
                });
                n.len += 1;
                n.value = 10 * n.value + d as u64;
            } else {
                if let Some(record) = num {
                    numbers.push(record);
                    num = None;
                }
                if chr != '.' {
                    symbols.push((r, c, chr));
                }
            }
        }
        if let Some(record) = num {
            numbers.push(record);
        }
    }

    Ok(Engine { numbers, symbols })
}

impl PartNumber {
    fn is_adjacent(&self, r: usize, c: usize) -> bool {
        self.row <= r + 1 && r <= self.row + 1 && self.col <= c + 1 && c <= self.col + self.len
    }
}

#[aoc(day3, part1, jorendorff)]
fn part_1(engine: &Engine) -> u64 {
    engine
        .numbers
        .iter()
        .filter(|num| {
            engine
                .symbols
                .iter()
                .copied()
                .any(|(r, c, _)| num.is_adjacent(r, c))
        })
        .map(|num| num.value)
        .sum()
}

#[aoc(day3, part2, jorendorff)]
fn part_2(engine: &Engine) -> u64 {
    engine
        .symbols
        .iter()
        .copied()
        .filter_map(|(r, c, chr)| -> Option<u64> {
            if chr != '*' {
                return None;
            }
            let nums = engine
                .numbers
                .iter()
                .filter(|num| num.is_adjacent(r, c))
                .collect::<Vec<_>>();
            if nums.len() != 2 {
                return None;
            }
            Some(nums[0].value * nums[1].value)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 4361);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 467835);
    }
}
