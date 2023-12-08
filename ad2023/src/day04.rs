use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

struct Card {
    winning: Vec<u32>,
    nums: Vec<u32>,
}

impl Card {
    fn hits(&self) -> u64 {
        self.nums
            .iter()
            .copied()
            .filter(|n| self.winning.contains(n))
            .count() as u64
    }
}

#[aoc_generator(day4, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Card>> {
    let p = parser!(
        lines(
            "Card" ' '+ usize ":" " "+
                winning:repeat_sep(u32, " "+)
                " "+ "|" " "+
                nums:repeat_sep(u32, " "+)
                => Card { winning, nums }
        )
    );
    Ok(p.parse(text)?)
}

#[aoc(day4, part1, jorendorff)]
fn part_1(input: &[Card]) -> u64 {
    input
        .iter()
        .map(|card| {
            let num_hits = card.hits();
            if num_hits == 0 {
                0
            } else {
                2_u64.pow(num_hits as u32 - 1)
            }
        })
        .sum()
}

#[aoc(day4, part2, jorendorff)]
fn part_2(cards: &[Card]) -> u64 {
    let mut counts = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        for j in 0..(card.hits() as usize) {
            counts[i + 1 + j] += counts[i];
        }
    }
    counts.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 30);
    }
}
