use aoc_runner_derive::*;

use std::ops::RangeInclusive;

type Coord = i64;

type Input = (RangeInclusive<Coord>, RangeInclusive<Coord>);

fn parse_range(vals: &str) -> RangeInclusive<Coord> {
    let (start, end) = vals.split_once("..").unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}

#[aoc_generator(day17, part1, jorendorff)]
#[aoc_generator(day17, part2, jorendorff)]
fn parse_input(text: &str) -> Input {
    let (tag, data) = text.trim().split_once(": ").unwrap();
    assert_eq!(tag, "target area");
    let (x, y) = data.split_once(", ").unwrap();
    let (varx, xvals) = x.split_once('=').unwrap();
    assert_eq!(varx, "x");
    let (vary, yvals) = y.split_once('=').unwrap();
    assert_eq!(vary, "y");
    (parse_range(xvals), parse_range(yvals))
}

#[aoc(day17, part1, jorendorff)]
fn part_1(input: &Input) -> i64 {
    let vy = input.1.start().abs() - 1;
    (0..=vy).sum()
}

#[aoc(day17, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let x_stop = input.0.end() + 1;
    let y_stop = input.1.start() - 1;
    let vy_start = *input.1.start();
    let vy_stop = input.1.start().abs();
    let vx_start = 0;
    let vx_stop = input.0.end() + 1;

    (vx_start..vx_stop)
        .flat_map(move |vx| {
            (vy_start..vy_stop).filter_map(move |vy| {
                let (mut x, mut y) = (0, 0);
                let mut vx = vx;
                let mut vy = vy;
                while x < x_stop && y > y_stop {
                    x += vx;
                    y += vy;
                    if input.0.contains(&x) && input.1.contains(&y) {
                        return Some((vx, vy));
                    }
                    if vx > 0 {
                        vx -= 1;
                    }
                    vy -= 1;
                }
                None
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
target area: x=20..30, y=-10..-5
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE)), 45);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE)), 112);
    }
}
