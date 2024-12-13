use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Game = ((i64, i64), (i64, i64), (i64, i64));

type Input = Vec<Game>;

#[aoc_generator(day13, part1, jorendorff)]
#[aoc_generator(day13, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(sections(
        line("Button A: X+" i64 ", Y+" i64)
        line("Button B: X+" i64 ", Y+" i64)
        line("Prize: X=" i64 ", Y=" i64)
    ));
    Ok(p.parse(text)?)
}

fn min_cost(game: Game) -> i64 {
    let ((ax, ay), (bx, by), (x, y)) = game;
    (0..=100)
        .flat_map(|na: i64| (0..=100).filter_map(move |nb: i64| {
            if na * ax + nb * bx == x && na * ay + nb * by == y {
                Some(3 * na + nb)
            } else {
                None
            }
        }))
        .min()
        .unwrap_or(0)
}

#[aoc(day13, part1, jorendorff)]
fn part_1(input: &Input) -> i64 {
    input
        .iter()
        .map(|game| min_cost(*game))
        .sum()
}

fn min_cost_2(game: Game) -> Option<i64> {
    let ((ax, ay), (bx, by), (x, y)) = game;
    let x = 10000000000000 + x;
    let y = 10000000000000 + y;

    assert_ne!((ax, ay), (0, 0));
    assert_ne!((bx, by), (0, 0));

    if ax * by == bx * ay {
        todo!("figure out 1D version of problem :D");
    }

    // if there is a solution, it is unique

    // what linear transform maps vector a to the unit y vector, and vector b to the unit x vector?
    // first skew everything in y direction by x times -by/bx.
    //     y -= x * by/bx;
    //     ay -= ax * by/bx;
    //     by = 0;  // by - bx * by/bx
    //
    // Now b lies on the x axis.
    //
    // Then skew everything in x direction by y times -ax/ay.
    //     x -= y * ax/ay;
    //     bx -= by * ax/ay;
    //     ax = 0;  // ax - ay * ax/ay
    //
    // Now a lies on the y axis.
    //
    // Now squish in the x direction by bx
    //     x = x / bx;
    //     bx = 1;
    //
    // And squish in the y direction by ay:
    //     y = y / ay;
    //     ay = 1;
    //
    // And now na = y and nb = x.
    // Putting all that together, we get
    //     let ay_bx = ay*bx - ax*by;
    //     let na = (y*bx - x * by) / (ay_bx);
    // Then nb follows.

    let y_bx = y * bx - x * by;
    let ay_bx = ay * bx - ax * by;

    if y_bx % ay_bx != 0 {
        return None;
    }
    let na = y_bx / ay_bx;

    if na * ax > x {
        return None;
    }
    let n = x - na * ax;
    if n % bx != 0 {
        return None;
    }
    let nb = n / bx;

    assert_eq!(na * ax + nb * bx, x);
    assert_eq!(na * ay + nb * by, y);

    Some(3 * na + nb)
}

#[aoc(day13, part2, jorendorff)]
fn part_2(input: &Input) -> i64 {
    input
        .iter()
        .filter_map(|game| min_cost_2(*game))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 480);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 875318608908);
    }

}
