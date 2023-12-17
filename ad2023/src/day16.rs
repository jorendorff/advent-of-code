use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<char>>;

#[aoc_generator(day16, part1, jorendorff)]
#[aoc_generator(day16, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        any_char+
    ));
    Ok(p.parse(text)?)
}

const RIGHT: usize = 0;
const UP: usize = 1;
const LEFT: usize = 2;
const DOWN: usize = 3;

fn dr(dir: usize) -> usize {
    [0, usize::MAX, 0, 1][dir]
}

fn dc(dir: usize) -> usize {
    [1, 0, usize::MAX, 0][dir]
}

fn solve(input: &Input, r0: usize, c0: usize, dir0: usize) -> usize {
    let h = input.len();
    let w = input[0].len();

    let mut lit = vec![vec![[false; 4]; w]; h];
    let mut todo: Vec<(usize, usize, usize)> = vec![(r0, c0, dir0)];

    lit[r0][c0][dir0] = true;

    fn enlist(lit: &mut Vec<Vec<[bool; 4]>>, todo: &mut Vec<(usize, usize, usize)>, r: usize, c: usize, dir: usize) {
        if r < lit.len() && c < lit[r].len() && !lit[r][c][dir] {
            lit[r][c][dir] = true;
            todo.push((r, c, dir));
        }
    }

    while let Some((r, c, dir)) = todo.pop() {
        match input[r][c] {
            '.' => {
                let r1 = r.wrapping_add(dr(dir));
                let c1 = c.wrapping_add(dc(dir));
                enlist(&mut lit, &mut todo, r1, c1, dir);
            }
            '/' => {
                let dir1 = match dir {
                    RIGHT => UP,
                    UP => RIGHT,
                    LEFT => DOWN,
                    DOWN => LEFT,
                    _ => panic!(),
                };
                enlist(&mut lit, &mut todo, r.wrapping_add(dr(dir1)), c.wrapping_add(dc(dir1)), dir1);
            }
            '\\' => {
                let dir1 = match dir {
                    RIGHT => DOWN,
                    UP => LEFT,
                    LEFT => UP,
                    DOWN => RIGHT,
                    _ => panic!(),
                };
                enlist(&mut lit, &mut todo, r.wrapping_add(dr(dir1)), c.wrapping_add(dc(dir1)), dir1);
            }
            '|' if dir == UP || dir == DOWN => {
                let r1 = r.wrapping_add(dr(dir));
                let c1 = c.wrapping_add(dc(dir));
                enlist(&mut lit, &mut todo, r1, c1, dir);
            }
            '-' if dir == LEFT || dir == RIGHT => {
                let r1 = r.wrapping_add(dr(dir));
                let c1 = c.wrapping_add(dc(dir));
                enlist(&mut lit, &mut todo, r1, c1, dir);
            }
            '|' => {
                enlist(&mut lit, &mut todo, r, c, UP);
                enlist(&mut lit, &mut todo, r, c, DOWN);
            }
            '-' => {
                enlist(&mut lit, &mut todo, r, c, LEFT);
                enlist(&mut lit, &mut todo, r, c, RIGHT);
            }
            _ => panic!(),
        }
    }

    for (lit_row, input_row) in lit.iter().zip(input) {
        for (lit_arr, input_ch) in lit_row.iter().zip(input_row.iter().copied()) {
            if lit_arr.iter().any(|q| *q) {
                print!("\x1b[31;103m{input_ch}\x1b[39;49m");
            } else {
                print!("{input_ch}");
            }
        }
        println!();
    }

    lit.into_iter().map(|row| {
        row.into_iter().map(|dirs| {
            if dirs.iter().any(|is_lit| *is_lit) {
                1_usize
            } else {
                0
            }
        }).sum::<usize>()
    }).sum()
}

#[aoc(day16, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    // #576 on the global leaderboard
    solve(input, 0, 0, 0)
}

#[aoc(day16, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    // #429 on the global leaderboard
    let h = input.len();
    let w = input[0].len();
    (0..w).map(|c| (0, c, DOWN))
        .chain((0..h).map(|r| (r, 0, RIGHT)))
        .chain((0..w).map(|c| (h - 1, c, UP)))
        .chain((0..h).map(|r| (r, w - 1, LEFT)))
        .map(|(r0, c0, dir0)| solve(input, r0, c0, dir0))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 46);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 51);
    }
}
