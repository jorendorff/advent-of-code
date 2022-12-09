use std::collections::HashSet;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<((i32, i32), usize)>;

#[aoc_generator(day9, part1, jorendorff)]
#[aoc_generator(day9, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(
        {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
        }
        " " usize
    ));
    aoc_parse(text, p)
}

#[allow(clippy::comparison_chain)]
fn move_toward(head: (i32, i32), tail: &mut (i32, i32)) {
    if head.0.abs_diff(tail.0).max(head.1.abs_diff(tail.1)) < 2 {
        // head and tail are touching, no move
        return;
    }

    if tail.0 < head.0 {
        tail.0 += 1;
    } else if tail.0 > head.0 {
        tail.0 -= 1;
    }

    if tail.1 < head.1 {
        tail.1 += 1;
    } else if tail.1 > head.1 {
        tail.1 -= 1;
    }
}

#[aoc(day9, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let mut visited = HashSet::new();
    let mut h = (0, 0);
    let mut t = (0, 0);
    visited.insert(t);
    for &((dx, dy), reps) in input {
        for _ in 0..reps {
            h.0 += dx;
            h.1 += dy;
            move_toward(h, &mut t);
            visited.insert(t);
        }
    }
    visited.len()
}

#[aoc(day9, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let mut visited = HashSet::new();
    const N: usize = 10;
    let mut knots = vec![(0, 0); N];
    visited.insert(knots[N - 1]);
    for &((dx, dy), reps) in input {
        for _ in 0..reps {
            knots[0].0 += dx;
            knots[0].1 += dy;
            for i in 1..N {
                move_toward(knots[i - 1], &mut knots[i]);
            }
            visited.insert(knots[N - 1]);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const EXAMPLE2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 1);
        assert_eq!(part_2(&parse_input(EXAMPLE2).unwrap()), 36);
    }
}
