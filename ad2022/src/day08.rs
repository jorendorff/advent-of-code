use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<i32>>;

#[aoc_generator(day8, part1, jorendorff)]
#[aoc_generator(day8, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines((a:digit => a as i32)+));
    Ok(p.parse(text)?)
}

fn mark_visible<Iter>(input: &[Vec<i32>], visible: &mut [Vec<usize>], iter: Iter)
where
    Iter: IntoIterator<Item = (usize, usize)>,
{
    let mut tallest = -1;
    for (r, c) in iter {
        if input[r][c] > tallest {
            visible[r][c] = 1;
            tallest = input[r][c];
        }
    }
}

#[aoc(day8, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let ncols = input[0].len();
    let nrows = input.len();

    let mut visible: Vec<Vec<usize>> = vec![vec![0; ncols]; nrows];

    for r in 0..nrows {
        mark_visible(input, &mut visible, (0..ncols).map(|c| (r, c)));
        mark_visible(input, &mut visible, (0..ncols).rev().map(|c| (r, c)));
    }

    for c in 0..ncols {
        mark_visible(input, &mut visible, (0..nrows).map(|r| (r, c)));
        mark_visible(input, &mut visible, (0..nrows).rev().map(|r| (r, c)));
    }

    visible.into_iter().flatten().sum()
}

fn view_distance<Iter>(grid: &Input, vantage_height: i32, sight_line: Iter) -> usize
where
    Iter: IntoIterator<Item = (usize, usize)>,
{
    let mut d = 0;
    for (r, c) in sight_line {
        d += 1;
        if grid[r][c] >= vantage_height {
            break;
        }
    }
    d
}

#[aoc(day8, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let width = input[0].len();
    let height = input.len();
    let cols = 1..width - 1;
    let rows = 1..height - 1;

    rows.flat_map(|r| {
        cols.clone().map(move |c| {
            let h = input[r][c];

            let n = view_distance(input, h, (0..r).rev().map(|rr| (rr, c)));
            let s = view_distance(input, h, (r + 1..height).map(|rr| (rr, c)));
            let w = view_distance(input, h, (0..c).rev().map(|cc| (r, cc)));
            let e = view_distance(input, h, (c + 1..width).map(|cc| (r, cc)));
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
