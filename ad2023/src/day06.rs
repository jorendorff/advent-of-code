use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = (Vec<u64>, Vec<u64>);

#[aoc_generator(day6, part1, jorendorff)]
fn parse_input_1(text: &str) -> anyhow::Result<Input> {
    let p = parser!(
        t:line("Time:" ' '* nums:repeat_sep(u64, ' '+) => nums)
        d:line("Distance:" ' '* nums:repeat_sep(u64, ' '+) => nums)
        => (t, d)
    );
    Ok(p.parse(text)?)
}

#[aoc(day6, part1, jorendorff)]
fn part_1(input: &Input) -> u64 {
    (input.0.iter().zip(input.1.iter()))
        .map(|(t, d)| {
            let t = *t;
            let d = *d;
            (0..=t).filter(|&t0| {
                (t - t0) * t0 > d
            }).count() as u64
        })
        .product()
}

#[aoc_generator(day6, part2, jorendorff)]
fn parse_input_2(text: &str) -> anyhow::Result<(u64, u64)> {
    let p = parser!(
        t:line("Time:" ' '* nums:repeat_sep(string(digit+), ' '+) => nums.join(""))
        d:line("Distance:" ' '* nums:repeat_sep(string(digit+), ' '+) => nums.join(""))
        => (t.parse::<u64>().unwrap(), d.parse::<u64>().unwrap())
    );

    Ok(p.parse(text)?)
}

#[aoc(day6, part2, jorendorff)]
fn part_2(input: &(u64, u64)) -> u64 {
    let (t, d) = *input;
    let center = t / 2;
    let radius = ((t * t - 4 * d) as f64).sqrt() / 2.0;
    let low = center - radius.floor() as u64;
    let hi = center + radius.floor() as u64;

    // I had low enough confidence in my ability to get the quadratic formula right under pressure
    // that I had the program check for me:
    let low_count = (low - 3 .. low + 3).filter(|&t0| {
        (t - t0) * t0 > d
    }).count() as u64;
    assert_ne!(low_count, 0);
    assert_ne!(low_count, 6);

    let hi_count = (hi - 3 .. hi + 3).filter(|&t0| {
        (t - t0) * t0 > d
    }).count() as u64;
    assert_ne!(hi_count, 0);
    assert_ne!(hi_count, 6);

    low_count + ((hi - 3) - (low + 3)) + hi_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input_1(EXAMPLE).unwrap()), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input_2(EXAMPLE).unwrap()), 71503);
    }
}
