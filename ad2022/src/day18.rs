use std::collections::{HashSet, VecDeque};

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<(i64, i64, i64)>;

#[aoc_generator(day18, part1, jorendorff)]
#[aoc_generator(day18, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(i64 ',' i64 ',' i64));
    Ok(p.parse(text)?)
}

#[aoc(day18, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    // Rank 347 on the day's leaderboard.
    let s = input.iter().copied().collect::<HashSet<(i64, i64, i64)>>();
    input
        .iter()
        .copied()
        .map(|(x, y, z)| {
            [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ]
            .into_iter()
            .filter(|p| !s.contains(&p))
            .count()
        })
        .sum()
}

#[aoc(day18, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    // Rank 292 on the day's leaderboard.
    let x_min = input.iter().copied().map(|(x, _, _)| x).min().unwrap() - 1;
    let x_max = input.iter().copied().map(|(x, _, _)| x).max().unwrap() + 1;
    let y_min = input.iter().copied().map(|(_, y, _)| y).min().unwrap() - 1;
    let y_max = input.iter().copied().map(|(_, y, _)| y).max().unwrap() + 1;
    let z_min = input.iter().copied().map(|(_, _, z)| z).min().unwrap() - 1;
    let z_max = input.iter().copied().map(|(_, _, z)| z).max().unwrap() + 1;

    let s = input.iter().copied().collect::<HashSet<(i64, i64, i64)>>();
    let mut exterior: HashSet<(i64, i64, i64)> = HashSet::new();

    let mut todo = VecDeque::new();
    todo.push_back((x_min, y_min, z_min));
    while let Some(p) = todo.pop_front() {
        let (x, y, z) = p;
        for q in [
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ] {
            let (x, y, z) = q;
            if x_min <= x
                && x <= x_max
                && y_min <= y
                && y <= y_max
                && z_min <= z
                && z <= z_max
                && !exterior.contains(&q)
                && !s.contains(&q)
            {
                exterior.insert(q);
                todo.push_back(q);
            }
        }
    }

    input
        .iter()
        .copied()
        .map(|(x, y, z)| {
            [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ]
            .into_iter()
            .filter(|p| exterior.contains(p))
            .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 64);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 58);
    }
}
