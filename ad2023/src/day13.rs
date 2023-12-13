use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<Vec<usize>>>;

#[aoc_generator(day13, part1, jorendorff)]
#[aoc_generator(day13, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(sections(
        lines(
            char_of(".#")+
        )
    ));
    Ok(p.parse(text)?)
}

// if clippy had its way this would be way uglier
#[allow(clippy::collapsible_if)]
fn solve(grid: &[Vec<usize>], ignore: Option<usize>) -> Option<usize> {
    let h = grid.len();
    for i in 1..h {
        if Some(100 * i) != ignore {
            if (0..(i.min(h - i))).all(|r| grid[i - 1 - r] == grid[i + r]) {
                return Some(100 * i);
            }
        }
    }
    let w = grid[0].len();
    for i in 1..w {
        if Some(i) != ignore {
            if (0..(i.min(w - i))).all(|c| (0..h).all(|r| grid[r][i - 1 - c] == grid[r][i + c])) {
                return Some(i);
            }
        }
    }
    None
}

#[aoc(day13, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    // 81 on the global leaderboard
    input.iter().map(|grid| solve(grid, None).unwrap()).sum()
}

#[aoc(day13, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    // 101 on the global leaderboard
    // out of the money by a second
    input
        .iter()
        .map(|grid| {
            let too_easy = solve(grid, None).unwrap();

            let mut grid = grid.clone();

            let h = grid.len();
            let w = grid[0].len();
            for r in 0..h {
                for c in 0..w {
                    grid[r][c] = 1 - grid[r][c];
                    if let Some(v) = solve(&grid, Some(too_easy)) {
                        return v;
                    }
                    grid[r][c] = 1 - grid[r][c];
                }
            }

            panic!("no solution");
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 405);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 400);
    }
}
