use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<usize>>;

#[aoc_generator(day10, part1, jorendorff)]
#[aoc_generator(day10, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines({
        digit,
        '.' => usize::MAX
    }+));
    Ok(p.parse(text)?)
}

#[allow(clippy::needless_range_loop)]
#[aoc(day10, part1, jorendorff)]
fn part_1(input: &Input) -> u32 {
    let nr = input.len();
    let nc = input[0].len();

    let mut todo = vec![HashMap::<(usize, usize), u128>::new(); 10];
    let mut trail_id = 0;
    for (r, row) in input.iter().enumerate() {
        for (c, &a) in row.iter().enumerate() {
            if a == 0 {
                *todo[0].entry((r, c)).or_default() |= 1 << trail_id;
                trail_id += 1;
            }
        }
    }

    let mut score = 0;
    for a in 0..=9 {
        for ((r, c), trailheads) in std::mem::take(&mut todo[a]) {
            if a == 9 {
                score += trailheads.count_ones();
            } else if a < 9 {
                if r > 0 {
                    let aa = input[r - 1][c];
                    if a + 1 == aa {
                        *todo[aa].entry((r - 1, c)).or_default() |= trailheads;
                    }
                }
                if r + 1 < nr {
                    let aa = input[r + 1][c];
                    if a + 1 == aa {
                        *todo[aa].entry((r + 1, c)).or_default() |= trailheads;
                    }
                }
                if c > 0 {
                    let aa = input[r][c - 1];
                    if a + 1 == aa {
                        *todo[aa].entry((r, c - 1)).or_default() |= trailheads;
                    }
                }
                if c + 1 < nc {
                    let aa = input[r][c + 1];
                    if a + 1 == aa {
                        *todo[aa].entry((r, c + 1)).or_default() |= trailheads;
                    }
                }
            }
        }
    }
    score
}

#[aoc(day10, part2, jorendorff)]
fn part_2(input: &Input) -> u64 {
    let nr = input.len();
    let nc = input[0].len();

    let mut todo = vec![HashMap::<(usize, usize), u64>::new(); 10];
    for (r, row) in input.iter().enumerate() {
        for (c, &a) in row.iter().enumerate() {
            if a == 0 {
                *todo[0].entry((r, c)).or_default() = 1;
            }
        }
    }

    let mut score = 0;
    for a in 0..=9 {
        for ((r, c), num_trails) in std::mem::take(&mut todo[a]) {
            if a == 9 {
                score += num_trails;
            } else if a < 9 {
                if r > 0 {
                    let aa = input[r - 1][c];
                    if a + 1 == aa {
                        *todo[aa].entry((r - 1, c)).or_default() += num_trails;
                    }
                }
                if r + 1 < nr {
                    let aa = input[r + 1][c];
                    if a + 1 == aa {
                        *todo[aa].entry((r + 1, c)).or_default() += num_trails;
                    }
                }
                if c > 0 {
                    let aa = input[r][c - 1];
                    if a + 1 == aa {
                        *todo[aa].entry((r, c - 1)).or_default() += num_trails;
                    }
                }
                if c + 1 < nc {
                    let aa = input[r][c + 1];
                    if a + 1 == aa {
                        *todo[aa].entry((r, c + 1)).or_default() += num_trails;
                    }
                }
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
";

    const EXAMPLE2: &str = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";

    const EXAMPLE3: &str = "\
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
";

    const EXAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE1).unwrap()), 2);
        assert_eq!(part_1(&parse_input(EXAMPLE2).unwrap()), 4);
        assert_eq!(part_1(&parse_input(EXAMPLE3).unwrap()), 3);
        assert_eq!(part_1(&parse_input(EXAMPLE).unwrap()), 36);
    }

    const EXAMPLE5: &str = "\
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
";

    const EXAMPLE6: &str = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";

    const EXAMPLE7: &str = "\
012345
123456
234567
345678
4.6789
56789.
";

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE5).unwrap()), 3);
        assert_eq!(part_2(&parse_input(EXAMPLE6).unwrap()), 13);
        assert_eq!(part_2(&parse_input(EXAMPLE7).unwrap()), 227);
        assert_eq!(part_2(&parse_input(EXAMPLE).unwrap()), 81);
    }
}
