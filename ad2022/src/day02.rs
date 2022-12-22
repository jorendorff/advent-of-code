use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

use Move::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Goal {
    Win,
    Lose,
    Draw,
}

use Goal::*;

#[aoc_generator(day2, part1, jorendorff)]
fn parse_input_1(text: &str) -> anyhow::Result<Vec<(Move, Move)>> {
    let p = parser!(lines(
        {"A" => Rock, "B" => Paper, "C" => Scissors}
        " "
        {"X" => Rock, "Y" => Paper, "Z" => Scissors}
    ));
    Ok(p.parse(text)?)
}

fn score((m1, m2): (Move, Move)) -> u64 {
    (match m2 {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }) + match (m1, m2) {
        _ if m1 == m2 => 3,
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
        _ => 0,
    }
}

#[aoc(day2, part1, jorendorff)]
fn part_1(lists: &[(Move, Move)]) -> u64 {
    lists.iter().copied().map(score).sum()
}

#[aoc_generator(day2, part2, jorendorff)]
fn parse_input_2(text: &str) -> anyhow::Result<Vec<(Move, Goal)>> {
    let p = parser!(lines(
        {"A" => Rock, "B" => Paper, "C" => Scissors}
        " "
        {"X" => Lose, "Y" => Draw, "Z" => Win}
    ));
    Ok(p.parse(text)?)
}

fn advice(opp: Move, goal: Goal) -> Move {
    match (opp, goal) {
        (opp, Draw) => opp,
        (Rock, Win) => Paper,
        (Paper, Win) => Scissors,
        (Scissors, Win) => Rock,
        (Rock, Lose) => Scissors,
        (Paper, Lose) => Rock,
        (Scissors, Lose) => Paper,
    }
}

#[aoc(day2, part2, jorendorff)]
fn part_2(guide: &[(Move, Goal)]) -> u64 {
    guide
        .iter()
        .copied()
        .map(|(m1, goal)| {
            let m2 = advice(m1, goal);
            score((m1, m2))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
A Y
B X
C Z
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input_1(EXAMPLE).unwrap()), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input_2(EXAMPLE).unwrap()), 12);
    }
}
