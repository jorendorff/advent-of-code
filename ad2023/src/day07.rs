use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<(Vec<i32>, u64)>;

#[aoc_generator(day07, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => -1,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
        }+ ' ' u64
    ));
    Ok(p.parse(text)?)
}

fn hand_key(hand: &(Vec<i32>, u64)) -> (Vec<usize>, Vec<i32>) {
    let (nums, _bid) = hand;
    let mut hist = std::collections::HashMap::<i32, usize>::new();
    for card in nums {
        *hist.entry(*card).or_insert(0) += 1;
    }
    let mut freq: Vec<usize> = hist.values().copied().collect();
    freq.sort_unstable();
    freq.reverse();
    (freq, nums.to_vec())
}

#[aoc(day07, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    // 416 on the global leaderboard
    let mut input = input.clone();
    input.sort_by_key(hand_key);
    input
        .into_iter()
        .enumerate()
        .map(|(index, (_cards, bid))| bid * (index as u64 + 1))
        .sum()
}

fn hand_key_2(hand: &(Vec<i32>, u64)) -> (Vec<usize>, Vec<i32>) {
    let (nums, _bid) = hand;
    let mut hist = std::collections::HashMap::<i32, usize>::new();
    let mut jacks = 0usize;
    for card in nums {
        if *card == -1 {
            jacks += 1;
        } else {
            *hist.entry(*card).or_insert(0) += 1;
        }
    }
    let mut freq: Vec<usize> = hist.values().copied().collect();
    freq.sort_unstable();
    freq.reverse();
    match freq.first_mut() {
        Some(item) => *item += jacks,
        None => freq.push(jacks),
    }
    (freq, nums.to_vec())
}

#[aoc(day07, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    // 459 on the global leaderboard
    let mut input = input.clone();
    input.sort_by_key(hand_key_2);
    input
        .into_iter()
        .enumerate()
        .map(|(index, (_cards, bid))| bid * (index as u64 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 6440);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 5905);
    }
}
