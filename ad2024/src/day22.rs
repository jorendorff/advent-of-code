use std::collections::{HashSet, HashMap};

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<u32>;

#[aoc_generator(day22, part1, jorendorff)]
#[aoc_generator(day22, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(u32));
    Ok(p.parse(text)?)
}

fn step(mut secret: u32) -> u32 {
    secret ^= secret << 6;
    secret &= 0xffffff;
    secret ^= secret >> 5;
    secret &= 0xffffff;
    secret ^= secret << 11;
    secret &= 0xffffff;

    secret
}

#[aoc(day22, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    input.iter().copied()
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = step(secret);
            }
            secret as u64
        })
        .sum::<u64>()
}

#[aoc(day22, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    let mut records = HashMap::<[i8; 4], u64>::new();

    for &v in input {
        let mut secret = v;
        let mut seen = HashSet::<[i8; 4]>::new();

        let mut arr = [0i8, 0, 0, 0];
        let mut prev_price = 0i8;
        for round in 0..2000 {
            secret = step(secret);
            let price = (secret % 10) as i8;
            arr = [price - prev_price, arr[0], arr[1], arr[2]];
            if round >= 4 {
                if !seen.contains(&arr) {
                    seen.insert(arr);
                    *records.entry(arr).or_default() += price as u64;
                }
            }
            prev_price = price;
        }
    }

    records.values().copied().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1
10
100
2024
";

    #[test]
    fn test_part_1() {
        assert_eq!(step(123), 15887950);
        assert_eq!(step(15887950), 16495136);
        assert_eq!(step(16495136), 527345);
        assert_eq!(step(527345), 704524);
        assert_eq!(step(704524), 1553684);
        assert_eq!(step(1553684), 12683156);
        assert_eq!(step(12683156), 11100544);
        assert_eq!(step(11100544), 12249484);
        assert_eq!(step(12249484), 7753432);
        assert_eq!(step(7753432), 5908254);

        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 37327623);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2(&parse_input(EXAMPLE).unwrap()) >= 23);
    }
}
