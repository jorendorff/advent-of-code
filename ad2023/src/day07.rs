use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<(Vec<i32>, u64)>;

#[aoc_generator(day07, part1, jorendorff)]
#[aoc_generator(day07, part2, jorendorff)]
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

fn hand_key(hand: &(Vec<i32>, u64)) -> (i32, Vec<i32>) {
    let (nums, _bid) = hand;
    let mut hist = std::collections::HashMap::<i32, usize>::new();
    for card in nums {
        *hist.entry(*card).or_insert(0) += 1;
    }
    let mut freq: Vec<usize> = hist.values().copied().collect();
    freq.sort_unstable();
    let ty = match *freq {
        [5] => 6,
        [1, 4] => 5,
        [2, 3] => 4,
        [1, 1, 3] => 3,
        [1, 2, 2] => 2,
        [1, 1, 1, 2] => 1,
        _ => 0,
    };
    (ty, nums.to_vec())
}

#[aoc(day07, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    // 416
    let mut input = input.clone();
    input.sort_by_key(hand_key);
    for h in &input {
        eprintln!("{h:?}, {:?}", hand_key(h).0);
    }
    input.into_iter().enumerate().map(|(index, (_cards, bid))| bid * (index as u64 + 1)).sum()
}

fn all_possibles(h: &[i32]) -> Vec<Vec<i32>> {
    let mut all = vec![vec![]];

    for card in h {
        let card = *card;
        if card == -1 {
            all = all.into_iter().flat_map(|seq| {
                (2..=14).map(move |c| {
                    let mut new_seq = seq.clone();
                    new_seq.push(c);
                    new_seq
                })
            }).collect();
        } else {
            for c in &mut all {
                c.push(card);
            }
        }
    }

    all
}

fn hand_key_2(hand: &(Vec<i32>, u64)) -> (i32, Vec<i32>) {
    let (nums, _bid) = hand;
    let ty = all_possibles(nums).into_iter().map(|nums| {
        let mut hist = std::collections::HashMap::<i32, usize>::new();
        for card in nums {
            *hist.entry(card).or_insert(0) += 1;
        }
        let mut freq: Vec<usize> = hist.values().copied().collect();
        freq.sort_unstable();
        match *freq {
            [5] => 6,
            [1, 4] => 5,
            [2, 3] => 4,
            [1, 1, 3] => 3,
            [1, 2, 2] => 2,
            [1, 1, 1, 2] => 1,
            _ => 0,
        }
    })
    .max().unwrap();
    (ty, nums.to_vec())
}

#[aoc(day07, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    // 459
    let mut input = input.clone();
    input.sort_by_key(hand_key_2);
    input.into_iter().enumerate().map(|(index, (_cards, bid))| bid * (index as u64 + 1)).sum()
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
