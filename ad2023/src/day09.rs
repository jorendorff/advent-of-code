use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<i64>>;

#[aoc_generator(day9, part1, jorendorff)]
#[aoc_generator(day9, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(repeat_sep(i64, " ")));
    Ok(p.parse(text)?)
}

fn predict(nums: &[i64]) -> i64 {
    let mut nums = nums.to_vec();
    let mut total = 0;
    while !nums.iter().copied().all(|x| x == 0) {
        total += *nums.last().unwrap();
        for i in 0..(nums.len() - 1) {
            nums[i] = nums[i + 1] - nums[i];
        }
        nums.pop();
    }
    total
}


#[aoc(day9, part1, jorendorff)]
fn part_1(input: &Input) -> i64 {
    // 245 on the global leaderboard
    input.iter().map(|row| predict(row)).sum()
}

#[aoc(day9, part2, jorendorff)]
fn part_2(input: &Input) -> i64 {
    // 206 on the global leaderboard
    input.iter().map(|row| {
        let mut row = row.to_vec();
        row.reverse();
        predict(&row)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 2);
    }
}
