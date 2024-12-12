// Part 2 rank 627.

use std::collections::VecDeque;

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::*;

type Input = Vec<Vec<char>>;

#[aoc_generator(day12, part1, jorendorff)]
#[aoc_generator(day12, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Input> {
    let p = parser!(lines(alpha+));
    Ok(p.parse(text)?)
}

fn get(input: &Input, r: isize, c: isize) -> char {
    if r < 0 || c < 0 || r >= input.len() as isize || c >= input[0].len() as isize {
        '#'
    } else {
        input[r as usize][c as usize]
    }
}

#[aoc(day12, part1, jorendorff)]
fn part_1(input: &Input) -> usize {
    let mut map = input.clone();
    let nr = map.len();
    let nc = map[0].len();

    let mut total = 0;
    for r in 0..nr {
        for c in 0..nc {
            let ch = map[r][c];
            if ch.is_ascii_uppercase() {
                map[r][c] = ch.to_ascii_lowercase();
                let mut perimeter = 0;
                let mut area = 0;
                let mut todo = [(r as isize, c as isize)]
                    .into_iter()
                    .collect::<VecDeque<(isize, isize)>>();
                while let Some((r, c)) = todo.pop_front() {
                    area += 1;
                    for (dr, dc) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                        match get(&map, r + dr, c + dc) {
                            x if x == ch.to_ascii_lowercase() => {}
                            x if x == ch => {
                                map[(r + dr) as usize][(c + dc) as usize] = ch.to_ascii_lowercase();
                                todo.push_back((r + dr, c + dc));
                            }
                            _ => {
                                perimeter += 1;
                            }
                        }
                    }
                }
                total += perimeter * area;
            }
        }
    }
    total
}

fn count(segments: Vec<(isize, isize)>) -> usize {
    1 + segments
        .windows(2)
        .filter(|pair| {
            let (r1, c1) = pair[0];
            pair[1] != (r1, c1 + 1)
        })
        .count()
}

#[aoc(day12, part2, jorendorff)]
fn part_2(input: &Input) -> usize {
    let mut map = input.clone();
    let nr = map.len();
    let nc = map[0].len();

    let mut total = 0;
    for r in 0..nr {
        for c in 0..nc {
            let ch = map[r][c];
            if ch.is_ascii_uppercase() {
                map[r][c] = ch.to_ascii_lowercase();
                let mut area = 0;
                let mut todo = [(r as isize, c as isize)]
                    .into_iter()
                    .collect::<VecDeque<(isize, isize)>>();
                let mut fences: Vec<Vec<(isize, isize)>> = vec![vec![]; 4];
                while let Some((r, c)) = todo.pop_front() {
                    area += 1;
                    for (dir, (dr, dc)) in
                        [(-1, 0), (1, 0), (0, 1), (0, -1)].into_iter().enumerate()
                    {
                        match get(&map, r + dr, c + dc) {
                            x if x == ch.to_ascii_lowercase() => {}
                            x if x == ch => {
                                map[(r + dr) as usize][(c + dc) as usize] = ch.to_ascii_lowercase();
                                todo.push_back((r + dr, c + dc));
                            }
                            _ => {
                                if dc == 0 {
                                    fences[dir].push((r.min(r + dr), c));
                                } else if dr == 0 {
                                    fences[dir].push((c.min(c + dc), r));
                                }
                            }
                        }
                    }
                }
                let num_sides = fences
                    .into_iter()
                    .map(|mut segs| {
                        segs.sort();
                        count(segs)
                    })
                    .sum::<usize>();
                total += num_sides * area;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
AAAA
BBCD
BBCC
EEEC
";

    const EXAMPLE2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    const EXAMPLE3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    const EXAMPLE4: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    const EXAMPLE5: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_input(EXAMPLE1).unwrap()), 140);
        assert_eq!(part_1(&parse_input(EXAMPLE2).unwrap()), 772);
        assert_eq!(part_1(&parse_input(EXAMPLE3).unwrap()), 1930);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input(EXAMPLE1).unwrap()), 80);
        assert_eq!(part_2(&parse_input(EXAMPLE2).unwrap()), 436);
        assert_eq!(part_2(&parse_input(EXAMPLE4).unwrap()), 236);
        assert_eq!(part_2(&parse_input(EXAMPLE5).unwrap()), 368);
    }
}
