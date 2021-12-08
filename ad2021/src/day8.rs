use std::str::FromStr;

use aoc_runner_derive::*;

#[derive(Copy, Clone)]
struct Pattern(u8);

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = 0;
        for c in s.chars() {
            bits |= 1
                << match c {
                    'a'..='g' => (c as u32) - ('a' as u32),
                    _ => anyhow::bail!("unrecognized character {:?} in pattern {:?}", c, s),
                };
        }
        Ok(Self(bits))
    }
}

const GOOD_PATTERN_STRS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

impl Pattern {
    fn read(self) -> u64 {
        match self.0 {
            0b1110111 => 0,
            0b0100100 => 1,
            0b1011101 => 2,
            0b1101101 => 3,
            0b0101110 => 4,

            0b1101011 => 5,
            0b1111011 => 6,
            0b0100101 => 7,
            0b1111111 => 8,
            0b1101111 => 9,
            _ => panic!("can't read digit: Pattern({:b})", self.0),
        }
    }
}

#[test]
fn test_patterns() {
    for (i, s) in GOOD_PATTERN_STRS.iter().enumerate() {
        assert_eq!(Pattern::from_str(s).unwrap().read(), i as u64);
    }
}

struct Entry {
    patterns: [Pattern; 10],
    output_value: [Pattern; 4],
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [patterns, output_value]: [&str; 2] = s
            .split('|')
            .collect::<Vec<&str>>()
            .try_into()
            .map_err(|_| anyhow::anyhow!("expected two fields separated by |, got {:?}", s))?;
        let patterns: [Pattern; 10] = patterns
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<Pattern>())
            .collect::<anyhow::Result<Vec<Pattern>>>()?
            .try_into()
            .map_err(|_| anyhow::anyhow!("expected 10 patterns, got {:?}", s))?;
        let output_value: [Pattern; 4] = output_value
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<Pattern>())
            .collect::<anyhow::Result<Vec<Pattern>>>()?
            .try_into()
            .map_err(|_| anyhow::anyhow!("expected 4-digit output value, got {:?}", s))?;
        Ok(Entry {
            patterns,
            output_value,
        })
    }
}

#[aoc_generator(day8)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Entry>> {
    text.lines().map(|line| line.parse::<Entry>()).collect()
}

#[aoc(day8, part1)]
fn part_1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .flat_map(|e| e.output_value)
        .filter(|p| matches!(p.0.count_ones(), 2 | 3 | 4 | 7))
        .count()
}

fn bitset(patterns: &[Pattern]) -> u128 {
    patterns.iter().map(|p| 1u128 << p.0).sum()
}

fn decode(entry: &Entry, good_patterns: u128) -> anyhow::Result<u64> {
    // Brute force. 7^7 is 823,543. (10^10 would be too much.)
    for (a, b, c, d, e, f, g) in itertools::iproduct!(0..7, 0..7, 0..7, 0..7, 0..7, 0..7, 0..7) {
        if (1 << a) | (1 << b) | (1 << c) | (1 << d) | (1 << e) | (1 << f) | (1 << g) == 0b0111_1111
        {
            // All values distinct makes a permutation.
            let permute = move |p: Pattern| -> Pattern {
                Pattern(
                    (0..7)
                        .zip([a, b, c, d, e, f, g])
                        .map(|(i, j)| if p.0 & (1 << i) != 0 { 1 << j } else { 0 })
                        .sum(),
                )
            };
            if bitset(&entry.patterns.map(permute)) == good_patterns {
                return Ok(entry
                    .output_value
                    .iter()
                    .map(|p| permute(*p).read())
                    .fold(0, |acc, digit| acc * 10 + digit));
            }
        }
    }
    anyhow::bail!("no solution found");
}

#[aoc(day8, part2)]
fn part_2(entries: &[Entry]) -> anyhow::Result<u64> {
    let good_patterns = GOOD_PATTERN_STRS
        .iter()
        .map(|s| Pattern::from_str(*s))
        .collect::<anyhow::Result<Vec<Pattern>>>()?;
    let good_patterns = bitset(&good_patterns);

    Ok(entries
        .iter()
        .map(|e| decode(e, good_patterns))
        .collect::<anyhow::Result<Vec<u64>>>()?
        .into_iter()
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()).unwrap(), 61229);
    }
}
