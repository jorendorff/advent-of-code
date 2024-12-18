// Part 2 rank 982

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<(usize, usize)>;

#[aoc_generator(day18, part1, jorendorff)]
#[aoc_generator(day18, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(usize "," usize));
    Ok(p.parse(text)?)
}

fn do_part_1(input: &Input, size: usize, count: usize) -> Option<usize> {
    let mut map = vec![vec![0u8; size]; size];
    for &(r, c) in &input[..count] {
        map[r][c] = 1;
    }

    map[0][0] = 2;
    let mut todo = vec![(0, 0)];
    let mut steps = 0;
    while !todo.is_empty() {
        let mut next = vec![];
        for (r, c) in todo {
            if (r, c) == (size - 1, size - 1) {
                return Some(steps);
            }
            let points = [
                (r, c + 1),
                (r + 1, c),
                (r, c.wrapping_sub(1)),
                (r.wrapping_sub(1), c),
            ];
            for (rr, cc) in points {
                if rr < size && cc < size && map[rr][cc] == 0 {
                    next.push((rr, cc));
                    map[rr][cc] = 2;
                }
            }
        }

        todo = next;
        steps += 1;
    }
    None
}

#[aoc(day18, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    do_part_1(input, 71, 1024).expect("no solution")
}

#[aoc(day18, part2, jorendorff)]
fn part_2(input: &Input) -> String {
    for i in 1024..input.len() {
        if do_part_1(input, 71, i + 1).is_none() {
            return format!("{:?}", input[i]);
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test_part_1() {
        assert_eq!(do_part_1(&parse_input(EXAMPLE).unwrap(), 7, 12), Some(22));
    }
}
