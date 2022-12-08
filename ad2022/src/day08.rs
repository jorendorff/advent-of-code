use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<i32>>;

#[aoc_generator(day8, part1, jorendorff)]
#[aoc_generator(day8, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(((a: digit) => a as i32)+));
    aoc_parse(text, p)
}

#[aoc(day8, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let w = input[0].len();
    let h = input.len();
    let cols = 0..w;
    let rows = 0..h;

    let mut visible: Vec<Vec<usize>> = vec![vec![0; w]; h];

    for r in rows.clone() {
        let mut tallest = -1;
        for c in cols.clone() {
            if input[r][c] > tallest {
                visible[r][c] = 1;
                tallest = input[r][c];
            }
        }

        tallest = -1;
        for c in cols.clone().rev() {
            if input[r][c] > tallest {
                visible[r][c] = 1;
                tallest = input[r][c];
            }
        }
    }

    for c in cols {
        let mut tallest = -1;
        for r in rows.clone() {
            if input[r][c] > tallest {
                visible[r][c] = 1;
                tallest = input[r][c];
            }
        }

        tallest = -1;
        for r in rows.clone().rev() {
            if input[r][c] > tallest {
                visible[r][c] = 1;
                tallest = input[r][c];
            }
        }
    }

    visible.into_iter().flatten().sum()
}

#[aoc(day8, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let width = input[0].len();
    let height = input.len();
    let cols = 1..width - 1;
    let rows = 1..height - 1;

    rows.flat_map(|r| {
        cols.clone().map(move |c| {
            println!("{r} {c}");

            let mut s = 0;
            for rr in r + 1..height {
                s += 1;
                if input[rr][c] >= input[r][c] {
                    break;
                }
            }
            assert!(s > 0);

            let mut n = 0;
            for rr in (0..r).rev() {
                n += 1;
                if input[rr][c] >= input[r][c] {
                    break;
                }
            }
            assert!(n > 0);

            let mut w = 0;
            for cc in (0..c).rev() {
                w += 1;
                if input[r][cc] >= input[r][c] {
                    break;
                }
            }
            assert!(w > 0);

            let mut e = 0;
            for cc in c + 1..width {
                e += 1;
                if input[r][cc] >= input[r][c] {
                    break;
                }
            }
            assert!(e > 0);

            n * s * e * w
        })
    })
    .max()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
30373
25512
65332
33549
35390
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 8);
    }
}
